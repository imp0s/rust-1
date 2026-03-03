# Installation Instructions

## Step 1: Install Rust

The Word Document Diff Viewer requires Rust to be installed on your system.

### Option A: Using rustup (Recommended)

1. Visit https://rustup.rs/
2. Download the rustup installer for Windows (x86_64-pc-windows-msvc)
3. Run the installer and follow the prompts
4. Restart your terminal after installation

### Option B: Download from official Rust website

1. Visit https://www.rust-lang.org/tools/install
2. Download the Windows installer
3. Run the installer

## Step 2: Verify Installation

After installing Rust, open a new PowerShell or Command Prompt and run:

```powershell
rustc --version
cargo --version
```

You should see version information for both `rustc` and `cargo`.

## Step 3: Build the Project

Navigate to the project directory and run:

```powershell
cd c:\Users\mpin\Projects\rust-1
cargo build --release
```

The first build will take some time (5-15 minutes) as Rust downloads and compiles all dependencies.

## Step 4: Run the Application

```powershell
cargo run --release
```

Or run the compiled binary directly:

```powershell
.\target\release\word-diff-viewer.exe
```

## Troubleshooting

### "cargo: command not found"
- Ensure Rust was installed correctly
- Restart your terminal/PowerShell
- Check that `C:\Users\<YourUsername>\.cargo\bin` is in your PATH environment variable

### Compilation errors
- Ensure you have Rust 1.70 or newer: `rustc --version`
- Delete the `target` directory and rebuild: `cargo clean && cargo build --release`

### Unable to open Word documents
- Ensure the files are in .docx format (not .doc or other formats)
- Check that you have read permissions for the files
- Verify the Word document is not corrupted
