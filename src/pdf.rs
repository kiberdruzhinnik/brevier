//! PDF text becomes Markdown locally; the parser is panic-contained because
//! downloaded documents are untrusted input.

/// Extract readable text from PDF bytes and title it with the shared filename.
pub fn markdown(bytes: &[u8], title: &str) -> Option<String> {
    let text = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pdf_extract::extract_text_from_mem(bytes)
    }))
    .ok()?
    .ok()?;
    let skladchik = is_skladchik_pdf(title, &text);
    let text = if skladchik {
        let has_footer = has_skladchik_footer(&text);
        let pages = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            extract_skladchik_pages(bytes)
        }));
        match pages {
            Ok(Some(pages)) if !pages.is_empty() => clean_skladchik_pages(&pages, has_footer),
            _ if has_footer => {
                let pages = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    pdf_extract::extract_text_from_mem_by_pages(bytes)
                }));
                match pages {
                    Ok(Ok(pages)) => cleanup_extracted_pages(&pages).unwrap_or(text),
                    _ => text,
                }
            }
            _ => text,
        }
    } else {
        text
    };
    let pages = text
        .split('\u{c}')
        .map(str::trim)
        .filter(|page| !page.is_empty())
        .collect::<Vec<_>>();
    if pages.is_empty() {
        return None;
    }
    Some(format!(
        "# {}\n\n{}\n",
        crate::feed::inline(title),
        join_pages(&pages, skladchik)
    ))
}

fn join_pages(pages: &[&str], skladchik: bool) -> String {
    pages.join(if skladchik { "\n" } else { "\n\n---\n\n" })
}

fn extract_skladchik_pages(bytes: &[u8]) -> Option<Vec<String>> {
    let document = pdf_extract::Document::load_mem(bytes).ok()?;
    let mut output = PositionedTextOutput::default();
    pdf_extract::output_doc(&document, &mut output).ok()?;
    Some(output.pages)
}

struct PositionedGlyph {
    x: f64,
    y: f64,
    font_size: f64,
    advance: f64,
    text: String,
}

#[derive(Default)]
struct PositionedTextOutput {
    pages: Vec<String>,
    glyphs: Vec<PositionedGlyph>,
    flip_ctm: Option<pdf_extract::Transform>,
}

impl pdf_extract::OutputDev for PositionedTextOutput {
    fn begin_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }

    fn begin_page(
        &mut self,
        _page_num: u32,
        media_box: &pdf_extract::MediaBox,
        _art_box: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), pdf_extract::OutputError> {
        self.glyphs.clear();
        self.flip_ctm = Some(pdf_extract::Transform::row_major(
            1.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            media_box.ury - media_box.lly,
        ));
        Ok(())
    }

    fn end_page(&mut self) -> Result<(), pdf_extract::OutputError> {
        self.flip_ctm = None;
        self.pages.push(render_positioned_page(&self.glyphs));
        Ok(())
    }

    fn output_character(
        &mut self,
        trm: &pdf_extract::Transform,
        width: f64,
        spacing: f64,
        font_size: f64,
        character: &str,
    ) -> Result<(), pdf_extract::OutputError> {
        let Some(flip_ctm) = self.flip_ctm.as_ref() else {
            return Ok(());
        };
        let position = trm.post_transform(flip_ctm);
        let transformed_x = (trm.m11 + trm.m21) * font_size;
        let transformed_y = (trm.m12 + trm.m22) * font_size;
        let transformed_font_size = (transformed_x * transformed_y).abs().sqrt();
        let (x, y) = (position.m31, position.m32);
        let horizontal_scale = (trm.m11 * trm.m11 + trm.m21 * trm.m21).sqrt();
        self.glyphs.push(PositionedGlyph {
            x,
            y,
            font_size: transformed_font_size,
            advance: (width * font_size + spacing) * horizontal_scale,
            text: character.to_owned(),
        });
        Ok(())
    }

    fn end_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }

    fn end_line(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
}

