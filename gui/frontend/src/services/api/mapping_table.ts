import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';

export type MappingTable = Record<string, string>;

export const strategySchema = z.enum(['txt_stem', 'txt_stem_stripped', 'dir_pattern']);
export type MappingTableGenStrategy = z.infer<typeof strategySchema>;

/**
 *
 * @param path
 * @param strategy
 * @returns
 *
 * @throws If the mapping table generation fails, an error is thrown with a message describing the failure.
 */
export async function generateMappingTable(path: string, strategy: MappingTableGenStrategy): Promise<MappingTable> {
  return await invoke<MappingTable>('generate_mapping_table', { path, strategy });
}

export function mappingTableToString(table: MappingTable): string {
  return (
    Object.entries(table)
      // sort by priority (key) and format as "priority rename_to"
      .sort(([a], [b]) => Number(a) - Number(b))
      .map(([priority, rename_to]) => `${priority} ${rename_to}`)
      .join('\n')
  );
}

export const mappingTableFromStr = (text: string): MappingTable => {
  const map: MappingTable = {};

  for (const line of text.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    const [priority, ...rest] = trimmed.split(/\s+/);
    if (!priority) continue;

    const name = rest.join(' ').trim();
    map[priority] = name;
  }

  return map;
};

/**
 *
 * @param path The path to the mapping table file to read.
 * @returns
 *
 * @throws If the mapping table cannot be read, an error is thrown with a message describing the failure.
 */
export async function readMappingTable(path: string): Promise<MappingTable> {
  return await invoke<MappingTable>('read_mapping_table', { path });
}
