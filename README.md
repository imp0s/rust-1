# Word Document Diff Viewer

A Rust application that allows you to compare two Word documents (.docx) and view their differences in a Git-style side-by-side format.

## Features

- **GUI Window**: Clean desktop interface built with egui
- **File Selection**: Open and select two Word documents using native file dialogs
- **Text Extraction**: Automatically extracts text content from .docx files
- **Diff Visualization**: Shows differences in Git-style format:
  - Red highlights for deleted lines (marked with `-`)
  - Green highlights for added lines (marked with `+`)
  - Gray background for unchanged lines (marked with spaces)
- **Line Numbers**: Displays line numbers for both documents
- **Scrollable Interface**: View long documents with scrolling

## Building and Running

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## How to Use

1. Launch the application
2. Click "Open Document 1" to select the first Word document
3. Click "Open Document 2" to select the second Word document
4. The diff will automatically display below, showing:
   - Lines deleted in Document 1 (red, left side)
   - Lines added in Document 2 (green, right side)
   - Unchanged lines (gray, both sides)

## Dependencies

- **egui** - Immediate mode GUI framework
- **eframe** - Application framework with egui
- **rfd** - Native file dialogs
- **docx-rs** - Word document handling
- **similar** - Diff algorithm implementation
- **zip** - ZIP archive handling for .docx files
- **serde** - Serialization framework

## Technical Details

- Extracts text from Word documents by parsing the `word/document.xml` XML structure within the .docx ZIP archive
- Uses the `TextDiff::from_lines()` function to compute line-based differences
- Displays results using egui's immediate mode rendering with color-coded sections

## Limitations

- Currently extracts only text content; formatting, styles, and images are not preserved in the diff
- Requires .docx format (modern Word format)
