import { get_parent } from '@/utils/path';

describe('get_parent function', () => {
  test('returns the same path if it ends with /', () => {
    const path = '/example/path/';
    expect(get_parent(path)).toBe(path);
  });

  test('returns the same path if it ends with \\', () => {
    const path = '\\example\\path\\';
    expect(get_parent(path)).toBe(path);
  });

  test('deletes tailing part until / if path does not end with /', () => {
    const path = '/example/path/file.txt';
    const expected = '/example/path';
    expect(get_parent(path)).toBe(expected);
  });

  test('deletes tailing part until \\ if path does not end with \\', () => {
    const path = '\\example\\path\\file.txt';
    const expected = '\\example\\path';
    expect(get_parent(path)).toBe(expected);
  });
});
