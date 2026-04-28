import * as monaco from 'monaco-editor';
import { MAPPING_TABLE_LANGUAGE_ID } from '..';

/** Monarch fallback tokenizer */
export const registerMonarchTokensProvider = (monacoEnv: typeof monaco) => {
  monacoEnv.languages.setMonarchTokensProvider(MAPPING_TABLE_LANGUAGE_ID, {
    tokenizer: {
      root: [
        // comment (highest priority)
        [/\/\/.*/, 'comment'],

        // line start: priority (non prefix hex)
        [/^\s*-?(?:[0-9a-fA-F]+)/, 'number'],

        // rename_to (rest of word chunks)
        [/[^\s#"]+/, 'string'],

        // whitespace
        [/\s+/, 'white'],
      ],
    },
  });

  // Pair color seems to only work when done in the following manner.
  // See: https://github.com/microsoft/monaco-editor/issues/3907#issuecomment-1502932923
  monacoEnv.languages.setLanguageConfiguration(MAPPING_TABLE_LANGUAGE_ID, {
    brackets: [
      ['(', ')'],
      ['{', '}'],
      ['[', ']'],
    ],
    comments: {
      lineComment: '//',
    },
  });
};
