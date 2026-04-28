import { OnChange, OnMount } from '@monaco-editor/react';
import { Typography } from '@mui/material';
import { editor } from 'monaco-editor';
import { useEffect, useRef } from 'react';
import { useEditorModeContext } from '../../../providers/EditorModeProvider';
import { supportMappingTable } from '../../MonacoEditor/support_mapping_table';
import { useEditorContext } from '../context/editorContext';
import { useTranslation } from '@/components/hooks/useTranslation';
import { MonacoEditor } from '@/components/organisms/MonacoEditor';

import type monaco from 'monaco-editor';

/** editor pane */
export const MappingTableEditor = ({ onMount }: { onMount?: OnMount }) => {
  const [state, dispatch] = useEditorContext();
  const { t } = useTranslation();
  const isVimMode = useEditorModeContext().editorMode === 'vim';
  const tab = state.tabs[state.active];
  const editorRef = useRef<MonacoEditor | null>(null);
  const monacoRef = useRef<typeof monaco | null>(null);
  const disposablesRef = useRef<monaco.IDisposable[]>([]);

  // NOTE: Prevent duplicate registrations when switching editor tabs
  useEffect(() => {
    if (!editorRef.current || !monacoRef.current) return;

    disposablesRef.current.forEach((d) => d.dispose());
    disposablesRef.current = [];
    disposablesRef.current = supportMappingTable(state.options)(editorRef.current, monacoRef.current);
  }, [state.options]);

  // NOTE: Prevent duplicate registrations when navigating away and returning to the page
  useEffect(() => {
    return () => {
      if (disposablesRef.current) {
        disposablesRef.current.forEach((d) => d.dispose());
        disposablesRef.current = [];
      }
    };
  }, []);

  const handleOnChange: OnChange = (text) => {
    if (text) {
      dispatch({ type: 'UPDATE_TEXT', text });
    }
  };

  const handleOnMount: OnMount = (editor, monaco) => {
    onMount?.(editor, monaco);
    editorRef.current = editor;
    monacoRef.current = monaco;

    // NOTE: To avoid duplicate registration of language features, dispose existing ones before registering new ones.
    if (disposablesRef.current) {
      disposablesRef.current.forEach((d) => d.dispose());
      disposablesRef.current = [];
    }
    disposablesRef.current = supportMappingTable(state.options)(editor, monaco);

    // Restore cursorPos in FileTab
    if (tab.cursorPos) {
      editor.setPosition(tab.cursorPos);
      editor.revealPositionInCenter(tab.cursorPos);
      editor.focus();
    }

    // Save position
    editor.onDidChangeCursorPosition(() => {
      const pos = editor.getPosition();
      if (pos) {
        dispatch({
          type: 'UPDATE_CURSOR',
          cursorPos: pos,
        });
      }
    });
  };

  return (
    <>
      <Typography variant='subtitle2' sx={{ px: 2, pt: 1, color: '#aaa' }}>
        {t('mapping-table-label')}
      </Typography>
      <MonacoEditor
        key={tab.id + tab.dirty}
        defaultLanguage='mapping_table' // link @/components/organisms/MonacoEditor/support_mapping_table/index.ts
        height='70%'
        value={tab.text}
        vimMode={isVimMode}
        options={MONACO_OPTIONS}
        onChange={handleOnChange}
        onMount={handleOnMount}
      />
    </>
  );
};

export const MONACO_OPTIONS = {
  fontSize: 13,
  minimap: { enabled: true },
  renderWhitespace: 'boundary',
  bracketPairColorization: {
    enabled: true,
  },
} as const satisfies editor.IStandaloneEditorConstructionOptions;
