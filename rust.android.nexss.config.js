let languageConfig = Object.assign({}, require("./rust.win32.nexss.config"));

const sudo = process.sudo;
const distName = process.distro;

languageConfig.dist = distName;

languageConfig.compilers = {
  python3: {
    install: `pkg install -y binutils rust
cargo install cargo-script`,
    command: "cargo",
    args: "script <file> --",
    help: ``,
  },
};

languageConfig.languagePackageManagers = {
  cargo: {
    installation: "installed.",
    messageAfterInstallation: "",
    installed: "cargo list",
    search: "cargo search",
    install: "cargo install",
    uninstall: "cargo rm",
    help: "cargo",
    version: "cargo --version",
    init: () => { },
    // if command not found in specification
    // run directly on package manager
    else: "cargo",
  },
};

module.exports = languageConfig;
