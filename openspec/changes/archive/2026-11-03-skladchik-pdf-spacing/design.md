# Design

## Context

See proposal.md for the reported extraction failure. `src/pdf.rs` currently asks `pdf-extract` for plain text, then uses page-level extraction for Skladchik footer cleanup. The library infers spaces from glyph positions with a fixed threshold; the supplied article shows that this can join words. The provider-reported title is already passed into the shared PDF reader. The PDF library exposes its parsed document and character-position output interface, so a Skladchik-only reader can use geometry without adding another PDF dependency.

## Goals / Non-Goals

**Goals:**
- Recover missing word boundaries for PDFs identified by a recognized Skladchik filename marker or by the complete purchase-footer signature in raw extracted text.
- Join text across PDF pages without emitting visible page-break markers such as `***`.
- Preserve heading and paragraph structure using readable blank-line separation after page joining.
- Feed the recovered text through existing Skladchik page-number and footer cleanup.
- Leave ordinary PDF extraction unchanged.

**Non-Goals:**
- Improve extraction for arbitrary PDFs or change provider share detection.
- Alter article content, punctuation, or language beyond spacing and existing Skladchik cleanup.
- Add JavaScript, cloud credentials, or a new PDF parser dependency.

## Decisions

### Identify the PDF before choosing spacing behavior

Treat the provider title and raw extracted text as independent Skladchik signals. A recognized filename marker selects the specialized path immediately. Otherwise, extract ordinary text first and check the combined footer signature before cleanup; if it matches, select the specialized path. Check the signature before the footer is removed. This preserves support for files whose names are opaque while covering the supplied file whose title contains `[skladchik.org]`.

### Recover spaces from positioned characters only on the specialized path

Use the PDF library's character output interface to compare glyph positions and infer word gaps for identified Skladchik PDFs. Keep the inference isolated from the ordinary `pdf-extract` path so heuristic changes cannot alter other PDFs. Accumulate output per page, then join page text without visible separators such as `***`. Preserve blank-line separation around headings and paragraphs; page boundaries alone must not create a visible marker or collapse existing paragraph structure. Finally reuse the existing Skladchik edge page-number removal and footer removal.

Prefer character geometry over dictionary-based word splitting: concatenated Russian text does not provide reliable word boundaries to recover from spelling alone. Calibrate the gap rule against the supplied Cloud Mail article and the three existing Skladchik examples, and keep regression coverage for ordinary glyph spacing so a lower threshold does not split words internally. If specialized extraction fails, fall back to the existing extracted text rather than losing readable content.

### Keep detection and spacing recovery testable separately

Cover filename and footer classification independently from glyph-gap reconstruction and page joining. Use small generated PDF fixtures for known glyph positions, heading and paragraph spacing, and page transitions. Test the supplied real PDF through the CLI against the same saved bytes when validating the final extraction change. Include non-Skladchik PDF cases to verify ordinary output remains byte-for-byte unchanged.

## Risks / Trade-offs

- [A tighter gap rule could insert spaces inside words] → Calibrate against the supplied real PDFs and add glyph-level cases for adjacent letters, kerning, and inter-word gaps.
- [The footer signature may be absent or text extraction may distort it] → Use the provider filename marker as a second independent signal; retain the footer signal for opaque filenames.
- [A custom positioned-text path may differ in line ordering from the library's plain-text path] → Preserve page coordinates and validate paragraph flow on all supplied Skladchik samples before accepting output.
- [Joining pages may erase structural whitespace or leave synthetic page markers] → Test multi-page heading and paragraph transitions, requiring blank-line spacing where the document structure calls for it and no visible `***` markers.

## Migration Plan

No data migration is required. The new specialized path is selected only for identified Skladchik PDFs. If it produces a regression, disable the specialized path while retaining ordinary PDF extraction and existing metadata/title behavior.
