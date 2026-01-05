import type { Metadata } from 'next';
// NOTE: Since turbo_pack bundler still cannot read JSON with @ aliases, use relative paths.
import packageJson from '../../../../../package.json';

export const metadata: Metadata = {
  title: packageJson.name,
  description: packageJson.description,
};

export const HELP_INFO = {
  homepage: packageJson.homepage,
  version: packageJson.version,
};
