let languageConfig = Object.assign({}, require("./rust.win32.nexss.config"));

let sudo = "sudo";
if (process.getuid && process.getuid() === 0) {
  sudo = "";
}
languageConfig.compilers = {
  rustNightly: {
    install: `${sudo}snap install rustup --classic && ${sudo}rustup install stable && ${sudo}rustup default stable && ${sudo}cargo install cargo-script`,
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
