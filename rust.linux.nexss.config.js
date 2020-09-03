let languageConfig = Object.assign({}, require("./rust.win32.nexss.config"));

languageConfig.compilers = {
  rustNightly: {
    install: `apt update && apt install curl && curl https://sh.rustup.rs -sSf | bash && apt install build-essential`,
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
    init: () => {},
    // if command not found in specification
    // run directly on package manager
    else: "cargo",
  },
};

module.exports = languageConfig;
