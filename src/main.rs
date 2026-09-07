#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(windows))]
fn main() {
    println!("WhatsApp Lite targets Windows (WebView2).");
}

#[cfg(windows)]
mod windows_app {
    use std::{path::PathBuf, sync::Mutex};
    use tauri::{
        menu::{CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder},
        tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
        webview::{NewWindowResponse, WebviewWindowBuilder},
        AppHandle, Manager, WebviewUrl, WindowEvent,
    };
    use tauri_plugin_autostart::ManagerExt;
    use webview2_com::{
        Microsoft::Web::WebView2::Win32::{
            COREWEBVIEW2_PERMISSION_KIND_CAMERA, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
            COREWEBVIEW2_PERMISSION_KIND_NOTIFICATIONS, COREWEBVIEW2_PERMISSION_STATE_ALLOW,
            COREWEBVIEW2_PERMISSION_STATE_DENY,
        },
        PermissionRequestedEventHandler,
    };
    use whatsapp_lite::{
        classify_navigation, load_settings, save_settings, CloseBehavior, NavigationAction,
        Settings,
    };

    const MAIN: &str = "main";
    const OPEN: &str = "open";
    const RELOAD: &str = "reload";
    const CLOSE_TO_TRAY: &str = "close_to_tray";
    const AUTOSTART: &str = "autostart";
    const NOTIFICATIONS: &str = "notifications";
    const CAMERA: &str = "camera";
    const MICROPHONE: &str = "microphone";
    const QUIT: &str = "quit";

    struct State {
        settings: Mutex<Settings>,
        settings_path: PathBuf,
    }

