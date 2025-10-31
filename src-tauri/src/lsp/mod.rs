use std::{net::SocketAddr, sync::{Arc, Mutex}, process::Stdio};
use tokio::{net::TcpListener, process::Command, io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, sync::Mutex as AsyncMutex};
use tauri::async_runtime::{spawn, JoinHandle};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use lazy_static::lazy_static;
use crate::settings;

// LSP server that bridges WebSocket connections to Intelephense stdio process
// Architecture: WebSocket (Monaco) <-> Bridge <-> stdio (Intelephense)

static PORT: u16 = 54331;

#[derive(Default)]
struct LspState {
    task: Option<JoinHandle<()>>,
}

lazy_static! {
    static ref STATE: Arc<Mutex<LspState>> = Arc::new(Mutex::new(LspState::default()));
}

/// Resolve Intelephense entry point using Node.js require.resolve
/// 
/// Note: This requires Node.js to be installed on the system.
/// In production builds, Intelephense must be available via npm (either globally installed
/// or accessible through the app's bundled node_modules if bundled).
/// 
/// For development: Requires `npm install intelephense` in the project directory.
/// For production: Node.js must be installed on the user's system with Intelephense available.
async fn resolve_intelephense_entry() -> Result<String, String> {
    let output = Command::new("node")
        .arg("-p")
        .arg("require.resolve('intelephense')")
        .output()
        .await
        .map_err(|e| format!("Failed to execute node: {}. Make sure Node.js is installed.", e))?;
    
    if !output.status.success() {
        return Err("Failed to resolve intelephense. Make sure intelephense is installed (npm install intelephense)".to_string());
    }
    
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return Err("Intelephense entry point is empty".to_string());
    }
    
    Ok(path)
}

/// Start the LSP WebSocket server that bridges to Intelephense
pub fn start_server_if_needed(app: tauri::AppHandle) -> Result<(), String> {
    let mut state = STATE.lock().map_err(|_| "Failed to lock LSP state".to_string())?;
    if state.task.is_some() {
        return Ok(());
    }

    let task = spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
        let listener = match TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[LSP] Failed to bind {}: {}", PORT, e);
                return;
            }
        };
        println!("[LSP] WebSocket server listening on ws://127.0.0.1:{}", PORT);

        loop {
            let (stream, _peer) = match listener.accept().await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[LSP] Accept error: {}", e);
                    break;
                }
            };

            let app_handle = app.clone();
            spawn(async move {
                let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("[LSP] WebSocket handshake error: {}", e);
                        return;
                    }
                };

                let (mut write, mut read) = ws_stream.split();

                // Resolve Intelephense entry point
                let intelephense_entry = match resolve_intelephense_entry().await {
                    Ok(path) => path,
                    Err(e) => {
                        eprintln!("[LSP] {}", e);
                        // Close connection if Intelephense is not available
                        let _ = write.send(Message::Close(None)).await;
                        return;
                    }
                };

                // Get settings for license key
                let license_key = match settings::get_settings(app_handle.clone()).await {
                    Ok(s) => s.intelephense_license_key.clone().unwrap_or_default(),
                    Err(_) => String::new(),
                };

                // Spawn Intelephense process with stdio
                let mut cmd = Command::new("node");
                cmd.arg(&intelephense_entry)
                    .arg("--stdio")
                    .env("ELECTRON_RUN_AS_NODE", "1")
                    .env("INTELEPHENSE_TELEMETRY_ENABLED", "0");
                
                if !license_key.is_empty() {
                    cmd.env("INTELEPHENSE_LICENSE_KEY", license_key);
                }
                
                let mut child = match cmd
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("[LSP] Failed to spawn Intelephense: {}", e);
                        let _ = write.send(Message::Close(None)).await;
                        return;
                    }
                };

                let mut child_stdin = child.stdin.take().expect("Failed to take stdin");
                let mut child_stdout = BufReader::new(child.stdout.take().expect("Failed to take stdout"));
                let child_stderr = child.stderr.take();

                // Forward stderr to console for debugging
                if let Some(mut stderr) = child_stderr {
                    spawn(async move {
                        let mut buf = vec![0u8; 1024];
                        loop {
                            match tokio::io::AsyncReadExt::read(&mut stderr, &mut buf).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    let text = String::from_utf8_lossy(&buf[..n]);
                                    eprintln!("[Intelephense stderr] {}", text.trim_end());
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }

                // Task: WebSocket -> Intelephense stdio
                let stdin_task = spawn(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(txt)) => {
                                // LSP protocol: send message with Content-Length header
                                let content = txt.into_bytes();
                                let header = format!("Content-Length: {}\r\n\r\n", content.len());
                                
                                if child_stdin.write_all(header.as_bytes()).await.is_err() {
                                    break;
                                }
                                if child_stdin.write_all(&content).await.is_err() {
                                    break;
                                }
                                if child_stdin.flush().await.is_err() {
                                    break;
                                }
                            }
                            Ok(Message::Binary(_)) => {
                                // Binary messages not supported in LSP protocol
                                eprintln!("[LSP] Binary message received (not supported)");
                            }
                            Ok(Message::Close(_)) => {
                                break;
                            }
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("[LSP] WebSocket read error: {}", e);
                                break;
                            }
                        }
                    }
                });

                // Task: Intelephense stdio -> WebSocket  
                let write_handle = Arc::new(AsyncMutex::new(write));
                let write_clone = write_handle.clone();
                let stdout_task = spawn(async move {
                    loop {
                        // Read headers until blank line (LSP protocol format)
                        let mut content_length: usize = 0;
                        loop {
                            let mut header_line = String::new();
                            match child_stdout.read_line(&mut header_line).await {
                                Ok(0) => return, // EOF
                                Err(_) => return,
                                Ok(_) => {}
                            }

                            let line = header_line.trim();
                            if line.is_empty() {
                                // Blank line indicates end of headers
                                break;
                            }

                            // Parse Content-Length header
                            if let Some(rest) = line.strip_prefix("Content-Length: ") {
                                content_length = rest.trim().parse().unwrap_or(0);
                            }
                            // Skip other headers (e.g., Content-Type)
                        }

                        if content_length == 0 {
                            continue;
                        }

                        // Read message content
                        let mut content_buf = vec![0u8; content_length];
                        if child_stdout.read_exact(&mut content_buf).await.is_err() {
                            break;
                        }

                        // Send to WebSocket
                        match String::from_utf8(content_buf) {
                            Ok(text) => {
                                let mut write = write_clone.lock().await;
                                if write.send(Message::Text(text)).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("[LSP] Invalid UTF-8 from Intelephense: {}", e);
                                break;
                            }
                        }
                    }
                });

                // Wait for either task to finish (connection closed or process died)
                tokio::select! {
                    _ = stdin_task => {
                        eprintln!("[LSP] WebSocket -> stdio task finished");
                    }
                    _ = stdout_task => {
                        eprintln!("[LSP] stdio -> WebSocket task finished");
                    }
                }

                // Clean up process
                let _ = child.kill().await;
                let mut write = write_handle.lock().await;
                let _ = write.send(Message::Close(None)).await;
            });
        }
    });

    state.task = Some(task);
    Ok(())
}

pub async fn restart_server(app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut state = STATE.lock().map_err(|_| "Failed to lock LSP state".to_string())?;
        if let Some(handle) = state.task.take() {
            handle.abort();
        }
    }
    start_server_if_needed(app)
}
