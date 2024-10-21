import { describe, expect, it } from 'vitest';

import { parseDarPath } from './parseDarPath';

describe('parseDarPath', () => {
  it('should parse DAR path correctly when "meshes" is present', () => {
    const path = '../ModName/meshes';
    const result = parseDarPath(path);

    expect(result).toHaveProperty('oarRoot', '../ModName/meshes');
    expect(result).toHaveProperty('modName', 'ModName');
  });

  it('should return the full path as oarRoot when "meshes" is not present', () => {
    const path = '../OtherMod/otherDir';
    const result = parseDarPath(path);

    expect(result).toHaveProperty('oarRoot', '../OtherMod/otherDir');
    expect(result).toHaveProperty('modName', 'otherDir');
  });

  it('should handle paths with mixed slashes', () => {
    const path = '../ModName\\meshes';
    const result = parseDarPath(path);

    expect(result).toHaveProperty('oarRoot', '../ModName/meshes');
    expect(result).toHaveProperty('modName', 'ModName');
  });

  it('should return only ASCII parts as modName when no "meshes" is found', () => {
    const path = '../ModName/NonASCII_ディレクトリ_With_English';
    const result = parseDarPath(path);

    expect(result).toHaveProperty('oarRoot', '../ModName/NonASCII_ディレクトリ_With_English');
    expect(result).toHaveProperty('modName', 'NonASCII__With_English'); // ASCII parts should be extracted
  });

  it('should return modName as the last ASCII directory when no "meshes" is found and it is ASCII', () => {
    const path = '../ModName/lastDir';
    const result = parseDarPath(path);

    expect(result).toHaveProperty('oarRoot', '../ModName/lastDir');
    expect(result).toHaveProperty('modName', 'lastDir'); // ASCII should yield 'lastDir'
  });
});
