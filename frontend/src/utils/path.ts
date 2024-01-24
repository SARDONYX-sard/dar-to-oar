export function get_parent(path: string): string {
  if (path.endsWith('/') || path.endsWith('\\')) {
    return path;
  } else {
    // Deletes tailing part until / if path does not end with / or \
    return path.replace(/[/\\][^/\\]*$/, '');
  }
}
