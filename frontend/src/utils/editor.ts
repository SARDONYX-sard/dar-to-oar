export type EditorMode = "default" | "vim" | undefined;

export function selectEditorMode(select: string): EditorMode {
  if (select === "vim") {
    return select;
  } else {
    return "default";
  }
}
