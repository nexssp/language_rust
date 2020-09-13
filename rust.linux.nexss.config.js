let languageConfig = Object.assign({}, require("./rust.win32.nexss.config"));

let sudo = "sudo ";
if (process.getuid && process.getuid() === 0) {
  sudo = "";
}
languageConfig.compilers = {
  rustNightly: {
    // install: `${sudo}snap install rustup --classic && ${sudo}rustup install stable && ${sudo}rustup default stable && ${sudo}cargo install cargo-script`,
    install: `${sudo}apt install -y curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
grep -qxF 'export PATH="$HOME/.cargo/bin:$PATH"' ~/.bashrc || echo 'export PATH="$HOME/.cargo/bin/:$PATH"' > ~/.bashrc
chmod +x $HOME/.cargo/env
. $HOME/.cargo/env
cargo install cargo-script`,
    command: "cargo",
    args: "script <file> --",
    help: ``,
  },
};

const {
  replaceCommandByDist,
  dist,
} = require(`${process.env.NEXSS_SRC_PATH}/lib/osys`);

const distName = dist();
languageConfig.dist = distName;

languageConfig.compilers.rustNightly.install = replaceCommandByDist(
  languageConfig.compilers.rustNightly.install
);

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