fn render_positioned_page(glyphs: &[PositionedGlyph]) -> String {
    let mut lines: Vec<Vec<&PositionedGlyph>> = Vec::new();
    for glyph in glyphs {
        let starts_line = lines
            .last()
            .and_then(|line| line.last())
            .is_some_and(|previous| {
                let line_delta = (glyph.y - previous.y).abs();
                line_delta > glyph.font_size.max(previous.font_size) * 0.5
                    || (glyph.x < previous.x
                        && line_delta > glyph.font_size.max(previous.font_size) * 0.2)
            });
        if lines.is_empty() || starts_line {
            lines.push(Vec::new());
        }
        lines.last_mut().unwrap().push(glyph);
    }

    let body_font_size =
        median(glyphs.iter().map(|glyph| glyph.font_size).collect()).unwrap_or_default();
    let mut rendered = String::new();
    let mut previous: Option<(&[&PositionedGlyph], f64, bool)> = None;
    for line in &lines {
        let line_y = median(line.iter().map(|glyph| glyph.y).collect()).unwrap_or_default();
        let line_font_size =
            median(line.iter().map(|glyph| glyph.font_size).collect()).unwrap_or_default();
        let is_heading = body_font_size > 0.0 && line_font_size >= body_font_size * 1.18;
        if let Some((previous_line, previous_y, previous_is_heading)) = previous {
            let previous_font_size =
                median(previous_line.iter().map(|glyph| glyph.font_size).collect())
                    .unwrap_or_default();
            let line_height = line_font_size.max(previous_font_size);
            let paragraph_gap = (line_y - previous_y).abs() > line_height * 1.55;
            rendered.push_str(if paragraph_gap || is_heading || previous_is_heading {
                "\n\n"
            } else {
                "\n"
            });
        }
        rendered.push_str(&render_positioned_line(line));
        previous = Some((line, line_y, is_heading));
    }
    rendered
}

fn render_positioned_line(glyphs: &[&PositionedGlyph]) -> String {
    let advance_scale = median(
        glyphs
            .windows(2)
            .filter_map(|pair| {
                let glyph = pair[0];
                let next = pair[1];
                let follows_text = !glyph.text.chars().last().is_some_and(char::is_whitespace)
                    && !next.text.chars().next().is_some_and(char::is_whitespace);
                let distance = next.x - glyph.x;
                (follows_text
                    && glyph.advance.is_finite()
                    && glyph.advance > 0.0
                    && distance > 0.0
                    && distance < glyph.advance * 1.8)
                    .then_some(distance / glyph.advance)
            })
            .collect(),
    )
    .unwrap_or(1.0);

    let mut text = String::new();
    for (index, glyph) in glyphs.iter().enumerate() {
        text.push_str(&glyph.text);
        let Some(next) = glyphs.get(index + 1) else {
            continue;
        };
        let follows_text = !glyph.text.chars().last().is_some_and(char::is_whitespace)
            && !next.text.chars().next().is_some_and(char::is_whitespace);
        let gap = next.x - glyph.x - glyph.advance * advance_scale;
        let punctuation_boundary = glyph
            .text
            .chars()
            .last()
            .is_some_and(|ch| "([{«“‘".contains(ch))
            || next
                .text
                .chars()
                .next()
                .is_some_and(|ch| ",.;:!?%)]}»”’".contains(ch));
        if follows_text && !punctuation_boundary && gap.is_finite() && gap > glyph.font_size * 0.1 {
            text.push(' ');
        }
    }
    text
}

fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    Some(values[values.len() / 2])
}

fn has_skladchik_footer(text: &str) -> bool {
    let normalized = normalize_signature(text);
    let Some(brand) = normalized.rfind("skladchik org") else {
        return false;
    };
    let footer = &normalized[brand..];
    footer.contains("что такое клуб складчик") && footer.ends_with("этот контент куплен на сайте")
}

fn has_skladchik_filename_marker(title: &str) -> bool {
    title.to_ascii_lowercase().contains("skladchik.org")
}

