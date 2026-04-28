import { save } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { useCallback } from 'react';
import { useEditorContext } from '../context/editorContext';
import { FileTab } from '../types/FileTab';
import { NOTIFY } from '@/lib/notify';
import { openPath } from '@/services/api/dialog';
import { writeFile } from '@/services/api/fs';
import { generateMappingTable, mappingTableToString } from '@/services/api/mapping_table';

/** Editor side-effect commands */
export const useTabEditorCommands = () => {
  const [state, dispatch] = useEditorContext();

  const openFiles = useCallback(
    async (paths: string[]) => {
      const opened: FileTab[] = [];

      for (const path of paths) {
        try {
          if (!path.startsWith('.txt')) {
            const table = await generateMappingTable(path, 'txt_stem_stripped');

            opened.push({
              id: path,
              inputPath: path,
              outputPath: inferMappingTableTxt(path),
              text: mappingTableToString(table),
              originalText: '',
              tableGen: { strategy: 'txt_stem_stripped' },
            });

            continue;
          }

          const text = await readTextFile(path);
          opened.push({
            id: path,
            inputPath: path,
            outputPath: inferMappingTableTxt(path),
            text,
            originalText: text,
            tableGen: { strategy: 'txt_stem_stripped' },
          });
        } catch (e) {
          NOTIFY.error(`Failed to load: ${path}`);
          if (e instanceof Error) console.error(e);
        }
      }

      if (opened.length) {
        dispatch({ type: 'OPEN_TABS', tabs: opened });
      }
    },
    [dispatch],
  );

  const handleOpenClick = async () => {
    const selected = await openPath('', {
      multiple: true,
      filters: [{ name: 'Mapping table Files', extensions: ['txt'] }],
    });
    if (selected) {
      await openFiles(Array.isArray(selected) ? selected : [selected]);
    }
  };

  /**
   *  Save the current active tab's text to its output path. If the output path is not set, a save dialog will be shown.
   */
  const saveCurrent = async () => {
    const tab = state.tabs[state.active];
    if (!tab) return;

    try {
      const path = await save({
        defaultPath: tab.outputPath,
        filters: [{ name: 'mapping_table.txt', extensions: ['txt'] }],
      });
      if (!path) return;

      await writeFile(path, tab.text);
      dispatch({ type: 'MARK_SAVED', index: state.active });
      NOTIFY.success('Saved successfully');
      return path;
    } catch (e) {
      NOTIFY.error('Save failed.');
      if (e instanceof Error) console.error(e);
    }
  };

  const handleNewClick = () => {
    dispatch({ type: 'NEW_TAB' });
  };

  return { openFiles, handleOpenClick, handleNewClick, saveCurrent };
};

const inferMappingTableTxt = (input: string) => {
  const base = input.split(/[\\/]/).pop() ?? input;
  const i = base.lastIndexOf('.');
  return i === -1
    ? `./mapping_tables/${base}_mapping_table.txt`
    : `./mapping_tables/${base.slice(0, i)}_mapping_table.txt`;
};
