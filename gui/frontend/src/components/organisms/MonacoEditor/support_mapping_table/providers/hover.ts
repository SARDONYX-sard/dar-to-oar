import * as monaco from 'monaco-editor';
import { MAPPING_TABLE_LANGUAGE_ID } from '..';

export const registerHoverProvider = (monacoEnv: typeof monaco) => {
  return monacoEnv.languages.registerHoverProvider(MAPPING_TABLE_LANGUAGE_ID, {
    provideHover(model, position) {
      const line = model.getLineContent(position.lineNumber);

      // Ignore hover inside comments
      const commentIndex = line.indexOf('//');
      if (commentIndex !== -1 && position.column - 1 > commentIndex) {
        return null;
      }

      const content = commentIndex !== -1 ? line.slice(0, commentIndex) : line;
      if (!content.trim()) return null;

      const match = content.match(/^\s*(\S+)/);
      if (!match) return null;

      const priority = match[1];

      const tmpRename = content.slice(match[0].length).trim();
      const inferred = !tmpRename ? inferRename(model, position.lineNumber) : null;
      const rename = tmpRename || inferred || '';

      const doc = collectDocComments(model, position.lineNumber);

      return {
        contents: [
          ...(doc
            ? [
                {
                  value: doc,
                },
              ]
            : []),

          {
            value: buildUnifiedDoc({ priority, rename, isInferred: !!inferred }),
          },
        ],
      };
    },
  });
};

// -----------------------------
// Unified Doc Builder
// -----------------------------
type UnifiedInput = {
  priority: string;
  rename: string;
  isInferred: boolean;
};

const buildUnifiedDoc = ({ priority, rename, isInferred }: UnifiedInput) => {
  const label = isActorBase(priority) ? 'Form ID (Actor Base)' : 'Priority';
  const renameLabel = isInferred ? 'Inferred Sequential Completion' : 'Rename to';

  return `
${label}: \`${priority}\` -> ${renameLabel}: \`${rename}\`

---

# Description

Defines a mapping from a priority identifier to a target name.

This name is used in the \`name\` field of OAR's \`config.json\` file and is the name displayed on the OAR menu screen.

- The first token is treated as the priority
- The second token (optional) is treated as the rename target
- If there are items you don't want to rename, just delete them.

---

# Examples

- Priority pattern (number + name)

\`\`\`mapping_table
// format: <priority> <rename_to>
200 Attack
400000 Foot
\`\`\`

- Actor base pattern (non prefix hex + name)

\`\`\`mapping_table
// format: <form id> <rename_to>
0001A692 GreatSword Animation
\`\`\`

- Mixed usage

\`\`\`mapping_table
200 Attack
0001A692 GreatSword Animation
\`\`\`

- Sequential Completion (Inferred Rename)

  If the rename target is omitted, it will be automatically generated based on the previous entry.

  Rules:
  - Uses the most recent explicit rename as the base
  - Appends an incrementing suffix: \`_1\`, \`_2\`, ...
  - The first entry MUST define a rename target (no base exists)

\`\`\`mapping_table
200 Attack
201        // → Attack_1
202        // → Attack_2
\`\`\`
`;
};

// -----------------------------
// Comment Collector
// -----------------------------
const collectDocComments = (model: monaco.editor.ITextModel, lineNumber: number) => {
  const docs: string[] = [];

  // Above comments
  for (let i = lineNumber - 1; i >= 1; i--) {
    const line = model.getLineContent(i).trim();
    if (!line) break;
    if (!line.startsWith('//')) break;

    docs.unshift(line.replace(/^\/\/\s?/, ''));
  }

  // Inline comment
  const currentLine = model.getLineContent(lineNumber);
  const idx = currentLine.indexOf('//');
  if (idx !== -1) {
    const inline = currentLine.slice(idx + 2).trim();
    if (inline) docs.push(inline);
  }

  return docs.length ? docs.join('\n') : null;
};

export const isActorBase = (value: string): boolean => {
  return /^[0-9a-fA-F]+$/.test(value) && /[a-fA-F]/.test(value);
};

const inferRename = (model: monaco.editor.ITextModel, lineNumber: number): string | null => {
  let lastBase: string | null = null;
  let counter = 0;

  for (let i = 1; i < lineNumber; i++) {
    const line = model.getLineContent(i);

    const commentIndex = line.indexOf('//');
    const content = commentIndex !== -1 ? line.slice(0, commentIndex) : line;

    if (!content.trim()) continue;

    const match = content.match(/^\s*(\S+)/);
    if (!match) continue;

    const rest = content.slice(match[0].length).trim();

    if (rest) {
      lastBase = rest;
      counter = 0;
    } else if (lastBase) {
      counter++;
    }
  }

  if (!lastBase) return null;

  return `${lastBase}_${counter + 1}`;
};
