import { Button } from '@mui/material';

import { useTranslation } from '@/components/hooks/useTranslation';
import { openUrl } from '@/services/api/shell';

import type { MouseEventHandler } from 'react';
export const MappingHelpButton = () => {
  const { t } = useTranslation();
  const href = `https://github.com/SARDONYX-sard/dar-to-oar/${t('mapping-wiki-url-leaf')}`;
  const handleMappingClick: MouseEventHandler<HTMLButtonElement> = (_e) => {
    openUrl(href);
  };

  return (
    <>
      {t('convert-form-mapping-helper')}
      <br />
      {t('convert-form-mapping-helper2')}
      <Button onClick={handleMappingClick} style={{ fontSize: 'small' }} type='button'>
        [{t('convert-form-mapping-help-link-name')}]
      </Button>
    </>
  );
};
