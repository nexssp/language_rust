let languageConfig = Object.assign({}, require("./rust.win32.nexss.config"));

const os = require(`${process.env.NEXSS_SRC_PATH}/node_modules/@nexssp/os/`);
const sudo = os.sudo();

const distName = os.name();

languageConfig.dist = distName;

languageConfig.compilers = {
  rustNightly: {
    // install: `${sudo}snap install rustup --classic && ${sudo}rustup install stable && ${sudo}rustup default stable && ${sudo}cargo install cargo-script`,
    install: `${sudo}apt install -y curl cmake gcc
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
switch (distName) {
  case os.distros.UBUNTU:
    languageConfig.compilers.rustNightly.install = `${sudo}apt update -y
${sudo}apt-get install -y rustc cargo
${sudo}cargo install cargo-script`;
    break;
  case os.distros.ALPINE:
    languageConfig.compilers.rustNightly.install = `${sudo} apk update
${sudo}apk add rust cargo
${sudo}cargo install cargo-script`;
    break;
  case os.distros.SUSE_LEAP:
  case os.distros.SUSE_TUMBLEWEED:
    languageConfig.compilers.rustNightly.install = `${sudo}zypper update
${sudo}zypper --non-interactive install -t pattern devel_basis devel_C_C++
${sudo}zypper --non-interactive install curl tar gzip libopenssl1_0_0 libicu
${sudo}curl -L https://github.com/PowerShell/PowerShell/releases/download/v7.0.3/powershell-7.0.3-linux-x64.tar.gz -o /tmp/powershell.tar.gz
${sudo}mkdir -p /opt/microsoft/powershell/7
${sudo}tar zxf /tmp/powershell.tar.gz -C /opt/microsoft/powershell/7
${sudo}chmod +x /opt/microsoft/powershell/7/pwsh
${sudo}ln -s /opt/microsoft/powershell/7/pwsh /usr/bin/pwsh`;
    break;
  default:
    languageConfig.compilers.rustNightly.install = process.replacePMByDistro(
      languageConfig.compilers.rustNightly.install
    );
    break;
}

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
