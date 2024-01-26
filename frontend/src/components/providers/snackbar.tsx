'use client';

import CloseIcon from '@mui/icons-material/Close';
import { Alert, AlertTitle, IconButton } from '@mui/material';
import {
  SnackbarProvider as SnackbarProviderInner,
  closeSnackbar,
  type CustomContentProps,
  type SnackbarKey,
  type SnackbarOrigin,
} from 'notistack';
import { forwardRef, memo } from 'react';

const getPosition = (): SnackbarOrigin => {
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

/**
 * ref
 * - https://github.com/iamhosseindhv/notistack/issues/477#issuecomment-1885706867
 * @export
 */
export default function SnackBarProvider() {
  return (
    <SnackbarProviderInner
      action={action}
      anchorOrigin={getPosition()}
      Components={{
        info: ThemeResponsiveSnackbar,
        success: ThemeResponsiveSnackbar,
        error: ThemeResponsiveSnackbar,
        warning: ThemeResponsiveSnackbar,
      }}
      dense
      maxSnack={3}
      preventDuplicate={true}
    />
  );
}

/** It exists to realize the deletion of the history of the passage at any timing by Click. */
const action = (id: SnackbarKey) => (
  <IconButton aria-label="close" color="inherit" size="small" onClick={() => closeSnackbar(id)}>
    <CloseIcon fontSize="inherit" />
  </IconButton>
);

const ThemeResponsiveSnackbarComp = forwardRef<HTMLDivElement, CustomContentProps>((props, forwardedRef) => {
  const {
    id,
    message,
    action: componentOrFnAction,
    variant: notistackVariant,
    hideIconVariant,
    style,
    className,
  } = props;

  const severity = notistackVariant === 'default' ? 'info' : notistackVariant;
  const action = typeof componentOrFnAction === 'function' ? componentOrFnAction(id) : componentOrFnAction;

  return (
    <Alert
      ref={forwardedRef}
      severity={severity}
      icon={hideIconVariant ? false : undefined}
      action={action}
      style={style}
      className={className}
      variant="outlined"
      sx={(theme) => ({
        alignItems: 'center',
        backgroundColor: '#1a1919e1',
        borderRadius: '8px',
        boxShadow: theme.shadows[8],
        display: 'flex',
        maxWidth: '35vw',
        willChange: 'transform',
      })}
    >
      <AlertTitle sx={{ color: '#fff' }}>{severity.toUpperCase()}</AlertTitle>
      {message}
    </Alert>
  );
});
ThemeResponsiveSnackbarComp.displayName = 'ThemeResponsiveSnackbarCompRef';
const ThemeResponsiveSnackbar = memo(ThemeResponsiveSnackbarComp);
