'use client';

import { useRouter, useRouterState } from '@tanstack/react-router';
import { useEffect } from 'react';
import { z } from 'zod';
import { PUB_CACHE_OBJ } from '@/lib/storage/cacheKeys';
import { schemaStorage } from '@/lib/storage/schemaStorage';

/**
 * usePageRedirect (TanStack Router)
 *
 * Handles:
 * - One-time redirect to lastPath
 * - Keeping lastPath up to date when user navigates
 * - Returning current selected index for UI
 */
export const usePageRedirect = <T extends Readonly<[string, ...string[]]>>(validPaths: T) => {
  const router = useRouter();
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  const pathSchema = z.enum(validPaths);
  const [lastPath, setLastPath] = schemaStorage.use(PUB_CACHE_OBJ.lastPath, pathSchema);

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
  const selectedIndex = Math.max(0, validPaths.indexOf(currentPath));

  // --- Redirect once per session ---
  useEffect(() => {
    if (!lastPath) return;

    const hasRedirected = sessionStorage.getItem('hasRedirected');

    if (hasRedirected) {
      return;
    }

    // Mark as redirected to prevent future redirects in the same session
    sessionStorage.setItem('hasRedirected', 'true');

    if (lastPath === '/' || pathname.endsWith(lastPath)) {
      return; // Already on the correct page, no need to redirect
    }

    router
      .navigate({
        replace: true,
        href: lastPath,
      })
      .catch((e) => console.error('Failed to redirect to last path', e));
  }, [lastPath, pathname, router]);

  // --- Keep lastPath up to date ---
  useEffect(() => {
    setLastPath(currentPath);
  }, [currentPath, setLastPath]);

  const navigateTo = (index: number) => {
    const target = validPaths[index];
    if (!target) return;

    router.navigate({ to: target }).catch((e) => console.error('Failed to navigate to target path', e));
  };

  return { selectedIndex, navigateTo };
};
