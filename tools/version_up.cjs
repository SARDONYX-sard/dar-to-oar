/** biome-ignore-all lint/nursery/noUnnecessaryConditions: To configurable  */
/** biome-ignore-all lint/nursery/noUnresolvedImports: To use Node.js */
//@ts-check

function usage() {
  return `
  Usage:
  node version_up.js <version_type>

  Version Types:
  1. major: Increment the major version.(e.g. 0.0.0 -> 1.0.0)
  2. minor: Increment the minor version.(e.g. 0.0.0 -> 0.1.0)
  3. patch: Increment the patch version.(e.g. 0.0.0 -> 0.0.1)

  Examples:
  node version_up.cjs minor
  node version_up.cjs 2
`;
}

// - Special flags configuration
const isDebug = false;
const defaultVersion = '2'; // 2: Bump up `minor` version is default
const useGpg = true; // Verified commit with GPG key.

// import modules
const fs = require('node:fs');
const path = require('node:path');
const { execSync } = require('node:child_process');
// paths
const basePath = path.resolve(__dirname, '..');
const packageJsonPath = path.join(basePath, 'package.json');
const cargoTomlPath = path.join(basePath, 'Cargo.toml');
const issueTemplatePath = path.join(basePath, '.github', 'ISSUE_TEMPLATE', 'bug-report.yaml');
// Constants by path
const packageJson = require(packageJsonPath);
const CURRENT_VERSION = packageJson.version;

if (isDebug) {
  console.log(packageJsonPath);
  console.log(cargoTomlPath);
  console.log(issueTemplatePath);
}
main();

function main() {
  const versionType = process.argv[2] || defaultVersion;
  const newVersion = updateVersion(CURRENT_VERSION, versionType);

  updatePackageJson(newVersion);
  updateCargoToml(newVersion);
  updateIssueTemplate(newVersion);
  gitCommitAndTag(CURRENT_VERSION, newVersion);

  console.log(`Updated version: ${CURRENT_VERSION} => ${newVersion}`);
}

/**
 * @param {string} currentVersion
 * @param {string} versionType
 * @returns {string} newVersion
 */
function updateVersion(currentVersion, versionType) {
  const [major, minor, patch] = currentVersion.split('.').map(Number);

  switch (versionType) {
    case 'major':
    case '1':
      return `${major + 1}.0.0`;
    case 'minor':
    case '2':
      return `${major}.${minor + 1}.0`;
    case 'patch':
    case '3':
      return `${major}.${minor}.${patch + 1}`;
    default:
      throw new Error(`Invalid version type. Please specify "major"(1), "minor"(2), or "patch"(3).\n${usage()}`);
  }
}

/**
 * @param {string} newVersion
 */
function updatePackageJson(newVersion) {
  const newPackageJson = {
    ...packageJson,
    version: newVersion,
  };
  fs.writeFileSync(packageJsonPath, `${JSON.stringify(newPackageJson, null, 2)}\n`);
}

/**
 * @param {string} newVersion
 */
function updateCargoToml(newVersion) {
  const workSpaceRegExp = /\[workspace\.package\]\nversion = "(.*)"/;
  let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
  cargoToml = cargoToml.replace(workSpaceRegExp, `[workspace.package]\nversion = "${newVersion}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);
}

/**
 * @param {string} newVersion
 */
function updateIssueTemplate(newVersion) {
  const issueRexExp = /options:\n((\s+- .*\n)+)/;
  let issueTemplate = fs.readFileSync(issueTemplatePath, 'utf8');
  const versionList = issueTemplate.match(issueRexExp)?.[1];
  if (!versionList) {
    throw new Error('Invalid version');
  }

  const versions = versionList.split('\n').map((v) => v.trim().slice(2));
  if (!versions.includes(newVersion)) {
    issueTemplate = issueTemplate.replace(issueRexExp, `options:\n$1        - ${newVersion}\n`);
    fs.writeFileSync(issueTemplatePath, issueTemplate);
  }
}

/**
 * Function to commit changes and create Git tag
 * @param {string} currentVersion
 * @param {string} newVersion New version
 */
function gitCommitAndTag(currentVersion, newVersion) {
  let tagFlags = '';
  let commitFlags = '';
  if (useGpg) {
    tagFlags += '-s ';
    commitFlags += '-S ';
  }

  try {
    // Commit changes to Git
    execSync(
      `git add . && git commit ${commitFlags} -m "build(version): bump ${packageJson.name} from ${currentVersion} to ${newVersion}"`,
    );

    // Create Git tag
    execSync(`git tag ${newVersion} ${tagFlags} -m "Version ${newVersion}"`);

    console.log('Git commit and tag created successfully.');
  } catch (error) {
    throw new Error(`Failed to create Git commit and tag: ${error}`);
  }
}
