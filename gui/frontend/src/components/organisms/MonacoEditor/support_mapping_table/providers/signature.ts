import { MAPPING_TABLE_LANGUAGE_ID } from '..';

import type * as monaco from 'monaco-editor';

export const registerSignatureHelpProvider = (_: monaco.editor.IStandaloneCodeEditor, monacoNS: typeof monaco) => {
  return monacoNS.languages.registerSignatureHelpProvider(MAPPING_TABLE_LANGUAGE_ID, {
    signatureHelpTriggerCharacters: [' '],

    provideSignatureHelp(model, position) {
      const line = model.getLineContent(position.lineNumber);

      // Ignore if cursor is after comment
      const commentIndex = line.indexOf('//');
      if (commentIndex !== -1 && position.column - 1 > commentIndex) {
        return null;
      }

      const before = line.slice(0, position.column - 1);
      const clean = before.split('//')[0];

      const trimmed = clean.trimStart();

      if (trimmed === '') {
        return valueOf(
          '<priority/form_id: number>',
          'priority: integer / form_id: non-prefix hex string (base id pattern).',
          'priority/form_id',
        );
      }

      const firstTokenMatch = trimmed.match(/^\S+/);
      const firstTokenLength = firstTokenMatch ? firstTokenMatch[0].length : 0;

      const leadingSpaces = clean.length - trimmed.length;
      const firstTokenEnd = leadingSpaces + firstTokenLength;

      const col = position.column - 1;

      if (col <= firstTokenEnd) {
        return valueOf(
          '<priority/form_id: number>',
          'priority: integer / form_id: non-prefix hex string (base id pattern).',
          'priority/form_id',
        );
      }

      return valueOf('<rename_to: string>', 'Target name.', 'rename_to');
    },
  });
};

const valueOf = (label: string, doc: string, paramLabel: string): monaco.languages.SignatureHelpResult => ({
  value: {
    signatures: [{ label, documentation: doc, parameters: [{ label: paramLabel }] }],
    activeSignature: 0,
    activeParameter: 0,
  },
  dispose() {},
});
