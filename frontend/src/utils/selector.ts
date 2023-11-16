import { LogLevel } from '@/tauri_cmd';

export type EditorMode = 'default' | 'vim' | undefined;
export function selectEditorMode(select: string): EditorMode {
  if (select === 'vim') {
    return select;
  } else {
    return 'default';
  }
}

export function selectLogLevel(logLevel: string): LogLevel {
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
