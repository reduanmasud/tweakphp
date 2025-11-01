import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'

// Local event registry to mimic Electron's ipcRenderer.on/removeListener
const listeners = new Map<string, Set<(...args: any[]) => void>>()

function addListener(channel: string, cb: (...args: any[]) => void) {
	if (!listeners.has(channel)) listeners.set(channel, new Set())
	listeners.get(channel)!.add(cb)
}

function removeListener(channel: string, cb: (...args: any[]) => void) {
	listeners.get(channel)?.delete(cb)
}

function once(channel: string, cb: (...args: any[]) => void) {
	const wrapper = (...args: any[]) => {
		removeListener(channel, wrapper)
		cb(...args)
	}
	addListener(channel, wrapper)
}

function emitLocal(channel: string, ...args: any[]) {
	const set = listeners.get(channel)
	if (!set) return
	for (const cb of Array.from(set)) {
		try {
			cb(...args)
		} catch (e) {
			console.error(`[ipcRenderer] listener error on ${channel}:`, e)
		}
	}
}

async function handleSend(channel: string, payload?: any) {
	try {
		switch (channel) {
			case 'init': {
				const result = await invoke('init')
				emitLocal('init.reply', result)
				break
			}
			case 'link.open': {
				// Use opener plugin directly from frontend
				if (typeof payload === 'string') {
					const { openUrl } = await import('@tauri-apps/plugin-opener')
					await openUrl(payload)
				}
				break
			}
			case 'source.open': {
				const directory = await openDialog({ directory: true, multiple: false })
				emitLocal('source.open.reply', directory)
				break
			}
			case 'source.openPath': {
				// Use Rust command to open path in system file manager
				try {
					await invoke('source_open_path', { path: payload })
				} catch (error) {
					console.error('[ipcRenderer] Error opening path:', error)
				}
				break
			}
			case 'update.check': {
				// Use Rust command to trigger update check
				try {
					await invoke('update_check')
					// The Rust command emits 'update.checking' event
					// The updater plugin will emit 'tauri://update-available' or 'tauri://update-not-available'
				} catch (error) {
					console.error('[ipcRenderer] Error checking for updates:', error)
				}
				break
			}
			case 'update.download': {
				// Use Rust command to trigger update download
				try {
					await invoke('update_download')
					// The updater plugin handles the actual download
					// It will emit 'tauri://update-downloaded' when complete
				} catch (error) {
					console.error('[ipcRenderer] Error downloading update:', error)
				}
				break
			}
			case 'lsp.restart': {
				try {
					await invoke('lsp_restart')
					emitLocal('lsp.restart.success')
				} catch (error) {
					console.error('[ipcRenderer] Error restarting LSP:', error)
				}
				break
			}
			case 'settings.store': {
				// Store settings using Rust command
				try {
					const foundPhpPath = await invoke<string>('store_settings', { settingsData: payload })
					// If PHP path was found in a directory, emit the event
					if (foundPhpPath) {
						emitLocal('settings.php-located', foundPhpPath)
					}
					// Emit success event if needed by frontend
					emitLocal('settings.stored')
				} catch (error) {
					console.error('[ipcRenderer] Error storing settings:', error)
					emitLocal('settings.store.error', error)
				}
				break
			}
			case 'client.connect': {
				try {
					const result = await invoke('client_connect', { payload })
					emitLocal('client.connect.reply', result)
				} catch (error) {
					console.error('[client.connect] Error:', error)
					// Extract error message from Tauri error
					const errorMessage = error instanceof Error 
						? error.message 
						: typeof error === 'string' 
							? error 
							: String(error)
					emitLocal('client.connect.reply', { 
						connected: false, 
						error: errorMessage,
						data: payload?.data 
					})
				}
				break
			}
			case 'client.execute': {
				try {
					const result = await invoke('client_execute', { payload })
					emitLocal('client.execute.reply', result)
				} catch (error) {
					emitLocal('client.execute.reply', error)
				}
				break
			}
			case 'client.action': {
				try {
					const result = await invoke('client_action', { payload })
					emitLocal('client.action.reply', result)
				} catch (error) {
					emitLocal('client.action.reply', { type: payload?.type, error })
				}
				break
			}
			case 'client.info': {
				try {
					const result = await invoke('client_info', { payload })
					emitLocal('client.info.reply', result)
				} catch (error) {
					emitLocal('client.info.reply', error)
				}
				break
			}
			default: {
				console.warn('[ipcRenderer] Unknown channel:', channel, payload)
			}
		}
	} catch (error) {
		console.error(`[ipcRenderer] Error handling ${channel}:`, error)
	}
}

