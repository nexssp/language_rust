scoop install rustup
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User") 
scoop rustup --version
rustup default stable
# rustup toolchain install nightly 
rustup toolchain install stable 
rustup update
rustup completions powershell | Out-String | Invoke-Expression

# %USERPROFILE%\.cargo\bin
Write-Host "NOTE: You may need to restart Powershell/Terminal after installation" -f Yellow

$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User") 

cargo install cargo-script --force

Write-Host "NOTE: You may need to restart Powershell/Terminal after installation" -f Yellow