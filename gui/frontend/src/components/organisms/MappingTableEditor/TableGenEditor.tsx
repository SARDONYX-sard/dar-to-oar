import { Box } from '@mui/material';
import { DragOverlay } from './components/DragOverlay';
import { EditToolbar } from './components/EditToolbar';
import { EmptyDragPoint } from './components/EmptyDragPoint';
import { FileSettingsBar } from './components/FileSettingsBar';
import { MappingTableEditor } from './components/MappingTableEditor';
import { TopBar } from './components/TopBar';
import { useEditorContext } from './context/editorContext';
import { MappingTableEditorProvider } from './context/editorProvider';
import { useTauriDragDrop } from './hooks/useDrag';
import { useTabEditorCommands } from './hooks/useTabEditorCommands';

/** Public editor component */
export const TableGenEditor = () => {
  return (
    <MappingTableEditorProvider>
      <TableGenEditorInner />
    </MappingTableEditorProvider>
  );
};

/** Public editor component */
const TableGenEditorInner = () => {
  const { openFiles } = useTabEditorCommands();
  const { dragging } = useTauriDragDrop(openFiles);

  const [state, _] = useEditorContext();
  const tab = state.tabs.at(state.active) ?? state.tabs.at(0);

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 56px)' }}>
      <TopBar />

      <FileSettingsBar />

      {tab ? (
        <>
          <EditToolbar />
          <MappingTableEditor />
        </>
      ) : (
        <EmptyDragPoint />
      )}

      {dragging && <DragOverlay />}
    </Box>
  );
};
