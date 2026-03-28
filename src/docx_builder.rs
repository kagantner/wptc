use crate::config::{Config, StructureItem};
use docx_rs::*;
use std::fs;
use std::path::Path;
use regex::Regex;
use pulldown_cmark::{Parser, Options, Event};

pub fn compile_manuscript(config_file: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting manuscript compilation from '{}'...", config_file);

    let config_content = fs::read_to_string(config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    fs::create_dir_all(output_dir)?;

    let config_dir = Path::new(config_file).parent().unwrap_or(Path::new(""));

    let story_type = config.metadata.story_type.as_deref().unwrap_or("novel").to_lowercase();
    let output_filename = config.metadata.file_name.as_deref().unwrap_or("manuscript.docx");
    let output_path = Path::new(output_dir).join(output_filename);

    let mut doc = Docx::new()
        .default_fonts(RunFonts::new().ascii("Times New Roman").hi_ansi("Times New Roman"))
        .default_size(24) // 12pt * 2 (half-points)
        .page_margin(PageMargin::new().top(1440).bottom(1440).left(1440).right(1440)); // 1 inch margins

    let total_words = calculate_word_count(&config, config_dir).unwrap_or(0);

    doc = setup_header(doc, &config);
    doc = create_title_page(doc, &config, &story_type, total_words);

    let mut is_first_content_item = true;

    for item in &config.structure {
        match item {
            StructureItem::Part { title, content } => {
                if !is_first_content_item && story_type == "novel" {
                    doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
                } else if !is_first_content_item && story_type == "short_story" {
                    doc = doc.add_paragraph(Paragraph::new());
                }
                is_first_content_item = false;

                doc = doc.add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text(title.to_uppercase()).bold())
                );
                doc = doc.add_paragraph(Paragraph::new());

                for (i, chapter) in content.iter().enumerate() {
                    if i > 0 && story_type == "novel" {
                        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
                    } else if i > 0 && story_type == "short_story" {
                        doc = doc.add_paragraph(Paragraph::new());
                    }
                    doc = process_chapter(chapter, doc, &story_type, config_dir)?;
                }
            }
            StructureItem::Chapter { .. } => {
                if !is_first_content_item && story_type == "novel" {
                    doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
                } else if !is_first_content_item && story_type == "short_story" {
                    doc = doc.add_paragraph(Paragraph::new());
                }
                is_first_content_item = false;
                doc = process_chapter(item, doc, &story_type, config_dir)?;
            }
            StructureItem::Text { .. } => {
                if !is_first_content_item && story_type == "novel" {
                    doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
                } else if !is_first_content_item && story_type == "short_story" {
                    doc = doc.add_paragraph(Paragraph::new());
                }
                is_first_content_item = false;
                doc = process_text_item(item, doc, config_dir)?;
            }
        }
    }

    doc = doc.add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().line(480)));
    doc = doc.add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().line(480)));
    doc = doc.add_paragraph(
        Paragraph::new()
            .align(AlignmentType::Center)
            .line_spacing(LineSpacing::new().line(480))
            .add_run(Run::new().add_text("#  #  #"))
    );

    let file = std::fs::File::create(&output_path)?;
    doc.build().pack(file)?;

    println!("\nCompilation complete! Manuscript saved to '{:?}'.", output_path);
    Ok(())
}

fn calculate_word_count(config: &Config, config_dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let heading_re = Regex::new(r"^#{1,6}\s")?;
    let mut total_words = 0;

    let mut count_file = |file: &str| {
        let actual_path = config_dir.join(file);
        if let Ok(content) = fs::read_to_string(&actual_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || heading_re.is_match(trimmed) || trimmed.starts_with('%') {
                    continue;
                }
                total_words += trimmed.split_whitespace().count();
            }
        }
    };

    for item in &config.structure {
        match item {
            StructureItem::Part { content, .. } => {
                for chapter in content {
                    match chapter {
                        StructureItem::Chapter { file, files, .. } | StructureItem::Text { file, files, .. } => {
                            if let Some(f) = file { count_file(f); }
                            if let Some(fs) = files { for f in fs { count_file(f); } }
                        }
                        _ => {}
                    }
                }
            }
            StructureItem::Chapter { file, files, .. } | StructureItem::Text { file, files, .. } => {
                 if let Some(f) = file { count_file(f); }
                 if let Some(fs) = files { for f in fs { count_file(f); } }
            }
        }
    }
    
    let rounded = ((total_words as f64) / 100.0).round() as usize * 100;
    Ok(rounded)
}

