# author: Marcin Polak / Nexss.com <mapoart@gmail.com>
# 25 January 2020
$temptempl = $env:temp
$env:temp = "$HOME\.nexssApps\temp" 

$env:CARGO_HOME = "$HOME\.nexssApps\cargo"
$env:RUSTUP_HOME = "$HOME\.nexssApps\rustup"
$env:CARGO_TARGET_DIR = "$HOME\.nexssApps\cargoTarget"

Write-Host "Set CARGO_HOME directory to $($env:CARGO_HOME)"
Write-Host "Set RUSTUP_HOME directory to $($env:RUSTUP_HOME)"
Write-Host "Set CARGO_TARGET_DIR directory to $($env:CARGO_TARGET_DIR)"

curl -o rustup-init.exe https://win.rustup.rs
./rustup-init.exe -y
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User") 
#$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User") 
rustup --version
rustup default stable
# rustup toolchain install nightly 
rustup toolchain install stable 
rustup update
#! optional
# rustup component add rust-src
# rustup component add rust-docs
# rustup component add rust-analysis
# rustup component add rls-preview
# rustup component add rustfmt-preview

rustup completions powershell | Out-String | Invoke-Expression

# %USERPROFILE%\.cargo\bin
# Write-Host "NOTE: You may need to restart Powershell/Terminal after installation" -f Yellow

cargo install cargo-script --force

Write-Host "NOTE: You may need to restart Powershell/Terminal after installation" -f Yellow

$env:temp = $temptempl