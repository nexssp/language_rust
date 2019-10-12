module.exports = {
  description: "Rust Language",
  type: "language",
  author: "Marcin Polak <mapoart@gmail.com>",
  version: "1.0",
  compiler: "cargo-script",
  extension: ".rs",
  executeCommandLine: "",
  InteractiveShell: "",
  errors: {
    "Uncaught Error: Class '(.*?)'": {
      win32: "curl https://sh.rustup.rs -sSf | sh",
      darwin: "curl https://sh.rustup.rs -sSf | sh <package>",
      linux: "nexss install ahk <package>"
    }
  },
  url: ""
};