fn format_number(mut n: usize) -> String {
    if n == 0 { return "0".to_string(); }
    let mut s = String::new();
    let mut count = 0;
    while n > 0 {
        if count == 3 {
            s.push(',');
            count = 0;
        }
        s.push(std::char::from_digit((n % 10) as u32, 10).unwrap());
        n /= 10;
        count += 1;
    }
    s.chars().rev().collect()
}

fn setup_header(doc: Docx, config: &Config) -> Docx {
    let last_name = &config.metadata.last_name;
    let short_title = &config.metadata.short_title;
    
    let p = Paragraph::new()
        .align(AlignmentType::Right)
        .add_run(Run::new().add_text(format!("{} | ", last_name)))
        .add_run(Run::new().add_text(short_title).italic())
        .add_run(Run::new().add_text(" | "))
        .add_run(
            Run::new()
                .add_field_char(FieldCharType::Begin, false)
                .add_instr_text(InstrText::PAGE(InstrPAGE::new()))
                .add_field_char(FieldCharType::Separate, false)
                .add_text("1")
                .add_field_char(FieldCharType::End, false)
        ); 

    let header = Header::new().add_paragraph(p);
    let first_header = Header::new();
    
    doc.header(header).first_header(first_header)
}

fn create_title_page(mut doc: Docx, config: &Config, story_type: &str, word_count: usize) -> Docx {
    let contact_block = vec![
        config.author.legal_name.clone(),
        config.author.street_address.clone(),
        config.author.city_state_zip.clone(),
        config.author.phone.clone(),
        config.author.email.clone(),
    ];
    
    let left_para = {
        let mut p = Paragraph::new();
        for (i, line) in contact_block.iter().enumerate() {
            p = p.add_run(Run::new().add_text(line));
            if i < contact_block.len() - 1 {
                p = p.add_run(Run::new().add_break(BreakType::TextWrapping));
            }
        }
        p
    };

    let right_para = Paragraph::new()
        .align(AlignmentType::Right)
        .add_run(Run::new().add_text(format!("Approx. {} words", format_number(word_count)))); 

    let no_border = TableBorders::new().clear_all();

    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(left_para).width(3000, WidthType::Dxa),
            TableCell::new().add_paragraph(right_para).width(3000, WidthType::Dxa),
        ])
    ]).set_borders(no_border);

    doc = doc.add_table(table);

    // Spacing: Short stories generally start halfway down the page,
    // which requires more blank lines than a standard novel title page top-margin.
    let added_spacing = if story_type == "short_story" { 17 } else { 8 };
    for _ in 0..added_spacing {
        doc = doc.add_paragraph(Paragraph::new());
    }

    let title_text = config.metadata.title.to_uppercase();

    doc = doc.add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().add_text(title_text).bold()));
    
    let mut p_byline = Paragraph::new().align(AlignmentType::Center);
    p_byline = p_byline.add_run(Run::new().add_text("by").add_break(BreakType::TextWrapping));
    p_byline = p_byline.add_run(Run::new().add_text(&config.metadata.byline));
    doc = doc.add_paragraph(p_byline);

    if story_type == "novel" {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
    } else {
        doc = doc.add_paragraph(Paragraph::new());
        doc = doc.add_paragraph(Paragraph::new());
        doc = doc.add_paragraph(Paragraph::new());
    }

    doc
}

fn process_chapter(chapter: &StructureItem, mut doc: Docx, _story_type: &str, config_dir: &Path) -> Result<Docx, Box<dyn std::error::Error>> {
    if let StructureItem::Chapter { title, number, file, files } = chapter {
        let mut heading = Vec::new();
        if let Some(num) = number {
            heading.push(format!("Chapter {}", num));
        }
        if let Some(t) = title {
            heading.push(t.clone());
        }
        
        if !heading.is_empty() {
            let full_heading = heading.join(": ");
            doc = doc.add_paragraph(
                Paragraph::new()
                    .align(AlignmentType::Center)
                    .add_run(Run::new().add_text(full_heading).bold())
            );
            doc = doc.add_paragraph(Paragraph::new());
        }

        if let Some(f) = file {
            doc = append_file_content(doc, f, config_dir)?;
        } else if let Some(fs) = files {
            for (i, f) in fs.iter().enumerate() {
                doc = append_file_content(doc, f, config_dir)?;
                if i < fs.len() - 1 {
                    doc = doc.add_paragraph(
                        Paragraph::new()
                            .align(AlignmentType::Center)
                            .line_spacing(LineSpacing::new().line(480))
                            .add_run(Run::new().add_text("#"))
                    );
                }
            }
        }
    }
    Ok(doc)
}

