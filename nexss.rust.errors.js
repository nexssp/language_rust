// more about errors: https://github.com/nexssp/cli/wiki/Errors-Solutions
module.exports = {
  "no such subcommand": `nexss rs install cargo-script`,
  "no default toolchain":
    "No default toolchain. Maybe this can fix that: rustup install stable AND rustup default stable",
  "Permission denied":
    "You may need to run this command with admin privilages (root?)",
};
