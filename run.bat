@echo off
REM Word Document Diff Viewer - Build and Run Script

echo Building Word Document Diff Viewer...
cargo build --release

if errorlevel 1 (
    echo Build failed. Make sure Rust is installed.
    echo Visit https://rustup.rs/ to install Rust.
    pause
    exit /b 1
)

echo.
echo Build successful! Launching application...
echo.

.\target\release\word-diff-viewer.exe