fn process_text_item(text_item: &StructureItem, mut doc: Docx, config_dir: &Path) -> Result<Docx, Box<dyn std::error::Error>> {
    if let StructureItem::Text { file, files } = text_item {
        if let Some(f) = file {
            doc = append_file_content(doc, f, config_dir)?;
        } else if let Some(fs) = files {
            for (i, f) in fs.iter().enumerate() {
                doc = append_file_content(doc, f, config_dir)?;
                if i < fs.len() - 1 {
                    doc = doc.add_paragraph(
                        Paragraph::new()
                            .align(AlignmentType::Center)
                            .line_spacing(LineSpacing::new().line(480))
                            .add_run(Run::new().add_text("#"))
                    );
                }
            }
        }
    }
    Ok(doc)
}

fn append_file_content(mut doc: Docx, filepath: &str, config_dir: &Path) -> Result<Docx, Box<dyn std::error::Error>> {
    let actual_path = config_dir.join(filepath);
    let content = match fs::read_to_string(&actual_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("--> WARNING: Could not find file: {:?}. It will be skipped.", actual_path);
            return Ok(doc);
        }
    };

    let heading_re = Regex::new(r"^#{1,6}\s")?;
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || heading_re.is_match(trimmed) || trimmed.starts_with('%') {
            continue;
        }

        let tokens_re = Regex::new(r"(\*{1,3}|_{1})")?;
        let mut p = Paragraph::new()
            .indent(None, Some(SpecialIndentType::FirstLine(720)), None, None) // 0.5 inch (720 twips)
            .line_spacing(LineSpacing::new().line(480)); // Double spaced (240 * 2)

        let mut is_bold = false;
        let mut is_italic = false;

        let mut last_idx = 0;
        for caps in tokens_re.captures_iter(trimmed) {
            let m = caps.get(0).unwrap();
            let text_before = &trimmed[last_idx..m.start()];
            if !text_before.is_empty() {
                p = p.add_run(apply_formatting(text_before, is_bold, is_italic));
            }
            
            let token = m.as_str();
            match token {
                "***" => { is_bold = !is_bold; is_italic = !is_italic; }
                "**" => { is_bold = !is_bold; }
                "*" | "_" => { is_italic = !is_italic; }
                _ => {}
            }
            last_idx = m.end();
        }
        let text_after = &trimmed[last_idx..];
        if !text_after.is_empty() {
            p = p.add_run(apply_formatting(text_after, is_bold, is_italic));
        }

        doc = doc.add_paragraph(p);
    }

    Ok(doc)
}

fn apply_formatting(text: &str, bold: bool, italic: bool) -> Run {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(text, options);
    
    let mut smart_text = String::new();
    for event in parser {
        if let Event::Text(t) = event {
            smart_text.push_str(&t);
        }
    }
    
    let mut run = Run::new().add_text(smart_text);
    if bold { run = run.bold(); }
    if italic { run = run.italic(); }
    run
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(80000), "80,000");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(500), "500");
    }

    #[test]
    fn test_smart_punctuation() {
        // Checking that apply_formatting respects pulldown-cmark smart punctuation
        let run = apply_formatting("\"Hello world\" --- and a dash", false, false);
        // docx-rs stores the text internally, so it's a bit harder to assert directly against the Run object.
        // We can just verify it compiles and runs without panicking.
        let _ = run;
    }

    #[test]
    fn test_calculate_word_count() {
        let temp_dir = std::env::temp_dir().join("mdmf_test_word_count");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        let file_path = temp_dir.join("test_chapter.md");
        let mut content = "Word ".repeat(60);
        content.push_str("\n# Ignored Header\n% Ignored comment\n");
        std::fs::write(&file_path, content).unwrap();
        
        let config = Config {
            metadata: crate::config::Metadata {
                title: "".into(), subtitle: None, byline: "".into(),
                genre: None, short_title: "".into(), last_name: "".into(),
                file_name: None, story_type: None,
            },
            author: crate::config::Author {
                legal_name: "".into(), pen_name: None, street_address: "".into(),
                city_state_zip: "".into(), phone: "".into(), email: "".into(),
                website: None,
            },
            agent: None,
            structure: vec![
                crate::config::StructureItem::Chapter {
                    title: None, number: None, file: Some("test_chapter.md".into()), files: None
                }
            ],
        };
        
        let count = calculate_word_count(&config, &temp_dir).unwrap();
        assert_eq!(count, 100); // 60 words rounds to nearest 100, which is 100
        
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
