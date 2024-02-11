import { InputLabel, FormControl } from '@mui/material';
import MenuItem from '@mui/material/MenuItem';
import Select, { type SelectChangeEvent } from '@mui/material/Select';
import { useCallback, useState } from 'react';

import { getPosition } from '@/components/notifications';
import { useTranslation } from '@/hooks';

import type { SnackbarOrigin } from 'notistack';

export const NoticePositionList = () => {
  const { t } = useTranslation();
  const [pos, setPos] = useState(getPosition);

  const handleChange = useCallback(
    (e: SelectChangeEvent<string>) => {
      const [vertical, horizontal] = e.target.value.split('_');

      const newPosition: SnackbarOrigin = {
        vertical: vertical === 'bottom' || vertical === 'top' ? vertical : 'bottom',
        horizontal: horizontal === 'center' || horizontal === 'right' || horizontal === 'left' ? horizontal : 'right',
      };

      localStorage.setItem(
        'snackbar-position',
        JSON.stringify({
          vertical: vertical,
          horizontal: horizontal,
        }),
      );
      setPos(newPosition);
    },
    [setPos],
  );

  return (
    <FormControl variant="filled" sx={{ m: 1, minWidth: 105 }}>
      <InputLabel id="notice-position-label">{t('notice-position-list-label')}</InputLabel>
      <Select
        id="notice-position"
        inputProps={{ MenuProps: { disableScrollLock: true } }}
        label="Editor Mode"
        labelId="notice-position-label"
        onChange={handleChange}
        value={`${pos.vertical}_${pos.horizontal}`}
      >
        <MenuItem value={'top_right'}>{t('notice-position-top-right')}</MenuItem>
        <MenuItem value={'top_center'}>{t('notice-position-top-center')}</MenuItem>
        <MenuItem value={'top_left'}>{t('notice-position-top-left')}</MenuItem>
        <MenuItem value={'bottom_right'}>{t('notice-position-bottom-right')}</MenuItem>
        <MenuItem value={'bottom_center'}>{t('notice-position-bottom-center')}</MenuItem>
        <MenuItem value={'bottom_left'}>{t('notice-position-bottom-left')}</MenuItem>
      </Select>
    </FormControl>
  );
};
