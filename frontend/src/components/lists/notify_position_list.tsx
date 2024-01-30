import { InputLabel, FormControl } from '@mui/material';
import MenuItem from '@mui/material/MenuItem';
import Select, { type SelectChangeEvent } from '@mui/material/Select';
import { useCallback, useState } from 'react';

import { getPosition } from '@/components/notifications';
import { useTranslation } from '@/hooks';

import type { SnackbarOrigin } from 'notistack';

export const NotifyPositionList = () => {
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
      <InputLabel htmlFor="editor-select">{t('notice-location-list-label')}</InputLabel>
      <Select
        name={'notify-position-list'}
        onChange={handleChange}
        label="Editor Mode"
        labelId="editor-select-label"
        id="editor-select"
        value={`${pos.vertical}_${pos.horizontal}`}
        // NOTE: Without this, padding will be added to the body during popup in consideration of nest,
        // and the design will be broken.
        inputProps={{ MenuProps: { disableScrollLock: true } }}
      >
        <MenuItem value={'top_right'}>{t('notice-location-top-right')}</MenuItem>
        <MenuItem value={'top_center'}>{t('notice-location-top-center')}</MenuItem>
        <MenuItem value={'top_left'}>{t('notice-location-top-left')}</MenuItem>
        <MenuItem value={'bottom_right'}>{t('notice-location-bottom-right')}</MenuItem>
        <MenuItem value={'bottom_center'}>{t('notice-location-bottom-center')}</MenuItem>
        <MenuItem value={'bottom_left'}>{t('notice-location-bottom-left')}</MenuItem>
      </Select>
    </FormControl>
  );
};