fn is_skladchik_pdf(title: &str, text: &str) -> bool {
    has_skladchik_filename_marker(title) || has_skladchik_footer(text)
}

fn normalize_signature(text: &str) -> String {
    let normalized = text
        .to_lowercase()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                ' '
            }
        })
        .collect::<String>();
    normalized.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Apply Skladchik cleanup to page-level text only when its footer signature is present.
pub fn cleanup_extracted_pages(pages: &[String]) -> Option<String> {
    has_skladchik_footer(&pages.join("\n")).then(|| clean_skladchik_pages(pages, true))
}

fn clean_skladchik_pages(pages: &[String], remove_footer: bool) -> String {
    let cleaned = pages
        .iter()
        .map(|page| {
            let mut lines = page.lines().collect::<Vec<_>>();
            while lines.first().is_some_and(|line| line.trim().is_empty()) {
                lines.remove(0);
            }
            while lines.last().is_some_and(|line| line.trim().is_empty()) {
                lines.pop();
            }
            if lines.first().is_some_and(|line| is_page_number(line)) {
                lines.remove(0);
            }
            if lines.last().is_some_and(|line| is_page_number(line)) {
                lines.pop();
            }

            let mut paragraphs = Vec::new();
            let mut paragraph = Vec::new();
            for line in lines {
                let line = line.trim();
                if line.is_empty() {
                    if !paragraph.is_empty() {
                        paragraphs.push(paragraph.join(" "));
                        paragraph.clear();
                    }
                } else {
                    paragraph.push(line);
                }
            }
            if !paragraph.is_empty() {
                paragraphs.push(paragraph.join(" "));
            }
            paragraphs.join("\n\n")
        })
        .filter(|page| !page.is_empty())
        .collect::<Vec<_>>();

    let text = cleaned.join("\u{c}");
    if remove_footer {
        let lower = text.to_ascii_lowercase();
        lower
            .rfind("skladchik.org")
            .map_or(text.as_str(), |start| &text[..start])
            .trim()
            .to_owned()
    } else {
        text.trim().to_owned()
    }
}

