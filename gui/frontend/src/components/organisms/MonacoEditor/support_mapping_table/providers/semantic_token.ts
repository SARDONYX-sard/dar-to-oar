import { MAPPING_TABLE_LANGUAGE_ID } from '..';

import type * as monaco from 'monaco-editor';

export const registerDocumentSemanticTokensProvider = (monacoEnv: typeof monaco) => {
  return monacoEnv.languages.registerDocumentSemanticTokensProvider(MAPPING_TABLE_LANGUAGE_ID, {
    getLegend: () => ({ tokenTypes: TOKEN_TYPES.slice(), tokenModifiers: TOKEN_MODIFIERS }),

    provideDocumentSemanticTokens(model) {
      const lines = model.getLinesContent();
      const data: number[] = [];

      let lastLine = 0;
      let lastChar = 0;

      const pushToken = (line: number, start: number, length: number, type: TokenType) => {
        const tokenTypeIndex = TOKEN_TYPES.indexOf(type);
        if (tokenTypeIndex === -1) return;

        const deltaLine = line - lastLine;
        const deltaStart = deltaLine === 0 ? start - lastChar : start;

        data.push(deltaLine, deltaStart, length, tokenTypeIndex, 0);

        lastLine = line;
        lastChar = start;
      };

      for (let i = 0; i < lines.length; i++) {
        const text = lines[i];

        // --- comment ---
        const commentIndex = text.indexOf('//');
        if (commentIndex !== -1) {
          pushToken(i, commentIndex, text.length - commentIndex, 'comment');
        }

        const line = commentIndex !== -1 ? text.slice(0, commentIndex) : text;

        if (!line.trim()) continue;

        // --- priority (first token, NOT number) ---
        const match = line.match(/^\s*(\S+)/);
        if (!match || match.index === undefined) continue;

        const priority = match[1];
        const priorityStart = match.index + (match[0].length - priority.length);

        pushToken(i, priorityStart, priority.length, 'number');

        // --- rename_to ---
        const restStart = priorityStart + priority.length;
        const rest = line.slice(restStart).trimStart();

        if (!rest) continue;

        const valueStart = line.indexOf(rest, restStart);

        // quoted string
        pushToken(i, valueStart, rest.length, 'string');
      }

      return { data: new Uint32Array(data) };
    },

    releaseDocumentSemanticTokens: () => {},
  });
};

const TOKEN_TYPES = [
  'number', // priority
  'string', // rename_to
  'comment',
] as const;

type TokenType = (typeof TOKEN_TYPES)[number];

const TOKEN_MODIFIERS: string[] = [];
