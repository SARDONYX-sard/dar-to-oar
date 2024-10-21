// The information necessary for the conversion
export type ParsedPath = {
  oarRoot: string;
  modName?: string;
};

const PATH_SEP = /\\|\//g;
// biome-ignore lint/suspicious/noControlCharactersInRegex: <explanation>
const ASCII0 = /^[\x00-\x7F]*$/;
// biome-ignore lint/suspicious/noControlCharactersInRegex: <explanation>
const NON_ASCII = /[^\x00-\x7F]+/;

// Check if a string is ASCII
function isAscii(str: string): boolean {
  return ASCII0.test(str);
}

// Extract ASCII parts from a string
function extractAsciiParts(str: string): string {
  return str
    .split(NON_ASCII) // Split by non-alphabetical characters
    .filter(isAscii)
    .join('');
}

// Function to parse the DAR path
export function parseDarPath(path: string): ParsedPath {
  const paths = path.split(PATH_SEP).filter(Boolean);

  const meshIndex = paths.findIndex((part) => part.toLowerCase() === 'meshes');

  // Correctly derive oarRoot based on the meshIndex
  const oarRoot = meshIndex !== -1 ? paths.slice(0, meshIndex + 1).join('/') : path;

  // Determine modName based on meshIndex
  let modName: string | undefined;
  if (meshIndex === -1) {
    const lastPart = paths.at(-1); // Using at method
    if (lastPart) {
      modName = extractAsciiParts(lastPart); // Extract ASCII parts from last directory
    }
  } else {
    modName = extractAsciiParts(paths.at(meshIndex - 1) || ''); // Extract ASCII parts from the preceding directory
  }

  return {
    oarRoot,
    modName,
  };
}
