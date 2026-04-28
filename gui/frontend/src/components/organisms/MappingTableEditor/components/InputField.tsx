import { Button, TextField, Box } from '@mui/material';
import { useCallback, useState } from 'react';
import { OpenIcon } from '../../OpenIcon/OpenIcon';
import { useEditorContext } from '../context/editorContext';
import { MappingPreviewDialog } from './MappingPreviewDialog';
import { StrategyList } from './StrategyList';
import { useTranslation } from '@/components/hooks/useTranslation';
import { SelectPathButton } from '@/components/molecules/SelectPathButton';
import { NOTIFY } from '@/lib/notify';
import {
  generateMappingTable,
  mappingTableFromStr,
  type MappingTableGenStrategy,
  mappingTableToString,
} from '@/services/api/mapping_table';

export const InputField = () => {
  const { t } = useTranslation();
  const [state, dispatch] = useEditorContext();
  const tab = state.tabs[state.active];

  const [generating, setGenerating] = useState(false);
  const [previewOpen, setPreviewOpen] = useState(false);
  const [draftText, setDraftText] = useState('');

  const setInputPath = useCallback(
    (inputPath: string) => {
      if (tab.inputPath !== inputPath) {
        dispatch({ type: 'UPDATE_INPUT', inputPath });
      }
    },
    [dispatch, tab.inputPath],
  );

  const generate = useCallback(async (path: string, strategy: MappingTableGenStrategy) => {
    setGenerating(true);
    try {
      const table = await generateMappingTable(path, strategy);
      setDraftText(mappingTableToString(table));
      setPreviewOpen(true);
    } catch (e) {
      NOTIFY.error(`Failed to generate mapping table: ${e instanceof Error ? e.message : e}`);
    } finally {
      setGenerating(false);
    }
  }, []);

  const handleApply = () => {
    dispatch({ type: 'UPDATE_TEXT', text: draftText });
    setPreviewOpen(false);
  };
  const handleMerge = () => {
    const { text, added } = appendMissingMappingTables(tab.text, draftText);

    dispatch({ type: 'UPDATE_TEXT', text });
    if (added.length > 0) {
      const MAX_PREVIEW = 8;
      const previewLines = added.slice(0, MAX_PREVIEW).map(({ priority, name }) => `- ${priority} ${name}`.trim());
      const more = added.length > MAX_PREVIEW ? `\n... (+${added.length - MAX_PREVIEW} more)` : '';

      NOTIFY.success(`Added ${added.length} new entries:\n${previewLines.join('\n')}${more}`);
    } else {
      NOTIFY.info('No new entries to add');
    }

    setPreviewOpen(false);
  };
  const handleCancel = () => {
    setPreviewOpen(false);
  };

  if (!tab) return null;

  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        gap: 1,
        width: '100%',
      }}
    >
      <OpenIcon path={tab.inputPath} />
      <TextField
        label={t('convert-form-dar-label')}
        size='small'
        variant='standard'
        sx={{ width: '60%' }}
        value={tab.inputPath}
        onChange={(e) =>
          dispatch({
            type: 'UPDATE_INPUT',
            inputPath: e.target.value,
          })
        }
      />

      <SelectPathButton
        sx={{ height: 48, display: 'flex', alignItems: 'center' }}
        path={tab.inputPath}
        isDir={true}
        setPath={setInputPath}
      />

      <StrategyList />

      {/* GENERATE */}
      <Button
        sx={{ p: '15px' }}
        size='small'
        variant='contained'
        disabled={generating || !tab.inputPath}
        onClick={() => generate(tab.inputPath, tab.tableGen.strategy)}
      >
        {t('generate-btn')}
      </Button>

      <MappingPreviewDialog
        open={previewOpen}
        text={draftText}
        onApply={handleApply}
        onAppend={handleMerge}
        onCancel={handleCancel}
      />
    </Box>
  );
};

const appendMissingMappingTables = (
  base: string,
  incoming: string,
): {
  text: string;
  added: { priority: string; name: string }[];
} => {
  const baseMap = mappingTableFromStr(base);
  const incomingMap = mappingTableFromStr(incoming);

  const added: { priority: string; name: string }[] = [];

  for (const [k, v] of Object.entries(incomingMap)) {
    if (k in baseMap) continue;

    baseMap[k] = v;
    added.push({ priority: k, name: v });
  }

  return {
    text: mappingTableToString(baseMap),
    added,
  };
};
