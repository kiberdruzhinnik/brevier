//! PDF text becomes Markdown locally; the parser is panic-contained because
//! downloaded documents are untrusted input.

/// Extract readable text from PDF bytes and title it with the shared filename.
pub fn markdown(bytes: &[u8], title: &str) -> Option<String> {
    let text = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pdf_extract::extract_text_from_mem(bytes)
    }))
    .ok()?
    .ok()?;
    let text = if has_skladchik_footer(&text) {
        let pages = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pdf_extract::extract_text_from_mem_by_pages(bytes)
        }));
        match pages {
            Ok(Ok(pages)) => cleanup_extracted_pages(&pages).unwrap_or(text),
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
        pages.join("\n\n---\n\n")
    ))
}

fn has_skladchik_footer(text: &str) -> bool {
    let normalized = normalize_signature(text);
    let Some(brand) = normalized.rfind("skladchik org") else {
        return false;
    };
    let footer = &normalized[brand..];
    footer.contains("что такое клуб складчик") && footer.ends_with("этот контент куплен на сайте")
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
    has_skladchik_footer(&pages.join("\n")).then(|| clean_skladchik_pages(pages))
}

fn clean_skladchik_pages(pages: &[String]) -> String {
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

    let text = cleaned.join("\n\n");
    let lower = text.to_ascii_lowercase();
    lower
        .rfind("skladchik.org")
        .map_or(text.as_str(), |start| &text[..start])
        .trim()
        .to_owned()
}

fn is_page_number(line: &str) -> bool {
    !line.trim().is_empty() && line.trim().parse::<u32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            clean_skladchik_pages(&pages),
            "A numbered section 1. A wrapped paragraph.\n\nBody continues.\n\nNext page starts."
        );
    }

    #[test]
    fn page_number_detection_only_accepts_standalone_integers() {
        assert!(is_page_number(" 12 "));
        assert!(!is_page_number("Section 12"));
        assert!(!is_page_number("1. Numbered item"));
    }
}