    fn show_main(app: &AppHandle) {
        if let Some(window) = app.get_webview_window(MAIN) {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }

    fn update_settings(app: &AppHandle, change: impl FnOnce(&mut Settings)) {
        let state = app.state::<State>();
        if let Ok(mut settings) = state.settings.lock() {
            change(&mut settings);
            if let Err(error) = save_settings(&state.settings_path, &settings) {
                eprintln!("failed to save settings: {error}");
            }
        };
    }

    pub fn run() {
        tauri::Builder::default()
            .plugin(tauri_plugin_single_instance::init(|app, _, _| {
                show_main(app)
            }))
            .plugin(tauri_plugin_autostart::Builder::new().build())
            .plugin(tauri_plugin_opener::init())
            .setup(|app| {
                let settings_path = app.path().app_config_dir()?.join("settings.json");
                let settings = load_settings(&settings_path);
                app.manage(State {
                    settings: Mutex::new(settings.clone()),
                    settings_path,
                });

                if settings.autostart {
                    let _ = app.autolaunch().enable();
                }

                let close_to_tray =
                    CheckMenuItemBuilder::with_id(CLOSE_TO_TRAY, "Minimize to tray on close")
                        .checked(settings.close_behavior == CloseBehavior::Tray)
                        .build(app)?;
                let autostart = CheckMenuItemBuilder::with_id(AUTOSTART, "Start with Windows")
                    .checked(settings.autostart)
                    .build(app)?;
                let notifications = CheckMenuItemBuilder::with_id(NOTIFICATIONS, "Notifications")
                    .checked(settings.allow_notifications)
                    .build(app)?;
                let camera = CheckMenuItemBuilder::with_id(CAMERA, "Camera")
                    .checked(settings.allow_camera)
                    .build(app)?;
                let microphone = CheckMenuItemBuilder::with_id(MICROPHONE, "Microphone")
                    .checked(settings.allow_microphone)
                    .build(app)?;
                let permissions = SubmenuBuilder::new(app, "Settings")
                    .item(&notifications)
                    .item(&camera)
                    .item(&microphone)
                    .build()?;
                let menu = MenuBuilder::new(app)
                    .text(OPEN, "Open WhatsApp")
                    .text(RELOAD, "Reload")
                    .separator()
                    .item(&permissions)
                    .item(&close_to_tray)
                    .item(&autostart)
                    .separator()
                    .text(QUIT, "Quit")
                    .build()?;

                TrayIconBuilder::new()
                    .icon(
                        app.default_window_icon()
                            .cloned()
                            .expect("app icon missing"),
                    )
                    .tooltip("WhatsApp Lite")
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id().as_ref() {
                        OPEN => show_main(app),
                        RELOAD => {
                            if let Some(window) = app.get_webview_window(MAIN) {
                                let _ = window.reload();
                            }
                        }
                        CLOSE_TO_TRAY => update_settings(app, |settings| {
                            settings.close_behavior =
                                if settings.close_behavior == CloseBehavior::Tray {
                                    CloseBehavior::Exit
                                } else {
                                    CloseBehavior::Tray
                                };
                        }),
                        AUTOSTART => {
                            let enabled = app.autolaunch().is_enabled().unwrap_or(false);
                            let result = if enabled {
                                app.autolaunch().disable()
                            } else {
                                app.autolaunch().enable()
                            };
                            if result.is_ok() {
                                update_settings(app, |settings| settings.autostart = !enabled);
                            }
                        }
                        NOTIFICATIONS => update_settings(app, |settings| {
                            settings.allow_notifications = !settings.allow_notifications;
                        }),
                        CAMERA => update_settings(app, |settings| {
                            settings.allow_camera = !settings.allow_camera;
                        }),
                        MICROPHONE => update_settings(app, |settings| {
                            settings.allow_microphone = !settings.allow_microphone;
                        }),
                        QUIT => app.exit(0),
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if matches!(
                            event,
                            TrayIconEvent::Click {
                                button: MouseButton::Left,
                                button_state: MouseButtonState::Up,
                                ..
                            }
                        ) {
                            show_main(tray.app_handle());
                        }
                    })
                    .build(app)?;

                let new_window_app = app.handle().clone();
                let window = WebviewWindowBuilder::new(
                    app,
                    MAIN,
                    WebviewUrl::External("https://web.whatsapp.com/".parse().unwrap()),
                )
                .title("WhatsApp Lite")
                .inner_size(1100.0, 760.0)
                .min_inner_size(720.0, 520.0)
                .on_navigation(move |url| match classify_navigation(url.as_str()) {
                    NavigationAction::InApp => true,
                    NavigationAction::External => {
                        if let Err(error) =
                            tauri_plugin_opener::open_url(url.as_str(), None::<&str>)
                        {
                            eprintln!("failed to open external URL: {error}");
                        }
                        false
                    }
                    NavigationAction::Blocked => {
                        eprintln!("blocked navigation: {url}");
                        false
                    }
                })
                .on_new_window(move |url, _| match classify_navigation(url.as_str()) {
                    NavigationAction::InApp => {
                        if let Some(window) = new_window_app.get_webview_window(MAIN) {
                            if let Err(error) = window.navigate(url) {
                                eprintln!("failed to open WhatsApp URL: {error}");
                            }
                        }
                        NewWindowResponse::Deny
                    }
                    NavigationAction::External => {
                        if let Err(error) =
                            tauri_plugin_opener::open_url(url.as_str(), None::<&str>)
                        {
                            eprintln!("failed to open new-window URL: {error}");
                        }
                        NewWindowResponse::Deny
                    }
                    NavigationAction::Blocked => {
                        eprintln!("blocked new-window URL: {url}");
                        NewWindowResponse::Deny
                    }
                })
                .build()?;

                let permission_app = app.handle().clone();
                window.with_webview(move |webview| unsafe {
                    let Ok(core) = webview.controller().CoreWebView2() else {
                        eprintln!("failed to access WebView2 permissions");
                        return;
                    };
                    let handler =
                        PermissionRequestedEventHandler::create(Box::new(move |_, args| {
                            let Some(args) = args else {
                                return Ok(());
                            };
                            let mut kind = Default::default();
                            args.PermissionKind(&mut kind)?;
                            let settings = permission_app.state::<State>();
                            let Ok(settings) = settings.settings.lock() else {
                                return Ok(());
                            };
                            let allowed = match kind {
                                COREWEBVIEW2_PERMISSION_KIND_NOTIFICATIONS => {
                                    Some(settings.allow_notifications)
                                }
                                COREWEBVIEW2_PERMISSION_KIND_CAMERA => Some(settings.allow_camera),
                                COREWEBVIEW2_PERMISSION_KIND_MICROPHONE => {
                                    Some(settings.allow_microphone)
                                }
                                _ => None,
                            };
                            if let Some(allowed) = allowed {
                                args.SetState(if allowed {
                                    COREWEBVIEW2_PERMISSION_STATE_ALLOW
                                } else {
                                    COREWEBVIEW2_PERMISSION_STATE_DENY
                                })?;
                            }
                            Ok(())
                        }));
                    let mut token = 0;
                    if let Err(error) = core.add_PermissionRequested(&handler, &mut token) {
                        eprintln!("failed to configure WebView2 permissions: {error}");
                    }
                })?;

                let close_app = app.handle().clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        let close_to_tray = close_app
                            .state::<State>()
                            .settings
                            .lock()
                            .map(|settings| settings.close_behavior == CloseBehavior::Tray)
                            .unwrap_or(true);
                        if close_to_tray {
                            api.prevent_close();
                            if let Some(window) = close_app.get_webview_window(MAIN) {
                                let _ = window.hide();
                            }
                        }
                    }
                });
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("failed to run WhatsApp Lite");
    }
}

#[cfg(windows)]
fn main() {
    windows_app::run();
}
