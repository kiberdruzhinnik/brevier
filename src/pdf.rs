//! PDF text becomes Markdown locally; the parser is panic-contained because
//! downloaded documents are untrusted input.

/// Extract readable text from PDF bytes and title it with the shared filename.
pub fn markdown(bytes: &[u8], title: &str) -> Option<String> {
    let text = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pdf_extract::extract_text_from_mem(bytes)
    }))
    .ok()?
    .ok()?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
