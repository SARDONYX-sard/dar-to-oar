import { FormControl, Tooltip, InputLabel } from '@mui/material';
import MenuItem from '@mui/material/MenuItem';
import Select, { type SelectChangeEvent } from '@mui/material/Select';
import { changeLanguage } from 'i18next';

import { useStorageState } from '@/hooks';

export const TranslationList = () => {
  const [lang, setLang] = useStorageState('locale', 'auto');

  const locale = 'Locale';
  const handleChange = (e: SelectChangeEvent<string>) => {
    const newLocale = e.target.value;
    setLang(newLocale);
    changeLanguage(newLocale === 'auto' ? window.navigator.language : newLocale);
  };

  return (
    <Tooltip title="Select Language" placement="top">
      <FormControl variant="filled" sx={{ m: 1, minWidth: 135 }}>
        <InputLabel htmlFor="style-select">Language</InputLabel>
        <Select
          id="locale-select"
          label={locale}
          labelId="locale-select-label"
          name={locale}
          onChange={handleChange}
          value={lang}
        >
          <MenuItem value={'auto'}>Auto</MenuItem>
          <MenuItem value={'en-US'}>English(US)</MenuItem>
          <MenuItem value={'ja-JP'}>Japanese</MenuItem>
        </Select>
      </FormControl>
    </Tooltip>
  );
};
