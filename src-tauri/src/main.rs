// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};
use tauri::{Url, WebviewUrl};

const LOG_PATH: &str = "C:\\Users\\mushroom\\muse-work\\muse-link-log.txt";

fn log_line(msg: &str) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
    {
        let _ = writeln!(f, "[{ts}] {msg}");
    }
}

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

// Open a URL in the default browser via the Windows file-protocol handler.
fn open_external(url: &Url) {
    let u = url.as_str();
    match std::process::Command::new("rundll32.exe")
        .args(["url.dll,FileProtocolHandler", u])
        .spawn()
    {
        Ok(_) => log_line(&format!("open ok: {u}")),
        Err(e) => log_line(&format!("open FAILED: {u} err={e}")),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://muse.ai".parse().unwrap()),
            )
            .title("Muse")
            .inner_size(1100.0, 760.0)
            .min_inner_size(720.0, 480.0)
            .center()
            .on_navigation(|url| {
                if is_internal(url) {
                    true
                } else {
                    log_line(&format!("navigation intercepted: {}", url.as_str()));
                    open_external(url);
                    false
                }
            })
            .on_new_window(|url, _features| {
                // Anything the page tries to open as a new window (links with
                // target="_blank", window.open) goes to the default browser
                // instead — including internal destinations such as the Meta
                // Account Center, which is a standalone page, not part of the
                // in-app sign-in flow (that flow navigates the main frame and
                // is handled by on_navigation above).
                log_line(&format!("new-window intercepted: {}", url.as_str()));
                open_external(&url);
                NewWindowResponse::Deny
            })
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Muse");
}
