use crate::client::{get_client, ClientConnectPayload, ClientExecutePayload, ClientActionPayload, ClientInfoPayload};
use tauri::{command, AppHandle, Emitter};

#[command]
pub async fn client_connect(
    app: AppHandle,
    payload: ClientConnectPayload,
) -> Result<serde_json::Value, String> {
    let mut client = get_client(payload.connection.clone(), app.clone()).await?;
    
    match client.connect().await {
        Ok(_) => {
            if let Some(ref data) = payload.data {
                if data.get("setup").and_then(|v| v.as_bool()).unwrap_or(false) {
                    if let Err(e) = client.setup().await {
                        let connection = client.get_connection();
                        let _ = app.emit("client:connect:reply", serde_json::json!({
                            "connected": false,
                            "connection": connection,
                            "data": payload.data,
                            "error": e,
                        }));
                        return Err(e);
                    }
                }
            }
            
            let connection = client.get_connection();
            let reply = serde_json::json!({
                "connected": true,
                "connection": connection,
                "data": payload.data,
            });
            
            let _ = app.emit("client:connect:reply", &reply);
            
            client.disconnect().await.ok();
            Ok(reply)
        }
        Err(e) => {
            let connection = client.get_connection();
            let reply = serde_json::json!({
                "connected": false,
                "connection": connection,
                "data": payload.data,
                "error": e.clone(),
            });
            
            let _ = app.emit("client:connect:reply", &reply);
            
            client.disconnect().await.ok();
            Err(e)
        }
    }
}

#[command]
pub async fn client_execute(
    app: AppHandle,
    payload: ClientExecutePayload,
) -> Result<serde_json::Value, String> {
    let mut client = get_client(payload.connection.clone(), app.clone()).await?;
    
    match client.connect().await {
        Ok(_) => {
            match client.execute(payload.code, payload.loader).await {
                Ok(result) => {
                    let trimmed = result.trim();
                    let output = if let Some(start) = trimmed.find("TWEAKPHP_RESULT:") {
                        let json_str = trimmed[start + 16..].trim();
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(parsed) => parsed,
                            Err(_) => serde_json::Value::String(json_str.to_string()),
                        }
                    } else {
                        serde_json::Value::String(trimmed.to_string())
                    };
                    
                    let _ = app.emit("client:execute:reply", &output);
                    
                    client.disconnect().await.ok();
                    Ok(output)
                }
                Err(e) => {
                    let error_value = serde_json::Value::String(e.clone());
                    let _ = app.emit("client:execute:reply", &error_value);
                    
                    client.disconnect().await.ok();
                    Err(e)
                }
            }
        }
        Err(e) => {
            let error_value = serde_json::Value::String(e.clone());
            let _ = app.emit("client:execute:reply", &error_value);
            
            client.disconnect().await.ok();
            Err(e)
        }
    }
}

#[command]
pub async fn client_action(
    app: AppHandle,
    payload: ClientActionPayload,
) -> Result<serde_json::Value, String> {
    let mut client = get_client(payload.connection.clone(), app.clone()).await?;
    
    match client.connect().await {
        Ok(_) => {
            match client.action(payload.type_field.clone(), payload.data.clone()).await {
                Ok(result) => {
                    let reply = serde_json::json!({
                        "type": payload.type_field,
                        "result": result,
                    });
                    
                    let _ = app.emit("client:action:reply", &reply);
                    
                    client.disconnect().await.ok();
                    Ok(reply)
                }
                Err(e) => {
                    let reply = serde_json::json!({
                        "type": payload.type_field,
                        "error": e.clone(),
                    });
                    
                    let _ = app.emit("client:action:reply", &reply);
                    
                    client.disconnect().await.ok();
                    Err(e)
                }
            }
        }
        Err(e) => {
            let reply = serde_json::json!({
                "type": payload.type_field,
                "error": e.clone(),
            });
            
            let _ = app.emit("client:action:reply", &reply);
            
            client.disconnect().await.ok();
            Err(e)
        }
    }
}

#[command]
pub async fn client_info(
    app: AppHandle,
    payload: ClientInfoPayload,
) -> Result<String, String> {
    let mut client = get_client(payload.connection.clone(), app.clone()).await?;
    
    match client.connect().await {
        Ok(_) => {
            match client.info(payload.loader).await {
                Ok(result) => {
                    let _ = app.emit("client:info:reply", &result);
                    
                    client.disconnect().await.ok();
                    Ok(result)
                }
                Err(e) => {
                    let _ = app.emit("client:info:reply", &e);
                    
                    client.disconnect().await.ok();
                    Err(e)
                }
            }
        }
        Err(e) => {
            let _ = app.emit("client:info:reply", &e);
            
            client.disconnect().await.ok();
            Err(e)
        }
    }
}

