import { FormControl, Tooltip, InputLabel } from '@mui/material';
import MenuItem from '@mui/material/MenuItem';
import Select, { type SelectChangeEvent } from '@mui/material/Select';

import { useTranslation } from '@/hooks';
import { selectPreset, presetStyles } from '@/utils/styles';

type Props = {
  setStyle: (value: string) => void;
  setPreset: (value: string) => void;
  preset: string;
};

export const StyleList = ({ preset, setPreset, setStyle }: Props) => {
  const { t } = useTranslation();
  const handleChange = (e: SelectChangeEvent<string>) => {
    const presetNumber = selectPreset(e.target.value);
    setPreset(presetNumber);
    if (presetNumber === '0') {
      setStyle(localStorage.getItem('customCSS') ?? '');
      return;
    }
    setStyle(presetStyles[presetNumber]);
  };

  return (
    <Tooltip
      title={
        <>
          <p>{t('css-preset-list-tooltip')}</p>
          <p>{t('css-preset-list-tooltip2')}</p>
        </>
      }
      placement="top"
    >
      <FormControl variant="filled" sx={{ m: 1, minWidth: 110 }}>
        <InputLabel htmlFor="style-select">{t('css-preset-list-label')}</InputLabel>
        <Select
          name={preset}
          onChange={handleChange}
          label="CSS Presets"
          labelId="style-select-label"
          id="style-select"
          value={preset}
        >
          <MenuItem value={'0'}>{t('css-preset-list-item0')}</MenuItem>
          <MenuItem value={'1'}>{t('css-preset-list-item1')}</MenuItem>
          <MenuItem value={'2'}>{t('css-preset-list-item2')}</MenuItem>
          <MenuItem value={'3'}>{t('css-preset-list-item3')}</MenuItem>
        </Select>
      </FormControl>
    </Tooltip>
  );
};
