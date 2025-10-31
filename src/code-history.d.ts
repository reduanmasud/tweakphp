export interface IHistoryApi {
  add: (tabId: number, code: string, cursor?: any) => void
  undo: (tabId: number) => void
  redo: (tabId: number) => void
  onUndoReply: (callback: (data: { code: string; cursor?: any }) => void) => void
  onRedoReply: (callback: (data: { code: string; cursor?: any }) => void) => void
  removeAllListeners: () => void
}

declare global {
  interface Window {
    historyApi: IHistoryApi
  }
}
