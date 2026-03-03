# Word Document Diff Viewer - Development Instructions

## Project Overview
A Rust GUI application for comparing Word documents (.docx) with Git-style side-by-side diff visualization.

## Build Requirements
- Rust 1.70+
- Windows, macOS, or Linux

## Building
```bash
cargo build --release
```

## Running
```bash
cargo run --release
```

## Project Structure
- `src/main.rs` - Main application with GUI, diff logic, and DOCX parsing
- `Cargo.toml` - Project dependencies and metadata
- `README.md` - User documentation

## Key Components

### MyApp struct
Main application state managing:
- Document paths and text content
- Diff computation and storage
- Error messages

### extract_text_from_docx()
Extracts text from .docx files by:
1. Opening the DOCX as a ZIP archive
2. Reading the `word/document.xml` file
3. Parsing XML to extract text from `<w:t>` elements

### update_diff()
Computes line-based differences using the `similar` crate and formats results for display.

### display_diff_line()
Renders individual diff lines with proper coloring and line numbers.

## Development Notes
- Uses egui for the GUI framework - immediate mode rendering
- DOCX parsing is basic XML extraction from ZIP
- Diff algorithm is line-based (not character-based)
- File dialogs use native OS dialogs via rfd crate

## Future Enhancements
- Character-level diff within lines
- Preserve formatting in diff view
- Support for other document formats
- Search/filter functionality
- Export diff results
