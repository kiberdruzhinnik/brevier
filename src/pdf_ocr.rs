//! OCR for the explicit PDF option. Nothing here is touched by ordinary PDF reading.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

use pdfium_render::prelude::*;
use xberg_tesseract::{TessPageSegMode, TesseractAPI};

const RUSSIAN_MODEL: &[u8] = include_bytes!("../assets/tessdata/rus.traineddata");
const ENGLISH_MODEL: &[u8] = include_bytes!("../assets/tessdata/eng.traineddata");
const MAX_PAGE_DIMENSION: i32 = 1800;
static MODEL_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct ModelDirectory(PathBuf);

impl ModelDirectory {
    fn create(parent: Option<&Path>) -> Option<Self> {
        let fallback = std::env::temp_dir();
        let parent = parent.unwrap_or(&fallback);
        fs::create_dir_all(parent).ok()?;
        for _ in 0..10 {
            let id = MODEL_DIR_ID.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!("brevier-pdf-ocr-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => {
                    let models = Self(path);
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        fs::set_permissions(&models.0, fs::Permissions::from_mode(0o700)).ok()?;
                    }
                    fs::write(models.0.join("rus.traineddata"), RUSSIAN_MODEL).ok()?;
                    fs::write(models.0.join("eng.traineddata"), ENGLISH_MODEL).ok()?;
                    return Some(models);
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(_) => return None,
            }
        }
        None
    }
}

impl Drop for ModelDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OcrWord {
    pub(crate) text: String,
    pub(crate) top: f64,
    pub(crate) left: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) line: (i32, i32, i32),
}

fn pdfium() -> Option<&'static Pdfium> {
    static PDFIUM: OnceLock<Option<Pdfium>> = OnceLock::new();
    PDFIUM
        .get_or_init(|| {
            let bindings = std::env::var_os("BREVIER_PDFIUM_LIBRARY")
                .map(Pdfium::bind_to_library)
                .unwrap_or_else(Pdfium::bind_to_system_library)
                .ok()?;
            Some(Pdfium::new(bindings))
        })
        .as_ref()
}

pub(crate) fn recognize(bytes: &[u8], model_parent: Option<&Path>) -> Option<Vec<Vec<OcrWord>>> {
    // Both libraries are loaded only after the reader enabled the preference.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        recognize_inner(bytes, model_parent)
    }))
    .ok()
    .flatten()
}

fn recognize_inner(bytes: &[u8], model_parent: Option<&Path>) -> Option<Vec<Vec<OcrWord>>> {
    let pdfium = pdfium()?;
    let document = pdfium.load_pdf_from_byte_vec(bytes.to_vec(), None).ok()?;
    let models = ModelDirectory::create(model_parent)?;
    let api = tesseract(&models)?;

    let mut pages = Vec::with_capacity(document.pages().len() as usize);
    for page in document.pages().iter() {
        let image = page
            .render_with_config(
                &PdfRenderConfig::new()
                    .set_maximum_width(MAX_PAGE_DIMENSION)
                    .set_maximum_height(MAX_PAGE_DIMENSION),
            )
            .ok()?
            .as_image()
            .ok()?
            .to_luma8();
        pages.push(recognize_luma(
            &api,
            image.as_raw(),
            image.width(),
            image.height(),
        )?);
    }
    Some(pages)
}

fn tesseract(models: &ModelDirectory) -> Option<TesseractAPI> {
    let api = TesseractAPI::new().ok()?;
    api.init_4(models.0.to_str()?, "rus+eng", 3, &[]).ok()?;
    api.set_page_seg_mode(TessPageSegMode::PSM_AUTO).ok()?;
    Some(api)
}

fn recognize_luma(
    api: &TesseractAPI,
    pixels: &[u8],
    width: u32,
    height: u32,
) -> Option<Vec<OcrWord>> {
    api.set_image(
        pixels,
        i32::try_from(width).ok()?,
        i32::try_from(height).ok()?,
        1,
        i32::try_from(width).ok()?,
    )
    .ok()?;
    api.recognize().ok()?;
    Some(parse_tsv(&api.get_tsv_text(0).ok()?, width, height))
}

fn parse_tsv(tsv: &str, page_width: u32, page_height: u32) -> Vec<OcrWord> {
    let mut words = Vec::new();
    for row in tsv.lines().skip(1) {
        let columns = row.splitn(12, '\t').collect::<Vec<_>>();
        if columns.len() < 12 || columns[0] != "5" {
            continue;
        }
        let (Ok(block), Ok(paragraph), Ok(line), Ok(left), Ok(top), Ok(width), Ok(height)) = (
            columns[2].parse::<i32>(),
            columns[3].parse::<i32>(),
            columns[4].parse::<i32>(),
            columns[6].parse::<i32>(),
            columns[7].parse::<i32>(),
            columns[8].parse::<i32>(),
            columns[9].parse::<i32>(),
        ) else {
            continue;
        };
        if columns[10].parse::<f32>().unwrap_or_default() < 20.0 {
            continue;
        }
        let word = columns[11].trim();
        if word.is_empty() || width <= 0 || height <= 0 {
            continue;
        }
        words.push(OcrWord {
            text: word.to_owned(),
            top: top as f64 / page_height.max(1) as f64,
            left: left as f64 / page_width.max(1) as f64,
            width: width as f64 / page_width.max(1) as f64,
            height: height as f64 / page_height.max(1) as f64,
            line: (block, paragraph, line),
        });
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_languages_recognize_russian_with_english_words() {
        let models = ModelDirectory::create(None).expect("embedded OCR models are writable");
        let api = tesseract(&models).expect("Russian and English models initialize together");
        let pgm = include_bytes!("../tests/fixtures/ocr-rus-eng.pgm");
        let (width, height, pixels) = pgm_raster(pgm);
        let words = recognize_luma(&api, pixels, width, height).expect("recognition succeeds");
        let text = words
            .iter()
            .map(|word| word.text.to_lowercase())
            .collect::<Vec<_>>();
        assert_eq!(
            text,
            [
                "старые",
                "деньги",
                "обозначают",
                "состояние",
                "семьи,",
                "передаваемое",
                "из",
                "поколения",
                "в",
                "поколение",
                "на",
                "протяжении",
                "длительного",
                "времени,",
                "в",
                "инвестиционных",
                "статьях",
                "old",
                "money",
                "и",
                "etf",
                "встречаются",
                "редко.",
            ]
        );
    }

    fn pgm_raster(pgm: &[u8]) -> (u32, u32, &[u8]) {
        let mut offset = 0;
        let mut tokens = Vec::new();
        while tokens.len() < 4 {
            while pgm.get(offset).is_some_and(u8::is_ascii_whitespace) {
                offset += 1;
            }
            let start = offset;
            while pgm
                .get(offset)
                .is_some_and(|byte| !byte.is_ascii_whitespace())
            {
                offset += 1;
            }
            tokens.push(std::str::from_utf8(&pgm[start..offset]).unwrap());
        }
        assert_eq!(tokens[0], "P5");
        assert_eq!(tokens[3], "255");
        offset += 1;
        let width = tokens[1].parse().unwrap();
        let height = tokens[2].parse().unwrap();
        (width, height, &pgm[offset..])
    }
}
