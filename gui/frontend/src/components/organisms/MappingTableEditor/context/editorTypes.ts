import { MappingTableGenStrategy } from '../../../../services/api/mapping_table';

import type { MappingTableLspOptions } from '../../MonacoEditor/support_mapping_table';
import type { FileTab } from '../types/FileTab';

/** Global editor state */
export type EditorState = {
  tabs: FileTab[];
  active: number;
  options: MappingTableLspOptions;
};

/** Editor reducer actions */
export type EditorAction =
  | { type: 'NEW_TAB' }
  | { type: 'OPEN_TABS'; tabs: FileTab[] }
  | { type: 'SET_ACTIVE'; index: number }
  | { type: 'CLOSE_TAB'; index: number }
  | { type: 'REVERT_ACTIVE_TAB' }
  | { type: 'UPDATE_TEXT'; text: string }
  | { type: 'UPDATE_CURSOR'; cursorPos: FileTab['cursorPos'] }
  | { type: 'UPDATE_INPUT'; inputPath: string }
  | { type: 'UPDATE_OUTPUT'; outputPath: string }
  | { type: 'MARK_SAVED'; index: number }
  | { type: 'SET_MAPPING_OPTIONS'; options: Partial<MappingTableLspOptions> }
  | { type: 'UPDATE_TABLE_GEN_STRATEGY'; strategy: MappingTableGenStrategy };
