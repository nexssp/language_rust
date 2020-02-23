let languageConfig = Object.assign({}, require("../config.win32"));
languageConfig.title = "Rust";
languageConfig.description =
  "A language empowering everyone to build reliable and efficient software.";
languageConfig.url = "https://www.rust-lang.org/";
languageConfig.founders = ["Graydon Hoare"];
languageConfig.developers = ["Mozilla"];
languageConfig.years = ["2010"];
languageConfig.extensions = [".rs", ".rlib"];
languageConfig.builders = {};
languageConfig.compilers = {
  rustNightly: {
    install: `Powershell -ExecutionPolicy Bypass -noexit -File ${__dirname}\\install\\installRustup.ps1`,
    command: "cargo",
    args: "script <file> --",
    help: ``
  }
};
languageConfig.errors = require("./nexss.rust.errors");
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
    else: "cargo"
  }
};

module.exports = languageConfig;
