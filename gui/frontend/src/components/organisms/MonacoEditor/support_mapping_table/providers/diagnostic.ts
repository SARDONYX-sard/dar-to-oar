import { MAPPING_TABLE_LANGUAGE_ID } from '..';

import type { OnMount } from '@monaco-editor/react';
import type * as monaco from 'monaco-editor';

export const registerCodeLen = (
  editor: monaco.editor.IStandaloneCodeEditor,
  monacoEnv: typeof monaco,
): monaco.IDisposable[] => {
  let disposables: monaco.IDisposable[] = [];

  disposables.push(
    editor.onDidFocusEditorText(() => {
      if (editor.getModel()?.getLanguageId() === MAPPING_TABLE_LANGUAGE_ID) {
        updateDiagnostics(editor, monacoEnv);
      }
    }),
  );

  disposables.push(
    editor.onDidChangeModelContent(() => {
      if (editor.getModel()?.getLanguageId() === MAPPING_TABLE_LANGUAGE_ID) {
        updateDiagnostics(editor, monacoEnv);
      }
    }),
  );

  return disposables;
};

const updateDiagnostics: OnMount = (editor, monacoEnv) => {
  const model = editor.getModel();
  if (!model) return;

  const markers: monaco.editor.IMarkerData[] = [];
  const lines = model.getLinesContent();

  const seenPriority = new Map<string, number>();

  let lastBaseName: string | null = null;

  for (let lineNumber = 1; lineNumber <= lines.length; lineNumber++) {
    const raw = lines[lineNumber - 1];

    // strip comment
    const commentIndex = raw.indexOf('//');
    const line = commentIndex !== -1 ? raw.slice(0, commentIndex) : raw;

    if (!line.trim()) continue;

    // --- priority (string) ---
    const match = line.match(/^\s*(\S+)/);
    if (!match) {
      markers.push({
        severity: monacoEnv.MarkerSeverity.Error,
        message: 'Missing priority.',
        startLineNumber: lineNumber,
        endLineNumber: lineNumber,
        startColumn: 1,
        endColumn: raw.length + 1,
      });
      continue;
    }

    const priority = match[1];

    // duplicate check
    if (seenPriority.has(priority)) {
      markers.push({
        severity: monacoEnv.MarkerSeverity.Warning,
        message: `Duplicate priority: ${priority}`,
        startLineNumber: lineNumber,
        endLineNumber: lineNumber,
        startColumn: 1,
        endColumn: match[0].length + 1,
      });
    } else {
      seenPriority.set(priority, lineNumber);
    }

    // --- rename_to ---
    const rest = line.slice(match[0].length).trim();

    if (!rest) {
      if (!lastBaseName) {
        markers.push({
          severity: monacoEnv.MarkerSeverity.Error,
          message: 'First entry cannot omit rename_to.',
          startLineNumber: lineNumber,
          endLineNumber: lineNumber,
          startColumn: match[0].length + 1,
          endColumn: raw.length + 1,
        });
      }
      continue;
    }

    lastBaseName = rest;

    // optional: string
    if (rest.startsWith('"') && !rest.endsWith('"')) {
      markers.push({
        severity: monacoEnv.MarkerSeverity.Error,
        message: 'Unterminated string.',
        startLineNumber: lineNumber,
        endLineNumber: lineNumber,
        startColumn: raw.indexOf(rest) + 1,
        endColumn: raw.length + 1,
      });
    }
  }

  monacoEnv.editor.setModelMarkers(model, DIAGNOSTIC_OWNER, markers);
};

const DIAGNOSTIC_OWNER = 'mapping-table';

export const clearDiagnostics = (monacoEnv: typeof monaco) => {
  monacoEnv.editor.removeAllMarkers(DIAGNOSTIC_OWNER);
};
