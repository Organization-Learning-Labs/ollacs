Write-Host "Building Release Binary..."
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --release --manifest-path rust-core/Cargo.toml -p agent
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Generating Windows Installer (MSI)..."
Set-Location rust-core
& "$env:USERPROFILE\.cargo\bin\cargo.exe" wix --package agent
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`nWindows Installer Generated Successfully!"
Write-Host "Location: rust-core/target/wix/"

Write-Host "`n--- Cross-Platform Instructions ---"
Write-Host "To generate macOS Installer (.dmg / .app):"
Write-Host "  1. Transfer the source code to a macOS machine."
Write-Host "  2. Install cargo-bundle: cargo install cargo-bundle"
Write-Host "  3. Run: cargo bundle --release"
Write-Host "  4. Output: target/release/bundle/osx/"

Write-Host "`nTo generate Linux Installer (.deb):"
Write-Host "  1. Transfer the source code to a Linux machine (Ubuntu/Debian)."
Write-Host "  2. Install cargo-deb: cargo install cargo-deb"
Write-Host "  3. Run: cargo deb -p agent"
Write-Host "  4. Output: target/debian/"

Write-Host "`nTo generate Linux AppImage:"
Write-Host "  1. Use linuxdeploy with the binary built on Linux."
Write-Host "  2. See: https://github.com/linuxdeploy/linuxdeploy"
