import { Button, type ButtonProps, Tooltip } from '@mui/material';

import type { ReactNode } from 'react';

type Props = {
  buttonName: ReactNode;
  tooltipTitle?: ReactNode;
} & ButtonProps;

export const ButtonWithToolTip = ({ buttonName, sx, tooltipTitle, ...props }: Props) => (
  <Tooltip placement='bottom' title={tooltipTitle}>
    <Button
      sx={{
        height: '60%',
        marginTop: '9px',
        width: '100%',
        ...sx,
      }}
      variant='outlined'
      {...props}
    >
      {buttonName}
    </Button>
  </Tooltip>
);
