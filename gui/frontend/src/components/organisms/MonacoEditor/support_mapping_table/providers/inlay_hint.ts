import * as monaco from 'monaco-editor';
import { MAPPING_TABLE_LANGUAGE_ID } from '..';
import { isActorBase } from './hover';

export const registerInlayHintsProvider = (monacoEnv: typeof monaco) => {
  return monacoEnv.languages.registerInlayHintsProvider(MAPPING_TABLE_LANGUAGE_ID, {
    provideInlayHints(model, range) {
      const hints: monaco.languages.InlayHint[] = [];

      let lastBase: string | null = null;
      let counter = 0;

      for (let lineNumber = 1; lineNumber <= model.getLineCount(); lineNumber++) {
        const raw = model.getLineContent(lineNumber);

        // strip comment
        const commentIndex = raw.indexOf('//');
        const content = commentIndex !== -1 ? raw.slice(0, commentIndex) : raw;

        if (!content.trim()) continue;

        // --- priority (string token) ---
        const match = content.match(/^\s*(\S+)/);
        if (!match || match.index === undefined) continue;

        const priority = match[1];

        const priorityStart = match.index + (match[0].length - priority.length);

        // show hint only if in visible range
        if (lineNumber >= range.startLineNumber && lineNumber <= range.endLineNumber) {
          hints.push({
            position: { lineNumber, column: priorityStart + 1 },
            label: isActorBase(priority) ? 'form_id:' : 'priority:',
            kind: monacoEnv.languages.InlayHintKind.Type,
            paddingRight: true,
          });
        }

        // --- rename_to ---
        const restStart = priorityStart + priority.length;
        const rest = content.slice(restStart).trimStart();

        if (rest) {
          // explicit rename
          lastBase = rest;
          counter = 0;

          if (lineNumber >= range.startLineNumber && lineNumber <= range.endLineNumber) {
            const valueStart = content.indexOf(rest, restStart);

            hints.push({
              position: { lineNumber, column: valueStart + 1 },
              label: 'rename:',
              kind: monacoEnv.languages.InlayHintKind.Type,
              paddingRight: true,
            });
          }
        } else {
          // inferred rename
          if (lastBase) {
            counter++;

            if (lineNumber >= range.startLineNumber && lineNumber <= range.endLineNumber) {
              hints.push({
                position: { lineNumber, column: priorityStart + priority.length + 1 },
                label: `→ ${lastBase}_${counter}`,
                kind: monacoEnv.languages.InlayHintKind.Parameter,
                paddingLeft: true,
              });
            }
          }
        }
      }

      return { hints, dispose: () => {} };
    },
  });
};
