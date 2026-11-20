// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};
use tauri::{AppHandle, Url, WebviewUrl};
use tauri_plugin_shell::ShellExt;

// Hosts that must stay inside the app: Muse itself and the sign-in flow.
fn is_internal(url: &Url) -> bool {
    if url.scheme() != "http" && url.scheme() != "https" {
        return true; // about:, data:, etc.
    }
    let host = match url.host_str() {
        Some(h) => h.to_ascii_lowercase(),
        None => return true,
    };
    const DOMAINS: [&str; 5] = [
        "muse.ai",
        "meta.com",
        "facebook.com",
        "accounts.google.com",
        "appleid.apple.com",
    ];
    DOMAINS
        .iter()
        .any(|d| host == *d || host.ends_with(&format!(".{d}")))
}

fn open_external(app: &AppHandle, url: &Url) {
    let _ = app.shell().open(url.as_str(), None);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let nav_handle = app.handle().clone();
            let win_handle = app.handle().clone();
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://muse.ai".parse().unwrap()),
            )
            .title("Muse")
            .inner_size(1100.0, 760.0)
            .min_inner_size(720.0, 480.0)
            .center()
            .on_navigation(move |url| {
                if is_internal(url) {
                    true
                } else {
                    open_external(&nav_handle, url);
                    false
                }
            })
            .on_new_window(move |url, _features| {
                if !is_internal(&url) {
                    open_external(&win_handle, &url);
                }
                NewWindowResponse::Deny
            })
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Muse");
}
