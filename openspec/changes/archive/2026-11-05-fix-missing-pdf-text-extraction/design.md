# Design

## Context

See proposal.md for the motivation. The screenshot confirms the definition is visible on page 4, while general and positioned PDF text extraction produce no characters in its area. This indicates the definition is represented as page artwork rather than a text span that the current extractor can return. Brevier's default path must remain model-free; OCR belongs in a separate module invoked by a page-level reader action.

## Goals / Non-Goals

**Goals:**
- Recover legible text rendered as PDF page artwork when the reader opts in to OCR.
- Preserve native PDF text and its order while adding OCR-recognized content without duplicates.
- Extract every page with the existing method first; keep OCR runtime/model initialization out of that default path.
- Let readers run OCR on a selected page, retain its regular result for switching back, and report the active loading stage.
- Support the shared Cloud Mail and Yandex Disk PDF path and the Android reader.

**Non-Goals:**
- Running OCR or loading an OCR model before the reader selects it for a page.
- Sending PDF pages to a remote OCR service.
- Replacing native PDF text extraction with OCR when native text is available.
- Changing text extraction for non-PDF pages.

## Decisions

- **Make OCR a per-page action in the dots menu.** Remove the global OCR preference. Opening or reloading a PDF always uses the existing PDF extraction method first. The dots popup menu offers “Recognize PDF text (OCR)” for a regular PDF and “Use regular PDF text” while its OCR result is displayed; neither item appears for non-PDF pages. Remove the inline article button. Keep the regular result alongside the OCR result for the currently loaded document so the reader can restore it immediately; reloading returns pages to regular extraction until OCR is selected again. Do not initialize the OCR runtime or models during regular extraction.
- **Render pages with PDFium.** Use `pdfium-render` for local page rasterization on Android and Linux, packaging the required native PDFium library for each supported target. The wrapper does not include that library, so its ABI coverage and APK footprint must be checked before release.
- **Recognize text with Rust bindings to Tesseract.** Use `xberg-tesseract`'s Rust API and its static Tesseract/Leptonica build for Linux and Android NDK targets. Its Android build path maps Rust targets to NDK ABIs and uses the configured NDK compiler. Run it locally on PDFium-rendered pages, with no network service. Embed both the Russian and English `tessdata_fast` models (Apache-2.0) in the optional OCR library and initialize them together as `rus+eng` only when OCR is requested. This supports articles that are mostly Russian but include English words. The binding exposes HOCR/TSV word boxes needed for positional merging. Verify both pinned model files, their license notices, the crate's Android build, offline runtime, and resulting package size; do not claim mixed-language recognition until it has been run against the supplied sample and a mixed-language page.
- **Observed Android footprint.** The pinned arm64 release APK built successfully at about 31 MiB with the Russian model embedded. Its stripped core was 23,083,712 bytes; `libpdfium.so` was 6,598,376 bytes. These are Russian-only baseline measurements; measure and record the incremental size from adding the English model. The Android build includes the Tesseract, Leptonica, PDFium and wrapper notices in app assets. OCR code and models are present in the APK; runtime/model initialization remains deferred until a page-level OCR action.
- **Use OCR to fill uncovered page regions.** Retain positioned native text spans and OCR word/line bounding boxes. Merge in page reading order, rejecting OCR spans that overlap native text so hybrid pages do not repeat their content. The supplied definition should be recognized from its artwork and placed between its introduction and the next native paragraph. Keep each page's regular result available while OCR is displayed, so switching back does not require another network fetch.
- **Report loading stages.** Surface the current stage for a load or page reprocessing: network fetching, regular text extraction, or OCR text extraction. Reload begins with network fetching and regular extraction; OCR extraction is shown only after the reader requests OCR for a page.
- **Mark recognized content.** Add a visible marker around OCR-derived spans so readers can distinguish recognition results from native PDF text and judge possible recognition errors.
- **Keep provider handling shared.** Pass the reader's OCR choice to the common PDF renderer, not separate Cloud Mail or Yandex Disk implementations.

## Risks / Trade-offs

### Release startup crash

The signed, minified x86_64 diagnostic release reproduced the crash on Android 31: `libbrevier.so` required `libc++_shared.so`, but the APK omitted it, causing `UnsatisfiedLinkError` while `Core` loaded from `App.onCreate`. The Android build now packages the NDK's matching C++ runtime for each ABI, and a Gradle verification task checks the final release APK.

The signed ARM64 release was rebuilt with the runtime included, passed APK v2 signature verification, and cold-started successfully on the Android 31 x86_64 emulator using its ARM64 translation layer. Cold start passed with OCR off and with the persisted OCR preference enabled; the app process remained alive and logcat showed no fatal or native crash.

Release-only startup failures can bypass core tests. Keep the packaged-runtime assertion and cold-start the signed APK as a release gate.

- [OCR can misread words, punctuation, or layout] → Label OCR-derived text and test the supplied passage plus a page with mixed native and OCR content.
- [The English model adds package size and language configuration may affect predominantly Russian recognition] → Measure its incremental package cost and verify `rus+eng` against Russian text containing English words as well as the supplied Russian passage.
- [The OCR runtime and model can increase app size or complicate Android builds] → Keep OCR initialization behind the page-level action, pin the Rust binding and native source inputs, measure the footprint, and verify offline Android and Linux behavior.
- [Merging native and OCR spans can reorder or duplicate text] → Use page coordinates for ordering and overlap checks, with regression coverage for exact-once output.
- [PDF rasterization can consume memory on large pages] → Bound render dimensions and verify the OCR path remains within the reader's page-memory limits.

## Migration Plan

No stored data or API changes are required. OCR is selected per page for the currently loaded document and is not a global setting. Reverting the change removes the page controls and optional path.
