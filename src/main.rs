// Hide the console window in release builds on Windows.
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::{fs, io::Cursor, path::{Path, PathBuf}};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme, WindowBuilder},
};
use wry::{WebContext, WebViewBuilder};
#[cfg(windows)]
use wry::WebViewBuilderExtWindows;

// uBlock Origin Lite (Chromium build), downloaded at build time into assets/ubol.zip
static UBOL_ZIP: &[u8] = include_bytes!("../assets/ubol.zip");
static INJECT_JS: &str = include_str!("inject.js");

/// Only YouTube (and the Google sign-in page) may be navigated to.
fn allowed(url: &str) -> bool {
    if url.starts_with("about:blank") {
        return true;
    }
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let host = rest
        .split(|c| c == '/' || c == '?' || c == '#')
        .next()
        .unwrap_or("")
        .rsplit('@')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let ok = |d: &str| host == d || host.ends_with(&format!(".{d}"));
    ok("youtube.com") || ok("youtu.be") || host == "accounts.google.com"
}

/// Unpack the bundled extension once, into %LOCALAPPDATA%\YTOnly\extensions\ubol
fn install_ubol(dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if dest.join("manifest.json").exists() {
        return Ok(());
    }
    fs::create_dir_all(dest)?;
    let mut zip = zip::ZipArchive::new(Cursor::new(UBOL_ZIP))?;
    let flat = zip.by_name("manifest.json").is_ok();
    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let Some(name) = f.enclosed_name() else { continue };
        let rel: PathBuf = if flat { name } else { name.components().skip(1).collect() };
        if rel.as_os_str().is_empty() {
            continue;
        }
        let out = dest.join(rel);
        if f.is_dir() {
            fs::create_dir_all(&out)?;
        } else {
            if let Some(p) = out.parent() {
                fs::create_dir_all(p)?;
            }
            let mut o = fs::File::create(&out)?;
            std::io::copy(&mut f, &mut o)?;
        }
    }
    Ok(())
}

fn main() -> wry::Result<()> {
    let base = PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".into())).join("YTOnly");
    let ext_root = base.join("extensions");
    let _ = install_ubol(&ext_root.join("ubol"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("YouTube")
        .with_theme(Some(Theme::Dark))
        .with_maximized(true)
        .build(&event_loop)
        .expect("failed to create window");

    let mut ctx = WebContext::new(Some(base.join("profile")));
    let builder = WebViewBuilder::new_with_web_context(&mut ctx)
        .with_url("https://www.youtube.com")
        .with_background_color((0, 0, 0, 255))
        .with_initialization_script(INJECT_JS)
        .with_navigation_handler(|u| allowed(&u))
        // GPU rendering + hardware video decode (keeps wry's default feature flags too)
        .with_additional_browser_args(
            "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
             --enable-features=D3D11VideoDecoder,CanvasOopRasterization \
             --enable-gpu-rasterization --enable-zero-copy --ignore-gpu-blocklist \
             --enable-smooth-scrolling",
        );

    #[cfg(windows)]
    let builder = builder
        .with_browser_extensions_enabled(true)
        .with_extensions_path(&ext_root);

    let webview = builder.build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        let _keep = (&webview, &ctx);
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
