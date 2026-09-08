<div align="center">
  <img src="icons/128x128@2x.png" width="112" height="112" alt="WhatsApp Lite icon">

# WhatsApp Lite

A focused Windows desktop wrapper for WhatsApp Web, built with Tauri v2 and the Microsoft Edge WebView2 runtime.

[Download latest release](https://github.com/officialputuid/WhatsAppLite/releases/latest) · [Report an issue](https://github.com/officialputuid/WhatsAppLite/issues)

[![Windows build](https://github.com/officialputuid/WhatsAppLite/actions/workflows/windows.yml/badge.svg)](https://github.com/officialputuid/WhatsAppLite/actions/workflows/windows.yml)
![Windows 10/11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows&logoColor=white)
![Tauri 2](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)

</div>

## Why WhatsApp Lite?

WhatsApp Lite keeps WhatsApp Web in a dedicated desktop window without bundling another full Chromium installation. It uses the WebView2 runtime already present on most Windows 10 and Windows 11 systems.

The result is a small native shell with persistent login, tray controls, and fewer distractions than a browser tab. Actual memory use still depends on WhatsApp Web, active chats, media, and WebView2.

## Features

- Persistent WhatsApp login through the WebView2 profile
- Close to system tray by default
- Optional launch at Windows startup
- Single-instance behavior
- Tray controls for opening, reloading, permissions, startup, and exit
- Configurable notifications, camera, and microphone access
- Notifications enabled by default; camera and microphone disabled by default
- Permission grants restricted to `https://web.whatsapp.com`
- External HTTPS links open in the default browser
- Non-HTTPS navigation blocked
- No Tauri IPC capabilities exposed to remote WhatsApp content
- Built-in About menu and GitHub release check
- MSI and NSIS installers built by GitHub Actions

## Download and install

Download the newest installer from [GitHub Releases](https://github.com/officialputuid/WhatsAppLite/releases/latest).

| Package | Best for |
| --- | --- |
| NSIS `.exe` | Normal desktop installation |
| MSI `.msi` | Managed or enterprise deployment |

WebView2 is required. If it is missing, the installer downloads Microsoft's official Evergreen bootstrapper.

Windows may show an Unknown Publisher or SmartScreen warning because current installers are not Authenticode-signed.

## Using the tray menu

Right-click the WhatsApp Lite icon in the Windows notification area:

- **Open WhatsApp** brings the window to the foreground.
- **Reload** refreshes WhatsApp Web.
- **Settings** controls notifications, camera, and microphone access.
- **Minimize to tray on close** keeps WhatsApp running after the window is closed.
- **Start with Windows** controls autostart.
- **About** shows app details, checks for updates, and links to GitHub.
- **Quit** fully closes WhatsApp Lite and its WebView2 processes.

## Privacy and security

WhatsApp Lite loads the official `https://web.whatsapp.com/` website. It does not inject scripts to inspect messages and does not store passwords, QR codes, cookies, or chat contents in its own settings file.

Session data and website storage are managed by the persistent WebView2 profile. WhatsApp's own privacy policy and security model still apply.

The update checker reads the latest published release from the GitHub Releases API. It only offers to open the release page; it does not silently download or install updates.

## Requirements

### Running the app

- Windows 10 or Windows 11, 64-bit
- Microsoft Edge WebView2 Evergreen Runtime
- Internet connection and a WhatsApp account

### Building from source

- Rust stable
- Microsoft C++ Build Tools with the MSVC toolchain
- Tauri CLI v2
- WebView2 Runtime

## Development

```powershell
# Install Tauri CLI once
cargo install tauri-cli --version "^2" --locked

# Run checks
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

# Start the development build
cargo tauri dev
```

## Build installers

```powershell
cargo tauri build
```

Generated packages:

```text
target/release/bundle/nsis/
target/release/bundle/msi/
```

GitHub Actions runs formatting, tests, Clippy, and Windows packaging for pushes and pull requests. A version tag such as `v0.1.2` publishes a GitHub Release when it matches the package version in `Cargo.toml`.

## Project structure

```text
.
├── .github/workflows/windows.yml  # Windows CI and releases
├── assets/                        # Local frontend fallback
├── capabilities/                  # Minimal Tauri capabilities
├── icons/                         # Application icons
├── src/lib.rs                     # Settings and testable policies
├── src/main.rs                    # Tauri window, tray, and WebView2 integration
├── Cargo.toml
└── tauri.conf.json
```

## Limitations

- Voice and video calls depend on WhatsApp Web and WebView2 compatibility.
- Memory usage is driven mainly by WhatsApp Web and may rise with large chats, media, calls, and long sessions.
- Real-time notifications require the WebView2 process to remain active while the window is hidden in the tray.
- Current installers are unsigned.

## Disclaimer

WhatsApp Lite is an independent, unofficial wrapper. It is not affiliated with, endorsed by, or sponsored by WhatsApp LLC or Meta Platforms, Inc. WhatsApp and related marks belong to their respective owners.

## Developer

Built and maintained by [officialputuid](https://github.com/officialputuid).
