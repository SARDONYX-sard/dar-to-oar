import * as monaco from 'monaco-editor';
import { MAPPING_TABLE_LANGUAGE_ID } from '..';

export const registerCompletionProvider = (monacoEnv: typeof monaco) => {
  return monacoEnv.languages.registerCompletionItemProvider(MAPPING_TABLE_LANGUAGE_ID, {
    triggerCharacters: [' ', '"'],

    provideCompletionItems(document, position): monaco.languages.CompletionList {
      const line = document.getLineContent(position.lineNumber);
      const before = line.slice(0, position.column - 1);
      const clean = before.split('//')[0]; // strip comment

      const range: monaco.IRange = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: position.column,
        endColumn: line.length + 1,
      };

      const trimmed = clean.trimStart();
      const col = position.column - 1;

      if (!trimmed) {
        return prioritySuggestions(range);
      }

      const firstTokenMatch = trimmed.match(/^\S+/);
      const firstTokenLength = firstTokenMatch ? firstTokenMatch[0].length : 0;

      const leadingSpaces = clean.length - trimmed.length;
      const firstTokenEnd = leadingSpaces + firstTokenLength;

      if (col <= firstTokenEnd) {
        return prioritySuggestions(range);
      }

      return renameSuggestions(range);
    },
  });
};

const prioritySuggestions = (range: monaco.IRange): monaco.languages.CompletionList => ({
  suggestions: [
    {
      label: 'mapping_entry',
      kind: monaco.languages.CompletionItemKind.Snippet,
      insertText: '${1:200} ${2:RenameSample}',
      insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      range,
      documentation: 'Insert mapping entry (priority + rename_to)',
      sortText: '0',
    },
    {
      label: 'priority',
      kind: monaco.languages.CompletionItemKind.Value,
      insertText: '${1:200}',
      insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      range,
      documentation: 'Numeric priority (integer)',
      sortText: '1',
    },
    {
      label: 'form_id',
      kind: monaco.languages.CompletionItemKind.Value,
      insertText: '${1:0001A692}',
      insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      range,
      documentation: 'Actor base ID (non prefix Hex String)',
      sortText: '100',
    },
  ],
});

const renameSuggestions = (range: monaco.IRange): monaco.languages.CompletionList => ({
  suggestions: [
    {
      label: 'rename_to',
      kind: monaco.languages.CompletionItemKind.Text,
      insertText: '${1:RenameSample}',
      insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      range,
      documentation: 'Rename target.',
    },
  ],
});