fn is_page_number(line: &str) -> bool {
    !line.trim().is_empty() && line.trim().parse::<u32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdf_extract::OutputDev;

    const MARAHOVSKY_FOOTER: &str = "\
        article body\n\n\
        skladchik.ORG\n\
        Ты еще не в Клубе?\n\n\
        Что такое Клуб «Складчик»?\n\
        Этот контент\n\
        куплен на сайте\n";
    const SNMI_FOOTER: &str = "\
        article body\n\n\
        SKLADCHIK.org\n\
        Что такое Клуб «Складчик»?\n\n\
        Этот контент куплен на сайте\n";
    const CRIMSON_FOOTER: &str = "\
        article body\n\n\
        skladchik.ORG\n\
        Что такое Клуб «Складчик»?\n\
        Присоединяйся и начинай экономить!\n\n\
        Этот контент\n\
        куплен на сайте\n";

    fn sample_pdf() -> Vec<u8> {
        let content = "BT /F1 12 Tf 50 100 Td (Yandex document text.) Tj ET\n";
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 300 144] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_owned(),
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_owned(),
            format!("<< /Length {} >>\nstream\n{content}endstream", content.len()),
        ];
        let mut pdf = String::from("%PDF-1.4\n");
        let mut offsets = vec![0usize];
        for (index, object) in objects.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.push_str(&format!("{} 0 obj\n{object}\nendobj\n", index + 1));
        }
        let xref = pdf.len();
        pdf.push_str(&format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len()));
        for offset in offsets.iter().skip(1) {
            pdf.push_str(&format!("{offset:010} 00000 n \n"));
        }
        pdf.push_str(&format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF",
            offsets.len()
        ));
        pdf.into_bytes()
    }

    fn emit_glyph(output: &mut PositionedTextOutput, x: f64, y: f64, width: f64, text: &str) {
        emit_glyph_at_size(output, x, y, width, 10.0, text);
    }

    fn emit_glyph_at_size(
        output: &mut PositionedTextOutput,
        x: f64,
        y: f64,
        width: f64,
        font_size: f64,
        text: &str,
    ) {
        let transform = pdf_extract::Transform::create_translation(x, y);
        output
            .output_character(&transform, width, 0.0, font_size, text)
            .unwrap();
    }

    #[test]
    fn malformed_pdf_bytes_are_a_clean_failure() {
        assert_eq!(markdown(b"not a PDF", "sample.pdf"), None);
    }

    #[test]
    fn extracts_text_from_a_public_pdf() {
        assert_eq!(
            markdown(&sample_pdf(), "research.pdf").as_deref(),
            Some("# research.pdf\n\nYandex document text.\n")
        );
    }

    #[test]
    fn recognizes_the_combined_skladchik_footer_in_supplied_pdf_samples() {
        for sample in [MARAHOVSKY_FOOTER, SNMI_FOOTER, CRIMSON_FOOTER] {
            assert!(has_skladchik_footer(sample));
        }
    }

    #[test]
    fn identifies_skladchik_pdfs_from_either_filename_or_footer() {
        assert!(has_skladchik_filename_marker(
            "2026.10.26 [Новостной деск] [skladchik.org].pdf"
        ));
        assert!(has_skladchik_filename_marker("article [SKLADCHIK.ORG].PDF"));
        assert!(!has_skladchik_filename_marker("article.pdf"));

        assert!(is_skladchik_pdf("article.pdf", SNMI_FOOTER));
        assert!(!is_skladchik_pdf(
            "article.pdf",
            "ordinary PDF article text"
        ));
        assert!(!is_skladchik_pdf(
            "article.pdf",
            "Article cites skladchik.org, but has no purchase footer."
        ));
    }

    #[test]
    fn checks_the_purchase_ad_before_footer_cleanup_removes_its_signal() {
        let raw = SNMI_FOOTER.to_owned();
        assert!(is_skladchik_pdf("opaque-name.pdf", &raw));

        let cleaned = cleanup_extracted_pages(&[raw]).unwrap();
        assert_eq!(cleaned, "article body");
        assert!(!is_skladchik_pdf("opaque-name.pdf", &cleaned));
    }

    #[test]
    fn page_aware_cleanup_requires_the_complete_signature() {
        let pages = [MARAHOVSKY_FOOTER.to_owned()];
        assert_eq!(
            cleanup_extracted_pages(&pages).as_deref(),
            Some("article body")
        );

        let ordinary = "ordinary PDF with a numbered section 1";
        assert_eq!(cleanup_extracted_pages(&[ordinary.to_owned()]), None);
    }

    #[test]
    fn does_not_recognize_an_isolated_brand_or_purchase_notice() {
        assert!(!has_skladchik_footer(
            "Article mentions skladchik.org here."
        ));
        assert!(!has_skladchik_footer(
            "Article ends: этот контент куплен на сайте"
        ));
        assert!(!has_skladchik_footer(
            "skladchik.org is cited in article text, followed by an ordinary ending."
        ));
        assert!(!has_skladchik_footer(
            "skladchik.org. Что такое Клуб «Складчик»? Article continues after the quote."
        ));
    }

    #[test]
    fn cleanup_removes_edge_numbers_and_the_complete_footer_but_keeps_body_numbers() {
        let pages = [
            "1\nA numbered section 1.\nA wrapped\nparagraph.\n\nBody continues.".to_owned(),
            "2\nNext page starts.\n\n skladchik.ORG\nAd\nЭтот контент\nкуплен на сайте".to_owned(),
        ];
        assert_eq!(
            clean_skladchik_pages(&pages, true),
            "A numbered section 1. A wrapped paragraph.\n\nBody continues.\u{c}Next page starts."
        );
    }

    #[test]
    fn page_number_detection_only_accepts_standalone_integers() {
        assert!(is_page_number(" 12 "));
        assert!(!is_page_number("Section 12"));
        assert!(!is_page_number("1. Numbered item"));
    }

    #[test]
    fn positioned_extraction_recovers_gaps_without_splitting_tight_glyphs() {
        let mut output = PositionedTextOutput::default();
        let media_box = pdf_extract::MediaBox {
            llx: 0.0,
            lly: 0.0,
            urx: 300.0,
            ury: 100.0,
        };
        output.begin_page(1, &media_box, None).unwrap();

        emit_glyph(&mut output, 10.0, 20.0, 0.5, "A");
        emit_glyph(&mut output, 15.0, 20.0, 0.5, "d");
        emit_glyph(&mut output, 20.0, 20.0, 0.5, "e");
        emit_glyph(&mut output, 29.0, 20.0, 0.5, "B");
        emit_glyph(&mut output, 34.0, 20.0, 0.5, "o");
        emit_glyph(&mut output, 39.0, 20.0, 0.25, " ");
        emit_glyph(&mut output, 41.5, 20.0, 0.5, "C");
        emit_glyph(&mut output, 46.0, 20.0, 0.5, "D");
        emit_glyph(&mut output, 0.0, 0.0, 0.5, "E");
        output.end_page().unwrap();

        output.begin_page(2, &media_box, None).unwrap();
        emit_glyph(&mut output, 10.0, 20.0, 0.5, "F");
        output.end_page().unwrap();

        assert_eq!(output.pages, ["Ade Bo CD\n\nE", "F"]);
    }

    #[test]
    fn positioned_extraction_preserves_paragraph_gaps_and_heading_padding() {
        let mut output = PositionedTextOutput::default();
        let media_box = pdf_extract::MediaBox {
            llx: 0.0,
            lly: 0.0,
            urx: 300.0,
            ury: 100.0,
        };
        output.begin_page(1, &media_box, None).unwrap();

        for (y, size, line) in [
            (20.0, 10.0, "Opening paragraph"),
            (30.0, 10.0, "continues here"),
            (50.0, 15.0, "Section heading"),
            (62.0, 10.0, "Section text"),
        ] {
            for (index, character) in line.chars().enumerate() {
                emit_glyph_at_size(
                    &mut output,
                    10.0 + index as f64 * size * 0.5,
                    y,
                    0.5,
                    size,
                    &character.to_string(),
                );
            }
        }
        output.end_page().unwrap();

        assert_eq!(
            output.pages[0],
            "Opening paragraph\ncontinues here\n\nSection heading\n\nSection text"
        );
        assert_eq!(
            clean_skladchik_pages(&output.pages, false),
            "Opening paragraph continues here\n\nSection heading\n\nSection text"
        );
    }

    #[test]
    fn skladchik_pages_join_without_visible_page_break_rules() {
        let pages = ["First page", "Second page"];
        assert_eq!(join_pages(&pages, true), "First page\nSecond page");
        assert_eq!(
            join_pages(&pages, false),
            "First page\n\n---\n\nSecond page"
        );
    }

    #[test]
    fn positioned_skladchik_text_keeps_page_edge_cleanup() {
        let mut output = PositionedTextOutput::default();
        let media_box = pdf_extract::MediaBox {
            llx: 0.0,
            lly: 0.0,
            urx: 300.0,
            ury: 100.0,
        };
        output.begin_page(1, &media_box, None).unwrap();
        for (y, line) in [
            (10.0, "1"),
            (20.0, "Article body"),
            (
                80.0,
                "skladchik.org что такое Клуб Складчик этот контент куплен на сайте",
            ),
        ] {
            for (index, character) in line.chars().enumerate() {
                emit_glyph(
                    &mut output,
                    10.0 + index as f64 * 5.0,
                    y,
                    0.5,
                    &character.to_string(),
                );
            }
        }
        output.end_page().unwrap();

        assert_eq!(
            output.pages,
            [
                "1\nArticle body\n\nskladchik.org что такое Клуб Складчик этот контент куплен на сайте"
            ]
        );
        assert_eq!(clean_skladchik_pages(&output.pages, true), "Article body");
    }
}
