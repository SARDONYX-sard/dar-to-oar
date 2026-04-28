import React, { useReducer } from 'react';
import z from 'zod';
import { FileTabSchema } from '../types/FileTab';
import { EditorContext } from './editorContext';
import { editorReducer } from './editorReducer';
import { PRIVATE_CACHE_OBJ } from '@/lib/storage/cacheKeys';
import { schemaStorage } from '@/lib/storage/schemaStorage';

import type { EditorState } from './editorTypes';

/** Provides mapping table editor state backed by schemaStorage */
export const MappingTableEditorProvider: React.FC<React.PropsWithChildren> = ({ children }) => {
  const tabs = schemaStorage.get(PRIVATE_CACHE_OBJ.mappingTableFileTabs, z.array(FileTabSchema));
  const active = schemaStorage.get(PRIVATE_CACHE_OBJ.mappingTableActiveTab, z.number());

  const options = schemaStorage.get(
    PRIVATE_CACHE_OBJ.mappingTableOptions,
    z.object({
      completion: z.boolean(),
      diagnostics: z.boolean(),
      formatter: z.boolean(),
      semanticTokens: z.boolean(),
      hover: z.boolean(),
      inlayHints: z.boolean(),
      signatureHelp: z.boolean(),
    }),
  );

  const initState = {
    tabs: tabs ?? [],
    active: z.number().safeParse(active).data ?? 0,
    options: options ?? {
      completion: true,
      diagnostics: true,
      formatter: true,
      semanticTokens: true,
      hover: true,
      inlayHints: false,
      signatureHelp: false,
    },
  } as const satisfies EditorState;

  const [state, dispatch] = useReducer(editorReducer, initState);

  return <EditorContext.Provider value={[state, dispatch]}>{children}</EditorContext.Provider>;
};
