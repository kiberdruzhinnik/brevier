//! PDF text becomes Markdown locally; the parser is panic-contained because
//! downloaded documents are untrusted input.

/// Extract readable text from PDF bytes and title it with the shared filename.
pub fn markdown(bytes: &[u8], title: &str) -> Option<String> {
    markdown_with_ocr(bytes, title, false)
}

/// OCR preference is carried here so both public-share providers share one path.
/// The enabled branch is implemented with the renderer and recognizer below.
pub fn markdown_with_ocr(bytes: &[u8], title: &str, _ocr_enabled: bool) -> Option<String> {
    markdown_with_ocr_model_dir(bytes, title, _ocr_enabled, None)
}

/// Variant for Android, where the caller supplies its private cache directory
/// for Tesseract's temporary language files.
pub(crate) fn markdown_with_ocr_model_dir(
    bytes: &[u8],
    title: &str,
    ocr_enabled: bool,
    model_parent: Option<&std::path::Path>,
) -> Option<String> {
    #[cfg(not(feature = "pdf-ocr"))]
    let _ = (ocr_enabled, model_parent);

    let text = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pdf_extract::extract_text_from_mem(bytes)
    }))
    .ok()?
    .ok()?;
    let skladchik = is_skladchik_pdf(title, &text);
    let has_footer = has_skladchik_footer(&text);
    let text = if skladchik {
        let pages = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            extract_skladchik_pages(bytes)
        }));
        match pages {
            Ok(Some(pages)) if !pages.is_empty() => clean_skladchik_pages(&pages, has_footer),
            _ => {
                let pages = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    pdf_extract::extract_text_from_mem_by_pages(bytes)
                }));
                match pages {
                    Ok(Ok(pages)) => clean_skladchik_pages(&pages, has_footer),
                    _ => remove_crimson_analytics_insertion(&text, skladchik),
                }
            }
        }
    } else {
        text
    };
    let mut pages = text
        .split('\u{c}')
        .map(str::trim)
        .filter(|page| !page.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    #[cfg(feature = "pdf-ocr")]
    if ocr_enabled
        && let Some(recognized) = crate::pdf_ocr::recognize(bytes, model_parent)
        && let Some((positioned, widths, heights)) = extract_positioned_layout(bytes)
    {
        let merged = positioned
            .iter()
            .enumerate()
            .map(|(index, glyphs)| {
                let ocr = recognized.get(index).map(Vec::as_slice).unwrap_or_default();
                let width = widths.get(index).copied().unwrap_or_default();
                let height = heights.get(index).copied().unwrap_or_default();
                render_positioned_page_with_ocr(
                    glyphs,
                    index == 0,
                    width,
                    height,
                    ocr,
                    pages.get(index).map(String::as_str).unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>();
        if skladchik {
            let merged = clean_skladchik_pages(&merged, has_footer);
            pages = merged
                .split('\u{c}')
                .map(str::trim)
                .filter(|page| !page.is_empty())
                .map(str::to_owned)
                .collect();
        } else {
            pages = merged
                .into_iter()
                .filter(|page| !page.trim().is_empty())
                .collect();
        }
    }
    omit_skladchik_ad_cover(&mut pages);
    if pages.is_empty() {
        return None;
    }
    let pages = pages.iter().map(String::as_str).collect::<Vec<_>>();
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

#[cfg(feature = "pdf-ocr")]
type PositionedLayout = (Vec<Vec<PositionedGlyph>>, Vec<f64>, Vec<f64>);
#[cfg(feature = "pdf-ocr")]
type PositionedOcrRows = Vec<((i32, i32, i32), f64, Vec<crate::pdf_ocr::OcrWord>)>;

#[cfg(feature = "pdf-ocr")]
fn extract_positioned_layout(bytes: &[u8]) -> Option<PositionedLayout> {
    let document = pdf_extract::Document::load_mem(bytes).ok()?;
    let mut output = PositionedTextOutput::default();
    pdf_extract::output_doc(&document, &mut output).ok()?;
    Some((
        output.positioned_pages,
        output.page_widths,
        output.page_heights,
    ))
}

#[derive(Clone)]
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
    positioned_pages: Vec<Vec<PositionedGlyph>>,
    page_widths: Vec<f64>,
    page_heights: Vec<f64>,
    flip_ctm: Option<pdf_extract::Transform>,
    page_num: u32,
}

impl pdf_extract::OutputDev for PositionedTextOutput {
    fn begin_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }

    fn begin_page(
        &mut self,
        page_num: u32,
        media_box: &pdf_extract::MediaBox,
        _art_box: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), pdf_extract::OutputError> {
        self.glyphs.clear();
        self.page_num = page_num;
        self.flip_ctm = Some(pdf_extract::Transform::row_major(
            1.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            media_box.ury - media_box.lly,
        ));
        self.page_heights.push(media_box.ury - media_box.lly);
        self.page_widths.push(media_box.urx - media_box.llx);
        Ok(())
    }

    fn end_page(&mut self) -> Result<(), pdf_extract::OutputError> {
        self.flip_ctm = None;
        self.pages
            .push(render_positioned_page(&self.glyphs, self.page_num == 1));
        self.positioned_pages.push(self.glyphs.clone());
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

fn render_positioned_page(glyphs: &[PositionedGlyph], join_initial_headline: bool) -> String {
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
    let line_layout = lines
        .iter()
        .map(|line| {
            (
                median(line.iter().map(|glyph| glyph.y).collect()).unwrap_or_default(),
                median(line.iter().map(|glyph| glyph.font_size).collect()).unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();
    let heading_lines = line_layout
        .iter()
        .map(|(_, font_size)| body_font_size > 0.0 && *font_size >= body_font_size * 1.18)
        .collect::<Vec<_>>();
    let headline = if join_initial_headline {
        heading_lines
            .iter()
            .position(|is_heading| *is_heading)
            .map(|start| {
                let mut end = start + 1;
                while end < lines.len() && heading_lines[end] {
                    let previous_font_size = line_layout[end - 1].1;
                    let line_height = line_layout[end].1.max(previous_font_size);
                    let line_gap = (line_layout[end].0 - line_layout[end - 1].0).abs();
                    if line_gap > line_height * 1.55 {
                        break;
                    }
                    end += 1;
                }
                start..end
            })
    } else {
        None
    };
    let mut rendered = String::new();
    let mut previous: Option<(usize, f64, bool)> = None;
    for (index, line) in lines.iter().enumerate() {
        let (line_y, line_font_size) = line_layout[index];
        let is_heading = heading_lines[index];
        if let Some((previous_index, previous_y, previous_is_heading)) = previous {
            let previous_font_size = line_layout[previous_index].1;
            let line_height = line_font_size.max(previous_font_size);
            let paragraph_gap = (line_y - previous_y).abs() > line_height * 1.55;
            let part_of_initial_headline = headline
                .as_ref()
                .is_some_and(|range| range.contains(&previous_index) && range.contains(&index));
            if part_of_initial_headline {
                rendered.push(' ');
            } else {
                rendered.push_str(if paragraph_gap || is_heading || previous_is_heading {
                    "\n\n"
                } else {
                    "\n"
                });
            }
        }
        rendered.push_str(&render_positioned_line(line));
        previous = Some((index, line_y, is_heading));
    }
    rendered
}

#[cfg(feature = "pdf-ocr")]
fn render_positioned_page_with_ocr(
    glyphs: &[PositionedGlyph],
    join_initial_headline: bool,
    page_width: f64,
    page_height: f64,
    ocr: &[crate::pdf_ocr::OcrWord],
    native_text: &str,
) -> String {
    let mut native_lines: Vec<Vec<&PositionedGlyph>> = Vec::new();
    for glyph in glyphs {
        let starts_line = native_lines
            .last()
            .and_then(|line| line.last())
            .is_some_and(|previous| {
                let line_delta = (glyph.y - previous.y).abs();
                line_delta > glyph.font_size.max(previous.font_size) * 0.5
                    || (glyph.x < previous.x
                        && line_delta > glyph.font_size.max(previous.font_size) * 0.2)
            });
        if native_lines.is_empty() || starts_line {
            native_lines.push(Vec::new());
        }
        native_lines.last_mut().unwrap().push(glyph);
    }
    let body_size = median(glyphs.iter().map(|glyph| glyph.font_size).collect()).unwrap_or(10.0);
    let mut rows = native_lines
        .iter()
        .map(|line| {
            (
                median(line.iter().map(|glyph| glyph.y).collect()).unwrap_or_default(),
                median(line.iter().map(|glyph| glyph.font_size).collect()).unwrap_or(body_size),
                render_positioned_line(line),
                false,
            )
        })
        .collect::<Vec<_>>();
    let native_words = normalize_signature(native_text)
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let native_compact = native_words.concat();
    let mut ocr_rows = PositionedOcrRows::new();
    for word in ocr {
        let y0 = word.top * page_height;
        if let Some((_, _, words)) = ocr_rows.iter_mut().find(|(line, _, _)| *line == word.line) {
            words.push(word.clone());
        } else {
            ocr_rows.push((word.line, y0, vec![word.clone()]));
        }
    }
    for (_, y, mut words) in ocr_rows {
        words.sort_by(|left, right| (left.left * page_width).total_cmp(&(right.left * page_width)));
        let mut text_words = words
            .iter()
            .filter(|word| !covered_by_native_glyphs(word, glyphs, page_width, page_height))
            .map(|word| word.text.clone())
            .collect::<Vec<_>>();
        let shared_prefix = (2..=text_words.len())
            .rev()
            .find(|length| {
                let prefix = text_words[..*length]
                    .iter()
                    .map(|word| normalize_signature(word).replace(' ', ""))
                    .collect::<String>();
                native_compact.contains(&prefix)
            })
            .unwrap_or(0);
        text_words.drain(..shared_prefix);
        let mut ocr_words = Vec::new();
        let mut word_ranges = Vec::with_capacity(text_words.len());
        for word in &text_words {
            let start = ocr_words.len();
            ocr_words.extend(
                normalize_signature(word)
                    .split_whitespace()
                    .map(str::to_owned),
            );
            word_ranges.push(start..ocr_words.len());
        }
        let overlaps = overlapping_ocr_words(&ocr_words, &native_words);
        text_words = text_words
            .into_iter()
            .zip(word_ranges)
            .filter_map(|(word, range)| {
                (!range.clone().all(|index| overlaps[index])).then_some(word)
            })
            .collect();
        let text = text_words.join(" ");
        if !text.is_empty() {
            rows.push((y + body_size * 0.8, body_size, text, true));
        }
    }
    rows.sort_by(|left, right| {
        left.0
            .total_cmp(&right.0)
            .then_with(|| left.3.cmp(&right.3))
    });

    let mut output = String::new();
    let mut previous: Option<(f64, f64, bool)> = None;
    let first_heading_size = rows
        .iter()
        .map(|row| row.1)
        .find(|size| *size >= body_size * 1.18);
    for (index, (y, size, text, is_ocr)) in rows.iter().enumerate() {
        if let Some((previous_y, previous_size, previous_ocr)) = previous {
            let gap = (y - previous_y).abs();
            let is_heading = *size >= body_size * 1.18;
            let previous_heading = previous_size >= body_size * 1.18;
            let join_headline = join_initial_headline
                && first_heading_size.is_some()
                && index == 1
                && is_heading
                && !is_ocr;
            output.push_str(
                if join_headline
                    || (*is_ocr && previous_ocr && gap <= size.max(previous_size) * 1.8)
                {
                    " "
                } else if gap > size.max(previous_size) * 1.55 || is_heading || previous_heading {
                    "\n\n"
                } else {
                    "\n"
                },
            );
        }
        if *is_ocr && !previous.is_some_and(|(_, _, was_ocr)| was_ocr) {
            output.push_str("[OCR] ");
        }
        output.push_str(text);
        previous = Some((*y, *size, *is_ocr));
    }
    output
}

#[cfg(feature = "pdf-ocr")]
fn overlapping_ocr_words(left: &[String], right: &[String]) -> Vec<bool> {
    let mut previous = vec![0; right.len() + 1];
    let mut current = vec![0; right.len() + 1];
    let mut overlapping = vec![false; left.len()];
    for (i, left_word) in left.iter().enumerate() {
        current[0] = 0;
        for (index, right_word) in right.iter().enumerate() {
            current[index + 1] = if ocr_words_match(left_word, right_word) {
                previous[index] + 1
            } else {
                0
            };
            let run = current[index + 1];
            if run >= 3 {
                overlapping[i + 1 - run..=i].fill(true);
            }
        }
        std::mem::swap(&mut previous, &mut current);
    }
    overlapping
}

#[cfg(feature = "pdf-ocr")]
fn ocr_words_match(left: &str, right: &str) -> bool {
    if left == right {
        return true;
    }
    let left = left.chars().collect::<Vec<_>>();
    let right = right.chars().collect::<Vec<_>>();
    let longest = left.len().max(right.len());
    if longest < 5 {
        return false;
    }
    let mut previous = (0..=right.len()).collect::<Vec<_>>();
    let mut current = vec![0; right.len() + 1];
    for (i, left_char) in left.iter().enumerate() {
        current[0] = i + 1;
        for (j, right_char) in right.iter().enumerate() {
            current[j + 1] = (previous[j + 1] + 1)
                .min(current[j] + 1)
                .min(previous[j] + usize::from(left_char != right_char));
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.len()] <= (longest / 8).max(1)
}

#[cfg(feature = "pdf-ocr")]
fn covered_by_native_glyphs(
    word: &crate::pdf_ocr::OcrWord,
    glyphs: &[PositionedGlyph],
    page_width: f64,
    page_height: f64,
) -> bool {
    let left = word.left * page_width;
    let right = left + word.width * page_width;
    let top = word.top * page_height;
    let bottom = top + word.height * page_height;
    let span = right - left;
    if span <= 0.0 {
        return false;
    }
    let covered = glyphs
        .iter()
        .filter(|glyph| {
            !glyph.text.trim().is_empty()
                && glyph.y >= top - glyph.font_size * 0.35
                && glyph.y <= bottom + glyph.font_size * 0.35
                && glyph.advance.is_finite()
                && glyph.advance > 0.0
        })
        .map(|glyph| {
            let glyph_left = glyph.x.max(left);
            let glyph_right = (glyph.x + glyph.advance).min(right);
            (glyph_right - glyph_left).max(0.0)
        })
        .sum::<f64>();
    covered / span >= 0.4
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
    has_skladchik_filename_marker(title)
        || has_skladchik_footer(text)
        || has_dated_article_series_signature(title, text)
}

fn has_dated_article_series_signature(title: &str, text: &str) -> bool {
    let Some((date, suffix)) = title.split_at_checked(8) else {
        return false;
    };
    if !date.bytes().all(|byte| byte.is_ascii_digit()) || !suffix.starts_with(" -") {
        return false;
    }

    let normalized = normalize_signature(text);
    let salutation = normalize_signature("ув. друзья");
    let author = normalize_signature("автор этих взволнованных строк");
    let normalized = format!(" {normalized} ");
    normalized.contains(&format!(" {salutation} ")) && normalized.contains(&format!(" {author} "))
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

fn is_skladchik_ad_cover(page: &str) -> bool {
    let normalized = normalize_signature(page);
    normalized.contains("куплено на skladchik org")
        && normalized.contains("все самые свежие курсы по лучшей цене")
}

fn omit_skladchik_ad_cover(pages: &mut Vec<String>) {
    if pages
        .first()
        .is_some_and(|page| is_skladchik_ad_cover(page))
    {
        pages.remove(0);
    }
}

/// Apply Skladchik cleanup to page-level text only when its footer signature is present.
pub fn cleanup_extracted_pages(pages: &[String]) -> Option<String> {
    has_skladchik_footer(&pages.join("\n")).then(|| clean_skladchik_pages(pages, true))
}

fn clean_skladchik_pages(pages: &[String], remove_footer: bool) -> String {
    let cleaned = pages
        .iter()
        .map(|page| {
            let page = remove_crimson_analytics_insertion(page, true);
            let mut lines = page.lines().collect::<Vec<_>>();
            while lines.first().is_some_and(|line| line.trim().is_empty()) {
                lines.remove(0);
            }
            while lines.last().is_some_and(|line| line.trim().is_empty()) {
                lines.pop();
            }
            while lines.first().is_some_and(|line| is_page_number(line)) {
                lines.remove(0);
            }
            while lines.last().is_some_and(|line| is_page_number(line)) {
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

fn remove_crimson_analytics_insertion(text: &str, is_skladchik: bool) -> String {
    if !is_skladchik {
        return text.to_owned();
    }

    let signature = normalize_signature("Кримсон Аналитика – CrimsonAnalytics");
    text.split('\u{c}')
        .map(|page| {
            let lines = page.lines().collect::<Vec<_>>();
            let mut cleaned = Vec::with_capacity(lines.len());
            let mut index = 0;
            while index < lines.len() {
                if lines[index].trim().is_empty() {
                    cleaned.push(lines[index]);
                    index += 1;
                    continue;
                }

                let mut matched = None;
                for end in index..(index + 3).min(lines.len()) {
                    if lines[end].trim().is_empty() {
                        break;
                    }
                    let candidate = normalize_signature(&lines[index..=end].join(" "));
                    if candidate == signature {
                        matched = Some(end + 1);
                        break;
                    }
                    if !signature.starts_with(&candidate) {
                        break;
                    }
                }

                if let Some(next) = matched {
                    index = next;
                } else {
                    cleaned.push(lines[index]);
                    index += 1;
                }
            }
            cleaned.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\u{c}")
}

fn is_page_number(line: &str) -> bool {
    !line.trim().is_empty() && line.trim().parse::<u32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdf_extract::OutputDev;

    #[test]
    fn disabled_ocr_keeps_existing_pdf_failure_behavior() {
        assert!(markdown_with_ocr(b"not a PDF", "sample.pdf", false).is_none());
    }

    #[cfg(feature = "pdf-ocr")]
    #[test]
    fn positioned_ocr_adds_only_uncovered_words_after_the_native_line() {
        let glyphs = vec![PositionedGlyph {
            x: 10.0,
            y: 100.0,
            font_size: 10.0,
            advance: 160.0,
            text: "Согласно определению Кембриджского словаря,".to_owned(),
        }];
        let words = [
            "Согласно",
            "определению",
            "Кембриджского",
            "словаря,",
            "термин",
            "«старые",
            "деньги»",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, text)| crate::pdf_ocr::OcrWord {
            text: text.to_owned(),
            top: 0.95,
            left: index as f64 * 0.08,
            width: 0.06,
            height: 0.04,
            line: (1, 1, 1),
        })
        .collect::<Vec<_>>();

        let native = "Согласно определению Кембриджского словаря,";
        let text = render_positioned_page_with_ocr(&glyphs, false, 500.0, 100.0, &words, native);
        let flattened = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(flattened.contains("Согласно определению Кембриджского словаря, [OCR] термин"));
        assert_eq!(text.matches("Согласно").count(), 1);
        assert_eq!(text.matches("[OCR]").count(), 1);
    }

    #[cfg(feature = "pdf-ocr")]
    #[test]
    fn positioned_ocr_drops_words_covered_by_native_glyphs() {
        let native =
            "Любопытство общественно поощряемая черта характера поэтому ей можно хвастаться";
        let glyphs = vec![PositionedGlyph {
            x: 10.0,
            y: 100.0,
            font_size: 10.0,
            advance: 480.0,
            text: native.to_owned(),
        }];
        let words = [
            "поошряемая",
            "черта",
            "характера",
            "поэтому",
            "ей",
            "можно",
            "хвастаться",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, text)| crate::pdf_ocr::OcrWord {
            text: text.to_owned(),
            top: 0.95,
            left: index as f64 * 0.08,
            width: 0.06,
            height: 0.04,
            line: (1, 1, 1),
        })
        .collect::<Vec<_>>();

        let text = render_positioned_page_with_ocr(&glyphs, false, 500.0, 100.0, &words, native);
        assert!(text.contains(native));
        assert_eq!(text.matches("[OCR]").count(), 0);
    }

    #[cfg(feature = "pdf-ocr")]
    #[test]
    fn ocr_runtime_failure_does_not_panic_or_replace_native_content() {
        assert!(crate::pdf_ocr::recognize(b"not a PDF", None).is_none());
        assert_eq!(
            markdown_with_ocr(b"not a PDF", "sample.pdf", true),
            markdown_with_ocr(b"not a PDF", "sample.pdf", false)
        );
    }

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

    fn emit_line_at_size(output: &mut PositionedTextOutput, y: f64, font_size: f64, line: &str) {
        for (index, character) in line.chars().enumerate() {
            emit_glyph_at_size(
                output,
                10.0 + index as f64 * font_size * 0.5,
                y,
                0.5,
                font_size,
                &character.to_string(),
            );
        }
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
    fn identifies_the_skladchik_ad_cover_from_its_purchase_notice_and_sales_slogan() {
        let supplied_cover = "куплено на\nskladchik.ORG\nВсе самые свежие\nкурсы по лучшей цене!";
        assert!(is_skladchik_ad_cover(supplied_cover));
        assert!(!is_skladchik_ad_cover("куплено на SKLADCHIK.ORG"));
        assert!(!is_skladchik_ad_cover(
            "В статье упомянуты skladchik.ORG и покупка на сайте."
        ));
        assert!(!is_skladchik_ad_cover(
            "Все самые свежие курсы по лучшей цене!"
        ));
    }

    #[test]
    fn omits_only_a_matching_first_page_and_preserves_article_pages() {
        let cover = "куплено на\nskladchik.ORG\nВсе самые свежие\nкурсы по лучшей цене!";
        let later_ad_like_text = cover.to_owned();
        let mut pages = vec![
            cover.to_owned(),
            "Статья начинается здесь; skladchik.ORG упоминается в тексте.".to_owned(),
            later_ad_like_text,
        ];

        omit_skladchik_ad_cover(&mut pages);

        assert_eq!(pages.len(), 2);
        assert!(pages[0].starts_with("Статья начинается здесь"));
        assert!(pages[1].starts_with("куплено на"));
    }

    #[test]
    fn preserves_the_first_page_when_only_the_purchase_phrase_matches() {
        let mut pages = vec!["куплено на SKLADCHIK.ORG".to_owned(), "Article".to_owned()];

        omit_skladchik_ad_cover(&mut pages);

        assert_eq!(pages, ["куплено на SKLADCHIK.ORG", "Article"]);
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
    fn identifies_the_dated_article_series_from_its_filename_and_text_signature() {
        let title = "20260413 - Робот-терапевт.pdf";
        let text = "Понедельник, УВ. друзья. Автор этих\nвзволнованных строк.";
        assert!(has_dated_article_series_signature(title, text));
        assert!(is_skladchik_pdf(title, text));

        assert!(!has_dated_article_series_signature(
            "20260413 - unrelated.pdf",
            "Ув. друзья, но нет второй авторской фразы."
        ));
        assert!(!has_dated_article_series_signature(
            "article.pdf",
            "Ув. друзья. Автор этих взволнованных строк."
        ));
        assert!(!has_dated_article_series_signature(
            "2026-04-13 - article.pdf",
            "Ув. друзья. Автор этих взволнованных строк."
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
            "1\nA numbered section 1.\n42\nA wrapped\nparagraph.\n\nBody continues.\n4".to_owned(),
            "2\nNext page starts.\n\n skladchik.ORG\nAd\nЭтот контент\nкуплен на сайте".to_owned(),
        ];
        assert_eq!(
            clean_skladchik_pages(&pages, true),
            "A numbered section 1. 42 A wrapped paragraph.\n\nBody continues.\u{c}Next page starts."
        );
    }

    #[test]
    fn cleanup_removes_multiple_edge_numbers_but_keeps_standalone_body_numbers() {
        let pages = ["1\n2\nArticle begins.\n\n42\n\nArticle ends.\n3\n4".to_owned()];
        assert_eq!(
            clean_skladchik_pages(&pages, false),
            "Article begins.\n\n42\n\nArticle ends."
        );
    }

    #[test]
    fn removes_inserted_crimson_analytics_text_from_skladchik_pages_only() {
        let excerpt = "причём эти сюжеты ещё и очень серьёзным образом\n\nКримсон Аналитика – CrimsonAnalytics\nпереплетены, причём в гораздо большей степени, чем это было раньше.";
        assert!(!is_skladchik_pdf("ordinary.pdf", excerpt));
        assert_eq!(
            remove_crimson_analytics_insertion(excerpt, true),
            "причём эти сюжеты ещё и очень серьёзным образом\n\nпереплетены, причём в гораздо большей степени, чем это было раньше."
        );
        assert_eq!(remove_crimson_analytics_insertion(excerpt, false), excerpt);
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
    fn positioned_extraction_joins_only_the_initial_wrapped_headline() {
        let mut output = PositionedTextOutput::default();
        let media_box = pdf_extract::MediaBox {
            llx: 0.0,
            lly: 0.0,
            urx: 800.0,
            ury: 300.0,
        };
        let headline = [
            "[Новостной деск] Аналитики Сороса",
            "рисуют кошмар Зеленского, огнемёт",
            "протащили на заседание Евросовета,",
            "британский мейнстрим готовится",
            "к размену украинских территорий",
        ];

        output.begin_page(1, &media_box, None).unwrap();
        emit_line_at_size(&mut output, 5.0, 10.0, "2 окт 19:14");
        for (index, line) in headline.iter().enumerate() {
            emit_line_at_size(&mut output, 20.0 + index as f64 * 12.0, 15.0, line);
        }
        for index in 0..8 {
            emit_line_at_size(
                &mut output,
                90.0 + index as f64 * 12.0,
                10.0,
                "Body paragraph text continues across the page for median font sizing.",
            );
        }
        emit_line_at_size(&mut output, 200.0, 15.0, "Later section heading");
        emit_line_at_size(&mut output, 220.0, 10.0, "Text after later heading.");
        output.end_page().unwrap();

        let page = &output.pages[0];
        let joined_headline = headline.join(" ");
        assert!(page.contains(&format!("{joined_headline}\n\nBody paragraph text")));
        assert!(!page.contains("Аналитики Сороса\n\nрисуют кошмар"));
        assert!(page.contains("Later section heading\n\nText after later heading."));

        output.begin_page(2, &media_box, None).unwrap();
        for (index, line) in headline.iter().enumerate() {
            emit_line_at_size(&mut output, 20.0 + index as f64 * 12.0, 15.0, line);
        }
        for index in 0..8 {
            emit_line_at_size(
                &mut output,
                90.0 + index as f64 * 12.0,
                10.0,
                "Body paragraph text continues across the page for median font sizing.",
            );
        }
        output.end_page().unwrap();
        assert!(output.pages[1].contains("Аналитики Сороса\n\nрисуют кошмар"));
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
