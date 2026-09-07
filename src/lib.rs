use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use url::Url;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseBehavior {
    #[default]
    Tray,
    Exit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct Settings {
    pub close_behavior: CloseBehavior,
    pub autostart: bool,
    pub allow_notifications: bool,
    pub allow_camera: bool,
    pub allow_microphone: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            close_behavior: CloseBehavior::Tray,
            autostart: false,
            allow_notifications: true,
            allow_camera: false,
            allow_microphone: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationAction {
    InApp,
    External,
    Blocked,
}

pub fn load_settings(path: &Path) -> Settings {
    fs::read(path)
        .ok()
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

pub fn save_settings(path: &Path, settings: &Settings) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(settings)?)?;
    fs::rename(temporary, path)
}

pub fn classify_navigation(raw: &str) -> NavigationAction {
    let Ok(url) = Url::parse(raw) else {
        return NavigationAction::Blocked;
    };
    if url.scheme() != "https" {
        return NavigationAction::Blocked;
    }
    match url.host_str() {
        Some(host)
            if host == "whatsapp.com"
                || host.ends_with(".whatsapp.com")
                || host == "whatsapp.net"
                || host.ends_with(".whatsapp.net") =>
        {
            NavigationAction::InApp
        }
        Some(_) => NavigationAction::External,
        None => NavigationAction::Blocked,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_defaults_are_safe() {
        let settings = Settings::default();
        assert_eq!(settings.close_behavior, CloseBehavior::Tray);
        assert!(!settings.autostart);
        assert!(settings.allow_notifications);
        assert!(!settings.allow_camera);
        assert!(!settings.allow_microphone);
    }

    #[test]
    fn settings_round_trip() {
        let dir = std::env::temp_dir().join(format!("whatsapp-lite-test-{}", std::process::id()));
        let path = dir.join("settings.json");
        let settings = Settings {
            close_behavior: CloseBehavior::Exit,
            autostart: true,
            allow_notifications: false,
            allow_camera: true,
            allow_microphone: true,
        };
        save_settings(&path, &settings).unwrap();
        assert_eq!(load_settings(&path), settings);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn corrupt_settings_fall_back_to_defaults() {
        let path =
            std::env::temp_dir().join(format!("whatsapp-lite-corrupt-{}.json", std::process::id()));
        std::fs::write(&path, b"not json").unwrap();
        assert_eq!(load_settings(&path), Settings::default());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn classifies_navigation_without_leaking_unsafe_schemes() {
        assert_eq!(
            classify_navigation("https://web.whatsapp.com/"),
            NavigationAction::InApp
        );
        assert_eq!(
            classify_navigation("https://static.whatsapp.net/asset.js"),
            NavigationAction::InApp
        );
        assert_eq!(
            classify_navigation("https://example.com/"),
            NavigationAction::External
        );
        assert_eq!(
            classify_navigation("http://web.whatsapp.com/"),
            NavigationAction::Blocked
        );
        assert_eq!(
            classify_navigation("javascript:alert(1)"),
            NavigationAction::Blocked
        );
        assert_eq!(classify_navigation("not a url"), NavigationAction::Blocked);
    }
}
