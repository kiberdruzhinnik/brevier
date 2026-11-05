# Proposal

## Why

The supplied PDF visibly contains the Cambridge Dictionary definition of “old money,” but Brevier's general and positioned PDF text extraction both omit it. The definition is rendered as page artwork rather than extracted characters, so text-layer recovery alone cannot preserve it.

After the signed ARM64 release APK was built, the user reported that the app repeatedly crashes on launch. The cause has not yet been diagnosed. Release startup stability therefore needs to be verified as part of this change.

## What Changes

- Add an optional, local OCR module that can recover readable text rendered as PDF page artwork, including articles that mix Russian text with English words.
- Remove the global OCR setting. Extract every PDF page with the existing method first; let the reader choose OCR for the current PDF from the dots popup menu and switch back to its retained regular result from that menu.
- Show the active loading stage while a page loads or is reprocessed: network fetching, regular text extraction, or OCR text extraction.
- Merge OCR text with native PDF text in page order, avoid duplicate text, and identify OCR-derived content in the rendered article.
- Add regression coverage for the supplied definition and for OCR-disabled behavior.
- Reproduce and diagnose the release startup crash, fix its root cause, and verify cold start of the signed release APK.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `public-cloud-shares`: Public Cloud Mail and Yandex Disk PDF reading can optionally recover visible text that is not represented in the PDF text layer.

## Impact

The shared PDF extraction path in `src/pdf.rs`, an optional OCR/rasterization module, per-page reader controls, loading-stage feedback, and Android release startup validation are in scope. The OCR runtime and Russian and English language models must work offline on Android and be initialized only when OCR is explicitly requested for a page. Both public share providers use the shared PDF path. The supplied PDF and screenshot are regression samples; follow repository corpus and licensing conventions when choosing what to retain.
