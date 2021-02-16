// based on https://github.com/EverlastingBugstopper/binary-install/tree/main/packages/binary-install-example
const { Binary } = require('./binary-install');
const os = require('os');
const cTable = require('console.table');

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

const { version, repository } = require('./package.json');
let name = 'capter';

const supportedPlatforms = [
  {
    TYPE: 'Windows_NT',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-pc-windows-msvc',
    BINARY_NAME: `${name}.exe`,
  },
  {
    TYPE: 'Linux',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-unknown-linux-musl',
    BINARY_NAME: name,
  },
  {
    TYPE: 'Darwin',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-apple-darwin',
    BINARY_NAME: name,
  },
];

const getPlatformMetadata = () => {
  const type = os.type();
  const architecture = os.arch();

  for (let index in supportedPlatforms) {
    let supportedPlatform = supportedPlatforms[index];
    if (
      type === supportedPlatform.TYPE &&
      architecture === supportedPlatform.ARCHITECTURE
    ) {
      return supportedPlatform;
    }
  }

  error(
    `Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
      supportedPlatforms
    )}`
  );
};

const getBinary = () => {
  const platformMetadata = getPlatformMetadata();
  return new Binary(
    platformMetadata.BINARY_NAME,
    'capterqa/cli',
    version,
    platformMetadata.RUST_TARGET
  );
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

module.exports = {
  install,
  run,
};