// History API implementation using Tauri commands
function setupHistoryApi() {
	if (typeof window === 'undefined') return

	const historyApi = {
		add: async (tabId: number, code: string, cursor: { lineNumber: number; column: number }) => {
			try {
				await invoke('code_add', {
					tabId,
					code,
					cursor: {
						lineNumber: cursor.lineNumber,
						column: cursor.column,
					},
				})
				// Emit reply for compatibility
				emitLocal('code-add.reply', { data: { success: true }, error: null })
			} catch (error) {
				console.error('[historyApi] Error adding code history:', error)
				emitLocal('code-add.reply', { data: null, error: 'Failed to add code history' })
			}
		},
		undo: async (tabId: number) => {
			try {
				const result = await invoke('code_undo', { tabId })
				if (result && result.data) {
					emitLocal('code-undo.reply', result)
				} else {
					emitLocal('code-undo.reply', result || { data: null, error: 'No previous state to undo.' })
				}
			} catch (error) {
				console.error('[historyApi] Error undoing:', error)
				emitLocal('code-undo.reply', { data: null, error: 'Failed to undo' })
			}
		},
		redo: async (tabId: number) => {
			try {
				const result = await invoke('code_redo', { tabId })
				if (result && result.data) {
					emitLocal('code-redo.reply', result)
				} else {
					emitLocal('code-redo.reply', result || { data: null, error: 'No next state to redo.' })
				}
			} catch (error) {
				console.error('[historyApi] Error redoing:', error)
				emitLocal('code-redo.reply', { data: null, error: 'Failed to redo' })
			}
		},
		onUndoReply: (callback: (data: { code: string; cursor?: { lineNumber: number; column: number } }) => void) => {
			addListener('code-undo.reply', (event: any) => {
				if (event && event.data) {
					callback(event.data)
				}
			})
		},
		onRedoReply: (callback: (data: { code: string; cursor?: { lineNumber: number; column: number } }) => void) => {
			addListener('code-redo.reply', (event: any) => {
				if (event && event.data) {
					callback(event.data)
				}
			})
		},
		removeAllListeners: () => {
			removeListener('code-undo.reply', () => {})
			removeListener('code-redo.reply', () => {})
			// Clear all listeners for these channels
			listeners.delete('code-undo.reply')
			listeners.delete('code-redo.reply')
		},
	}

	// @ts-ignore
	window.historyApi = historyApi
}

export function installIpcShimOnWindow() {
	const ipcShim = {
		send: (channel: string, payload?: any) => void handleSend(channel, payload),
		on: (channel: string, cb: (...args: any[]) => void) => addListener(channel, cb),
		removeListener: (channel: string, cb: (...args: any[]) => void) => removeListener(channel, cb),
		once: (channel: string, cb: (...args: any[]) => void) => once(channel, cb),
	}
	// @ts-ignore
	if (typeof window !== 'undefined') {
		window.ipcRenderer = ipcShim
		
		// Set up platformInfo API (needed by App.vue and other components)
		// Detect platform synchronously using navigator (works immediately)
		// This matches Electron's behavior where platform is available immediately
		const detectPlatform = (): string => {
			const navPlatform = navigator.platform.toLowerCase()
			if (navPlatform.includes('mac') || navPlatform.includes('darwin')) return 'darwin'
			if (navPlatform.includes('win')) return 'win32'
			if (navPlatform.includes('linux')) return 'linux'
			return 'darwin' // default fallback
		}
		
		let cachedPlatform = detectPlatform()
		
		window.platformInfo = {
			getPlatform: () => cachedPlatform
		}
		
		// Optionally update with Tauri's more accurate platform detection
		invoke('get_platform').then((platform: string) => {
			// Convert Tauri format to Electron format
			if (platform === 'macos') cachedPlatform = 'darwin'
			else if (platform === 'windows') cachedPlatform = 'win32'
			else if (platform === 'linux') cachedPlatform = 'linux'
		}).catch(() => {
			// Keep using navigator-based detection if Tauri call fails
		})
		
		// Also set up history API
		setupHistoryApi()
	}
}
