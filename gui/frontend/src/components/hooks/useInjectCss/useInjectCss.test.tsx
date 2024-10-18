import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { CSS } from '@/lib/css';

import { useInjectCss } from './useInjectCss';

// Mock the external dependencies used in the hook
vi.mock('@/lib/css', () => ({
  // biome-ignore lint/style/useNamingConvention: <explanation>
  CSS_PRESETS: {
    get: vi.fn(() => 'default'),
    getPreset: vi.fn(() => 'body { background: black; }'),
    setPreset: vi.fn(),
  },
}));

describe('useInjectCss hook', () => {
  beforeEach(() => {
    document.head.innerHTML = ''; // Clear document.head before each test
  });

  it('should inject CSS into the document on mount', () => {
    const { result } = renderHook(() => useInjectCss());

    // Expect the style element to be added to the document head
    const styleElement = document.getElementById('user-custom-css');
    expect(styleElement).not.toBeNull();
    expect(styleElement?.innerHTML).toBe('body { background: black; }');

    // Check initial state of the hook
    expect(result.current.preset).toBe('default');
    expect(result.current.css).toBe('body { background: black; }');
  });

  it('should update CSS when setCss is called', () => {
    const { result } = renderHook(() => useInjectCss());

    // Change the CSS via the setCss function
    act(() => {
      result.current.setCss('body { color: red; }');
    });

    const styleElement = document.getElementById('user-custom-css');
    expect(styleElement?.innerHTML).toBe('body { color: red; }');
  });

  it('should update preset when setPreset is called', () => {
    const { result } = renderHook(() => useInjectCss());

    // Change the preset via the setPreset function
    act(() => {
      result.current.setPreset('1');
    });

    expect(CSS.setPreset).toHaveBeenCalledWith('1');
    expect(result.current.preset).toBe('1');
  });

  it('should remove the style element on unmount', () => {
    const { unmount } = renderHook(() => useInjectCss());

    // Ensure style element exists initially
    const styleElement = document.getElementById('user-custom-css');
    expect(styleElement).not.toBeNull();

    // Unmount the hook and check that the style element is removed
    unmount();
    expect(document.getElementById('user-custom-css')).toBeNull();
  });
});
