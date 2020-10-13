// more about errors: https://github.com/nexssp/cli/wiki/Errors-Solutions
module.exports = {
  "no such subcommand": `nexss rs install cargo-script`,
  "no default toolchain":
    "No default toolchain. Maybe this can fix that: rustup install stable AND rustup default stable",
  "Permission denied":
    "You may need to run this command with admin privilages (root?)",
  "Updating crates.io":
    "At the very first run/execute it will display errors because of installing/updating creates. Try run again and there won't be any errors.",
  "cargo: command not found":
    "You may need to run '. $HOME/.cargo/env' OR '. ~/.bashrc'",
};
