// NOTE: Since turbo_pack bundler still cannot read JSON with @ aliases, use relative paths.
import packageJson from '../../../../../package.json';

export const metadata = {
  title: packageJson.name,
  description: packageJson.description,
} as const;

export const HELP_INFO = {
  homepage: packageJson.homepage,
  version: packageJson.version,
} as const;
