/// <reference types="vite/client" />

declare global {
  interface Window {
    ipcRenderer: any
    platformInfo: any
    Sfdump: any
  }
}

export {}
