# WhatsApp Lite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a lightweight Windows 10/11 WhatsApp Web wrapper with Tauri v2, WebView2 Evergreen, tray settings, persistent login, and MSI/NSIS installers.

**Architecture:** A dependency-light Rust core owns typed settings and URL policy. The Tauri binary owns WebView2, native menus/tray, single-instance handling, autostart, and external URL opening. Remote WhatsApp content receives no Tauri IPC capability.

**Tech Stack:** Rust stable, Tauri v2, WebView2 Evergreen, serde/serde_json, official Tauri single-instance/autostart/opener plugins, GitHub Actions Windows runner.

**Spec:** `docs/superpowers/specs/2026-09-08-whatsapp-lite-design.md`

## Global Constraints

- Target Windows 10/11 x64.
- Product name is `WhatsApp Lite`.
- Native Windows frame; no local frontend framework.
- WebView URL is `https://web.whatsapp.com/`.
- Default close behavior is minimize to tray.
- Build both MSI and NSIS EXE using WebView2 `downloadBootstrapper` mode.
- Voice/video calling is best-effort and must not be represented as guaranteed.
- Never expose Tauri IPC, shell execution, message content, credentials, or session data to remote content.

---

### Task 1: Tested settings and URL policy core

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`

**Interfaces:**
- Produces: `Settings`, `CloseBehavior`, `load_settings`, `save_settings`, `classify_navigation`, and `NavigationAction`.

- [ ] Write unit tests for defaults, JSON round-trip, corrupt JSON fallback, and WhatsApp/external/blocked URL classification.
- [ ] Run `cargo test`; verify tests fail because interfaces do not exist.
- [ ] Implement minimum typed settings and strict HTTPS navigation classification.
- [ ] Run `cargo test`; verify all tests pass.

### Task 2: Native Tauri shell

**Files:**
- Create: `build.rs`
- Create: `src/main.rs`
- Create: `tauri.conf.json`
- Create: `capabilities/main.json`
- Create: `icons/icon.ico`
- Create: `icons/icon.png`

**Interfaces:**
- Consumes core settings and URL policy from Task 1.
- Produces one persistent remote WebView2 window, tray menu, close behavior, autostart toggle, single-instance focus, reload, and quit.

- [ ] Add a compile-time smoke test for menu IDs and URL constants.
- [ ] Configure direct remote URL loading, native frame, persistent data directory, WebView2 download bootstrapper, MSI and NSIS targets.
- [ ] Implement tray actions and close-request interception.
- [ ] Implement navigation/new-window policy: approved WhatsApp HTTPS URLs remain in-app; external HTTPS URLs open in default browser; unsafe schemes are blocked.
- [ ] Register official single-instance, autostart, and opener plugins without remote capabilities.
- [ ] Run format, tests, and clippy.

### Task 3: Windows packaging and operator docs

**Files:**
- Create: `.github/workflows/windows-build.yml`
- Create: `.gitignore`
- Create: `README.md`

**Interfaces:**
- Produces downloadable `WhatsApp-Lite-installers` artifact containing `.msi` and `-setup.exe`.

- [ ] Configure GitHub Actions `windows-latest` with stable Rust and `tauri build --bundles msi,nsis`.
- [ ] Upload MSI and NSIS files as one workflow artifact.
- [ ] Document local Windows prerequisites, build command, supported settings, limitations, and manual smoke-test checklist.
- [ ] Validate YAML/JSON/TOML syntax, run Rust tests and formatting checks, and inspect repository for secrets/build artifacts.
