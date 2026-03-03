use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use similar::{ChangeTag, TextDiff};
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Word Document Diff Viewer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    doc1_path: Option<PathBuf>,
    doc2_path: Option<PathBuf>,
    doc1_text: String,
    doc2_text: String,
    diff_lines: Vec<DiffLine>,
    error_message: String,
}

#[derive(Clone, Debug)]
struct DiffLine {
    line_num_left: Option<usize>,
    line_num_right: Option<usize>,
    tag: ChangeTag,
    content_left: String,
    content_right: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            doc1_path: None,
            doc2_path: None,
            doc1_text: String::new(),
            doc2_text: String::new(),
            diff_lines: Vec::new(),
            error_message: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Word Document Diff Viewer");

            ui.horizontal(|ui| {
                if ui.button("Open Document 1").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Word Documents", &["docx"])
                        .pick_file()
                    {
                        self.doc1_path = Some(path.clone());
                        match extract_text_from_docx(&path) {
                            Ok(text) => {
                                self.doc1_text = text;
                                self.error_message.clear();
                                self.update_diff();
                            }
                            Err(e) => {
                                self.error_message = format!("Error reading document 1: {}", e);
                            }
                        }
                    }
                }

                if let Some(path) = &self.doc1_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy()));
                } else {
                    ui.label("No document selected");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Open Document 2").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Word Documents", &["docx"])
                        .pick_file()
                    {
                        self.doc2_path = Some(path.clone());
                        match extract_text_from_docx(&path) {
                            Ok(text) => {
                                self.doc2_text = text;
                                self.error_message.clear();
                                self.update_diff();
                            }
                            Err(e) => {
                                self.error_message = format!("Error reading document 2: {}", e);
                            }
                        }
                    }
                }

                if let Some(path) = &self.doc2_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy()));
                } else {
                    ui.label("No document selected");
                }
            });

            if !self.error_message.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error_message);
            }

            ui.separator();

            // Display diff in side-by-side format
            if !self.diff_lines.is_empty() {
                ui.heading("Differences (Git-style)");
                
                let available_height = ui.available_height();
                egui::ScrollArea::vertical()
                    .max_height(available_height)
                    .show(ui, |ui| {
                        for diff_line in &self.diff_lines {
                            display_diff_line(ui, diff_line);
                        }
                    });
            } else if self.doc1_path.is_some() && self.doc2_path.is_some() {
                ui.label("Documents are identical");
            } else {
                ui.label("Select two documents to compare");
            }
        });
    }
}

impl MyApp {
    fn update_diff(&mut self) {
        let diff = TextDiff::from_lines(&self.doc1_text, &self.doc2_text);
        
        self.diff_lines.clear();
        let mut left_line_num = 1;
        let mut right_line_num = 1;

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    self.diff_lines.push(DiffLine {
                        line_num_left: Some(left_line_num),
                        line_num_right: None,
                        tag: ChangeTag::Delete,
                        content_left: change.value().to_string(),
                        content_right: String::new(),
                    });
                    left_line_num += 1;
                }
                ChangeTag::Insert => {
                    self.diff_lines.push(DiffLine {
                        line_num_left: None,
                        line_num_right: Some(right_line_num),
                        tag: ChangeTag::Insert,
                        content_left: String::new(),
                        content_right: change.value().to_string(),
                    });
                    right_line_num += 1;
                }
                ChangeTag::Equal => {
                    self.diff_lines.push(DiffLine {
                        line_num_left: Some(left_line_num),
                        line_num_right: Some(right_line_num),
                        tag: ChangeTag::Equal,
                        content_left: change.value().to_string(),
                        content_right: change.value().to_string(),
                    });
                    left_line_num += 1;
                    right_line_num += 1;
                }
            }
        }
    }
}

fn display_diff_line(ui: &mut egui::Ui, diff_line: &DiffLine) {
    let (color, prefix_left, prefix_right) = match diff_line.tag {
        ChangeTag::Delete => (egui::Color32::from_rgb(255, 200, 200), "- ", ""),
        ChangeTag::Insert => (egui::Color32::from_rgb(200, 255, 200), "", "+ "),
        ChangeTag::Equal => (egui::Color32::from_rgb(240, 240, 240), "  ", "  "),
    };

    ui.horizontal(|ui| {
        // Left side
        ui.group(|ui| {
            ui.set_max_width(ui.available_width() / 2.0 - 5.0);
            
            if let Some(line_num) = diff_line.line_num_left {
                ui.colored_label(egui::Color32::GRAY, format!("{:4}", line_num));
            } else {
                ui.colored_label(egui::Color32::GRAY, "    ");
            }
            
            let left_text = if diff_line.tag == ChangeTag::Delete {
                format!("{}{}", prefix_left, diff_line.content_left.trim_end())
            } else if diff_line.tag == ChangeTag::Equal {
                format!("{}{}", prefix_left, diff_line.content_left.trim_end())
            } else {
                String::new()
            };

            if !left_text.is_empty() {
                ui.colored_label(color, left_text);
            }
        });

        // Right side
        ui.group(|ui| {
            ui.set_max_width(ui.available_width());
            
            if let Some(line_num) = diff_line.line_num_right {
                ui.colored_label(egui::Color32::GRAY, format!("{:4}", line_num));
            } else {
                ui.colored_label(egui::Color32::GRAY, "    ");
            }
            
            let right_text = if diff_line.tag == ChangeTag::Insert {
                format!("{}{}", prefix_right, diff_line.content_right.trim_end())
            } else if diff_line.tag == ChangeTag::Equal {
                format!("{}{}", prefix_right, diff_line.content_right.trim_end())
            } else {
                String::new()
            };

            if !right_text.is_empty() {
                ui.colored_label(color, right_text);
            }
        });
    });
}

fn extract_text_from_docx(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut zip = zip::ZipArchive::new(file)?;
    
    // Try to extract text from document.xml
    let mut xml_content = String::new();
    if let Ok(mut xml_file) = zip.by_name("word/document.xml") {
        use std::io::Read;
        xml_file.read_to_string(&mut xml_content)?;
    } else {
        return Err("Could not find document.xml in DOCX file".into());
    }

    // Parse XML and extract text from <w:t> elements (Word text elements)
    let text = extract_text_from_xml(&xml_content);
    Ok(text)
}

fn extract_text_from_xml(xml: &str) -> String {
    let mut text = String::new();
    let mut current_text = String::new();

    // Simple XML parsing for <w:t> tags
    for line in xml.lines() {
        // Look for opening paragraph tags to add line breaks
        if line.contains("<w:p>") && !current_text.is_empty() {
            text.push_str(&current_text);
            text.push('\n');
            current_text.clear();
        }

        // Extract text from <w:t> elements
        if let Some(start) = line.find("<w:t") {
            if let Some(end) = line.find("</w:t>") {
                if let Some(content_start) = line[start..].find('>') {
                    let content = &line[start + content_start + 1..end];
                    current_text.push_str(content);
                }
            }
        }
    }

    if !current_text.is_empty() {
        text.push_str(&current_text);
    }

    text
}
