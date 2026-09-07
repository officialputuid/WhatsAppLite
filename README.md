# WhatsApp Lite

Small Windows wrapper for `https://web.whatsapp.com/`, built with Tauri v2 and Microsoft WebView2.

## Features

- Persistent WebView2 profile, so WhatsApp login survives restarts
- Close-to-tray by default
- Tray actions: open, reload, toggle close behavior, toggle Windows autostart, quit
- Single-instance launch
- External HTTPS links open in the default browser
- Non-HTTPS navigation blocked
- No Tauri IPC capabilities exposed to remote WhatsApp content

## Requirements

- Windows 10 or 11
- WebView2 Runtime (installer downloads bootstrapper when needed)
- Rust stable with MSVC build tools

## Development

```powershell
cargo test --all-targets
cargo tauri dev
```

## Installer

```powershell
cargo install tauri-cli --version "^2" --locked
cargo tauri build
```

Outputs land under `target/release/bundle/msi/` and `target/release/bundle/nsis/`. GitHub Actions also builds both installers on Windows.
