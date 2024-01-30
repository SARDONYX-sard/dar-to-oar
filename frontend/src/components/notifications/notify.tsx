import { enqueueSnackbar } from 'notistack';

import type { OptionsObject, SnackbarMessage, SnackbarOrigin } from 'notistack';

// Notify design is defined in provider
/**
 * Wrapper to simplify refactoring of libraries such as snackbar and toast
 */
export const notify = {
  info(message: SnackbarMessage, options?: OptionsObject<'info'>) {
    enqueueSnackbar(message, { variant: 'info', ...options });
  },
  success(message: SnackbarMessage, options?: OptionsObject<'success'>) {
    enqueueSnackbar(message, { variant: 'success', ...options });
  },
  warn(message: SnackbarMessage, options?: OptionsObject<'warning'>) {
    enqueueSnackbar(message, { variant: 'warning', ...options });
  },
  error(message: SnackbarMessage, options?: OptionsObject<'error'>) {
    enqueueSnackbar(message, { variant: 'error', ...options });
  },
};

export const getPosition = (): SnackbarOrigin => {
  const defaultPosition = {
    horizontal: 'right',
    vertical: 'bottom',
  } as const;
  const posJson = JSON.parse(localStorage.getItem('snackbar-position') ?? '{}') as Partial<SnackbarOrigin>;

  return {
    horizontal: posJson?.horizontal ?? defaultPosition.horizontal,
    vertical: posJson?.vertical ?? defaultPosition.vertical,
  };
};
