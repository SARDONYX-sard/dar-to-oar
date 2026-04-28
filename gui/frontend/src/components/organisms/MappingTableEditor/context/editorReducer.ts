import { FileTab } from '../types/FileTab';
import { STORAGE } from '@/lib/storage';
import { PRIVATE_CACHE_OBJ } from '@/lib/storage/cacheKeys';

import type { EditorAction, EditorState } from './editorTypes';

/** Reducer handling editor state transitions */
export const editorReducer = (state: EditorState, action: EditorAction): EditorState => {
  switch (action.type) {
    case 'NEW_TAB': {
      const newTab: FileTab = {
        id: crypto.randomUUID(),
        text: '',
        inputPath: '',
        originalText: '',
        dirty: false,
        cursorPos: { lineNumber: 1, column: 1 },
        outputPath: './mapping_tables/new_mapping_table.txt',
        tableGen: { strategy: 'txt_stem_stripped' },
      };

      const tabs = [...state.tabs, newTab];
      const newTabIndex = tabs.length - 1;

      cacheFileTabs(tabs);
      cacheActiveTab(newTabIndex);

      return {
        ...state,
        tabs,
        active: newTabIndex,
      };
    }
    case 'OPEN_TABS': {
      const existingIds = new Set(state.tabs.map((t) => t.id));

      const newTabs: FileTab[] = [];
      const skippedIds: string[] = [];

      for (const tab of action.tabs) {
        if (existingIds.has(tab.id)) {
          skippedIds.push(tab.id);
          continue;
        }
        newTabs.push(tab);
      }

      if (skippedIds.length > 0) {
        console.info('[Editor] skipped tabs due to duplicate id:', skippedIds);
      }

      if (newTabs.length === 0) {
        return state;
      }

      const tabs = [...state.tabs, ...newTabs];

      cacheFileTabs(tabs);

      return {
        ...state,
        tabs,
        active: state.tabs.length,
      };
    }

    case 'SET_ACTIVE':
      cacheActiveTab(action.index);
      return { ...state, active: action.index };

    case 'CLOSE_TAB': {
      cacheActiveTab(action.index);

      const tabs = state.tabs.filter((_, i) => i !== action.index);
      cacheFileTabs(tabs);

      const active = Math.max(0, Math.min(state.active, tabs.length - 1));
      return { ...state, tabs, active };
    }

    case 'REVERT_ACTIVE_TAB': {
      const tabs = [...state.tabs];

      const activeTab = tabs[state.active];
      tabs[state.active] = {
        ...activeTab,
        text: activeTab.originalText,
        dirty: false,
      };
      cacheFileTabs(tabs);

      return { ...state, tabs };
    }

    case 'UPDATE_TEXT': {
      const tabs = [...state.tabs];
      tabs[state.active] = { ...tabs[state.active], text: action.text, dirty: true };

      cacheFileTabs(tabs);

      return { ...state, tabs };
    }

    case 'UPDATE_CURSOR': {
      const tabs = [...state.tabs];
      tabs[state.active] = { ...tabs[state.active], cursorPos: action.cursorPos };

      cacheFileTabs(tabs);
      return { ...state, tabs };
    }

    case 'UPDATE_INPUT': {
      const tabs = [...state.tabs];
      tabs[state.active] = { ...tabs[state.active], inputPath: action.inputPath };

      cacheFileTabs(tabs);
      return { ...state, tabs };
    }

    case 'UPDATE_OUTPUT': {
      const tabs = [...state.tabs];
      tabs[state.active] = { ...tabs[state.active], outputPath: action.outputPath };

      cacheFileTabs(tabs);
      return { ...state, tabs };
    }

    /** From rust */
    case 'MARK_SAVED': {
      const tabs = [...state.tabs];
      tabs[action.index] = { ...tabs[action.index], dirty: false };

      cacheFileTabs(tabs);
      return { ...state, tabs };
    }

    case 'SET_MAPPING_OPTIONS': {
      const options = {
        ...state.options,
        ...action.options,
      };
      STORAGE.set(PRIVATE_CACHE_OBJ.mappingTableOptions, JSON.stringify(options));
      return { ...state, options };
    }

    case 'UPDATE_TABLE_GEN_STRATEGY': {
      const tabs = [...state.tabs];
      tabs[state.active] = { ...tabs[state.active], tableGen: { strategy: action.strategy } };

      cacheFileTabs(tabs);
      return { ...state, tabs };
    }

    default:
      return state;
  }
};

function cacheFileTabs(tabs: FileTab[]) {
  STORAGE.set(PRIVATE_CACHE_OBJ.mappingTableFileTabs, JSON.stringify(tabs));
}

function cacheActiveTab(index: number) {
  STORAGE.set(PRIVATE_CACHE_OBJ.mappingTableActiveTab, JSON.stringify(index));
}
