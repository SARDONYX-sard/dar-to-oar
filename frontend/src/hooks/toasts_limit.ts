import { useEffect } from 'react';
import { toast, useToasterStore } from 'react-hot-toast';

/**
 * Avoid endless toast notices.
 * @param limit
 */
export function useToastLimit(limit: number) {
  const { toasts } = useToasterStore();
  useEffect(() => {
    toasts
      .filter((t) => t.visible)
      .filter((_, i) => i >= limit)
      .forEach((t) => toast.dismiss(t.id));
  }, [limit, toasts]);
}
