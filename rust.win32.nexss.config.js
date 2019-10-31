let languageConfig = Object.assign({}, require("../config.win32"));
languageConfig.title = "Rust";
languageConfig.description =
  "A language empowering everyone to build reliable and efficient software.";
languageConfig.url = "https://www.rust-lang.org/";
languageConfig.extensions = [".rs"];
languageConfig.builders = {};
languageConfig.compilers = {
  rustNightly: {
    install:
      "scoop install rustup & rustup toolchain install nightly & rustup update & cargo install cargo-script",
    // Cpp does not have possibility to compile and run on the fly. We need to save it as a exe file first.
    command: "cargo script",
    args: "<file>",
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
