export type EditorMode = "vim" | undefined;

export function selectEditorMode(select: string): EditorMode {
  if (select === "vim") {
    return select;
  }
}
