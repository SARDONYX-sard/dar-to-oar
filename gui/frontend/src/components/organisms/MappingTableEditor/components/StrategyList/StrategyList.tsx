import { type SelectChangeEvent, Tooltip } from '@mui/material';
import { useCallback } from 'react';
import { useEditorContext } from '../../context/editorContext';
import { useTranslation } from '@/components/hooks/useTranslation';
import { SelectWithLabel } from '@/components/molecules/SelectWithLabel';
import { strategySchema } from '@/services/api/mapping_table';

export const StrategyList = () => {
  const [state, dispatch] = useEditorContext();
  const { t } = useTranslation();
  const tab = state.tabs[state.active];

  const handleOnChange = useCallback(
    ({ target }: SelectChangeEvent) => {
      dispatch({
        type: 'UPDATE_TABLE_GEN_STRATEGY',
        strategy: strategySchema.safeParse(target.value).data ?? 'txt_stem',
      });
    },
    [dispatch],
  );

  const menuItems = [
    { value: 'txt_stem', label: t('strategy-txt-stem') },
    { value: 'txt_stem_stripped', label: t('strategy-txt-stem-stripped') },
    { value: 'dir_pattern', label: t('strategy-dir-pattern') },
  ] as const;

  return (
    <Tooltip placement='left-start' title={<TOOLTIP t={t} />}>
      <SelectWithLabel
        label={t('strategy-list-label')}
        menuItems={menuItems}
        onChange={handleOnChange}
        value={tab.tableGen.strategy}
      />
    </Tooltip>
  );
};

const TOOLTIP = ({ t }: { t: ReturnType<typeof useTranslation>['t'] }) => (
  <div style={{ maxWidth: 420, fontSize: 12, lineHeight: 1.4 }}>
    <p>Strategy defines how rename targets are extracted.</p>
    <br />

    <p>
      <b>{t('strategy-txt-stem')}</b>: {t('strategy-txt-stem-desc')}
    </p>

    <pre>{`
Input:
  DynamicAnimationReplacer
    └── _CustomConditions
          └── 666003
              ├── AELA ultimate sit.txt
              └── _conditions.txt (ignored)
`}</pre>

    <pre>{`
Output:
  666003 AELA ultimate sit
`}</pre>

    <hr />

    <p>
      <b>{t('strategy-txt-stem-stripped')}</b>: {t('strategy-txt-stem-stripped-desc')}
    </p>

    <pre>{`
Input:
  DynamicAnimationReplacer
    └── _CustomConditions
          └── 666003
              ├── foo123.txt
              └── _conditions.txt (ignored)
`}</pre>

    <pre>{`
Output:
  666003 foo
`}</pre>

    <hr />

    <p>
      <b>{t('strategy-dir-pattern')}</b>: {t('strategy-dir-pattern-desc')}
    </p>

    <pre>{`
Input:
  DynamicAnimationReplacer
    ├── _CustomConditions
    |     └── 666003 - Sit
    └── Skyrim.esm
          └── 1A2B - Attack
`}</pre>

    <pre>{`
Output:
  666003 Sit
  1A2B Attack
`}</pre>
  </div>
);
