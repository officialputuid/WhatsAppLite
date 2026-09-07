# WhatsApp Lite Design

## Goal

Build a lightweight Windows 10/11 desktop wrapper for `https://web.whatsapp.com/` using Tauri v2 and Microsoft WebView2 Evergreen, distributed as MSI and NSIS EXE installers.

## Scope

- One persistent WhatsApp Web window with native Windows frame.
- Persistent WebView2 profile so login survives restarts.
- System tray: Open WhatsApp, Settings, Reload, Quit.
- Configurable close behavior: minimize to tray by default or exit.
- Configurable launch at startup and notifications.
- Upload/download through WebView2 native behavior.
- Camera and microphone permissions for voice/video calls when WebView2 and WhatsApp permit them.
- Single instance; a second launch focuses the existing window.
- External non-WhatsApp links open in the default browser.
- MSI and NSIS EXE bundles for x64 Windows.

## Architecture

Tauri creates the main webview directly from the remote WhatsApp URL. Rust owns window lifecycle, tray menu, settings persistence, single-instance handling, navigation policy, and startup registration. No frontend framework or injected message-reading script is used.

A small local Settings window is intentionally avoided. Settings live in the tray submenu to keep the app smaller and prevent remote WhatsApp content from receiving Tauri IPC access.

## Security

- No remote Tauri capability or custom command is exposed to `web.whatsapp.com`.
- Top-level navigation permits only HTTPS WhatsApp-owned hosts needed by WhatsApp Web; external links go to the system browser.
- No message, contact, QR, cookie, password, or media content is read by application code.
- DevTools are disabled in release configuration.
- Shell execution is not exposed.
- Settings are stored as a small JSON file under the platform app-config directory.

## Settings

- `close_behavior`: `tray` (default) or `exit`.
- `autostart`: disabled by default.
- `notifications`: enabled by default where WebView2 permits web notifications.

The tray Settings submenu exposes these values without an extra webview.

## Compatibility

WebView2 Evergreen is shared with Windows. Installer mode `downloadBootstrapper` keeps installer size low and installs WebView2 when absent. Windows 11 normally includes it; supported Windows 10 installations can receive it through the bootstrapper.

Voice/video calls are best-effort. WhatsApp may reject WebView2, change browser checks, or limit calling independently of this app. Basic chat, upload, download, microphone/camera permission requests, and web notifications remain the target.

## Verification

- Rust unit tests cover settings defaults, persistence, corrupt-file recovery, and navigation classification.
- `cargo test` and `cargo clippy -- -D warnings` run on Linux for portable logic.
- GitHub Actions Windows runner executes tests and `tauri build --bundles msi,nsis`, then publishes both installers as workflow artifacts.
- Real Windows smoke test: QR login persists; upload/download work; mic/camera prompts appear; close behavior changes; tray actions work; second launch focuses first; external links leave the app.
