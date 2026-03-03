#!/usr/bin/env pwsh
# Word Document Diff Viewer - Build and Run Script

Write-Host "Building Word Document Diff Viewer..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Make sure Rust is installed." -ForegroundColor Red
    Write-Host "Visit https://rustup.rs/ to install Rust." -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "Build successful! Launching application..." -ForegroundColor Green
Write-Host ""

& ".\target\release\word-diff-viewer.exe"
