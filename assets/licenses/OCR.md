OCR components included in the Android reader:

- `tessdata/rus.traineddata` and `tessdata/eng.traineddata`: Russian and English
  `tessdata_fast` models from https://github.com/tesseract-ocr/tessdata_fast,
  Apache-2.0. They are embedded in the optional OCR library, written to a
  private temporary directory for Tesseract initialization, and removed when the
  OCR operation ends.
- `licenses/tesseract/`: MIT license for the `xberg-tesseract` Rust binding and
  upstream licenses for Tesseract and Leptonica.
- `pdfium-render-MIT.txt` and `pdfium-render-APACHE.txt`: the Rust PDFium
  wrapper license notices.
- `pdfium/`: PDFium binary distribution license and third-party notices,
  unpacked from the pinned `bblanchon/pdfium-binaries` release at build time.
