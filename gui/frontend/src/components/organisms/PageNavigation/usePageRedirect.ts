'use client';
import { useRouter, useRouterState } from '@tanstack/react-router';
import { useState, useEffect } from 'react';

export const usePageRouter = <T extends Readonly<[string, ...string[]]>>(validPaths: T) => {
  const router = useRouter();
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  });
  const [selectedIndex, setSelectedIndex] = useState(0);

  const normalizePath = (path: string): (typeof validPaths)[number] => {
    for (const name of validPaths) {
      if (name === '/') continue;
      if (path.endsWith(name) || path.endsWith(`${name}/`)) {
        return name;
      }
    }
    return '/';
  };

  const currentPath = normalizePath(pathname);

  useEffect(() => {
    const index = validPaths.indexOf(currentPath);
    setSelectedIndex(index >= 0 ? index : 0);
  }, [currentPath, validPaths]);

  const navigateTo = (index: number) => {
    const target = validPaths[index];
    if (!target) return;
    setSelectedIndex(index);
    router.navigate({ to: target }).catch((e) => console.error('Failed to navigate to target path', e));
  };

  return { selectedIndex, navigateTo };
};
