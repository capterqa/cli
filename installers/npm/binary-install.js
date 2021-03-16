const { existsSync, mkdirSync, createWriteStream } = require('fs');
const { join } = require('path');
const { spawnSync } = require('child_process');
const axios = require('axios');
const rimraf = require('rimraf');

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

class Binary {
  constructor(name, repo, version, target) {
    this.name = name;
    this.repo = repo;
    this.version = version;
    this.target = target;
    this.installDirectory = join(__dirname, 'bin');

    if (!existsSync(this.installDirectory)) {
      mkdirSync(this.installDirectory, { recursive: true });
    }

    this.binaryPath = join(this.installDirectory, this.name);
  }

  install() {
    if (existsSync(this.installDirectory)) {
      rimraf.sync(this.installDirectory);
    }

    mkdirSync(this.installDirectory, { recursive: true });

    console.log(`downloading binary from ${this.repo}...`);

    return axios({
      url: `https://api.github.com/repos/${this.repo}/releases`,
      headers: {
        Accept: 'application/vnd.github.v3.raw',
      },
    })
      .then((res) => {
        var release = res.data[0];
        var asset = release.assets.find((r) => {
          return r.name.includes(this.target);
        });
        var assetId = asset.id;
        return axios({
          url: `https://api.github.com/repos/${this.repo}/releases/assets/${assetId}`,
          responseType: 'stream',
          headers: {
            Accept: 'application/octet-stream',
          },
        })
          .then((res) => {
            res.data.pipe(
              createWriteStream(`${this.installDirectory}/${this.name}`, {
                mode: 0o755,
              })
            );
          })
          .then(() => {
            console.log(`${this.name} has been installed!`);
          })
          .catch((e) => {
            error(`error fetching release: ${e.message}`);
          });
      })
      .catch((e) => {
        error(`error fetching release: ${e.message}`);
      });
  }

  run() {
    if (!existsSync(this.binaryPath)) {
      error(`you must install ${this.name} before you can run it`);
    }

    const [, , ...args] = process.argv;

    const options = { cwd: process.cwd(), stdio: 'inherit' };

    const result = spawnSync(this.binaryPath, args, options);

    if (result.error) {
      error(result.error);
    }

    process.exit(result.status);
  }
}

module.exports.Binary = Binary;
