import * as monaco from 'monaco-editor';
import { MAPPING_TABLE_LANGUAGE_ID } from '..';

export const registerDocumentFormattingEditProvider = (monacoEnv: typeof monaco) => {
  return monacoEnv.languages.registerDocumentFormattingEditProvider(MAPPING_TABLE_LANGUAGE_ID, {
    provideDocumentFormattingEdits(model) {
      return [
        {
          range: model.getFullModelRange(),
          text: formatText(model.getValue()),
        },
      ];
    },
  });
};

const formatText = (text: string): string => {
  const lines = text.split('\n');

  return lines.map(formatLine).join('\n');
};

const formatLine = (raw: string): string => {
  const trimmed = raw.trim();

  // Empty line
  if (!trimmed) return '';

  // Comment-only line
  if (trimmed.startsWith('//')) {
    return `// ${trimmed.slice(2).trim()}`;
  }

  // Split comment
  const commentIndex = raw.indexOf('//');
  const comment = commentIndex !== -1 ? raw.slice(commentIndex + 2).trim() : '';
  const content = commentIndex !== -1 ? raw.slice(0, commentIndex) : raw;

  // Parse priority (first non-whitespace token)
  const match = content.match(/^\s*(\S+)/);
  if (!match) {
    // Invalid line → normalize whitespace only
    return trimmed;
  }

  const priority = match[1];

  // Parse rename_to (optional)
  const rest = content.slice(match[0].length).trim();

  let result = priority;

  if (rest) {
    result += ` ${rest}`;
  }

  if (comment) {
    result += ` // ${comment}`;
  }

  return result;
};
