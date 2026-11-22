# YT Only
A YouTube-only browser in Rust (tao + wry / WebView2), OLED black, Shorts auto-scroll, uBlock Origin Lite bundled.

## Build on Windows (needs Rust from https://rustup.rs)
    powershell -ExecutionPolicy Bypass -File fetch-ubol.ps1
    cargo build --release
Result: target\release\ytonly.exe (single file, runs on Windows 11 — WebView2 is built in).

## Build without installing anything
Push this folder to a GitHub repo -> Actions tab -> "Build Windows exe" -> download the ytonly-windows artifact.
