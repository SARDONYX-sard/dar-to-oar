import { LogLevel } from '@/tauri_cmd';

export type EditorMode = 'default' | 'vim';
/** 'default' if null or undefined */
export function selectEditorMode(select?: string | null): EditorMode {
  if (select === 'vim') {
    return select;
  } else {
    return 'default';
  }
}

/** 'error' if null or undefined */
export function selectLogLevel(logLevel?: string | null): LogLevel {
  switch (logLevel) {
    case 'trace':
    case 'debug':
    case 'info':
    case 'warn':
    case 'error':
      return logLevel;
    default:
      return 'error';
  }
}
