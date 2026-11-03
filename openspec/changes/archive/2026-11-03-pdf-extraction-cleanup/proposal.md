# Proposal

## Why

Public PDFs from Yandex Disk and Cloud Mail currently arrive as flattened text that can include page numbers and promotional material, while losing useful paragraph flow. This makes otherwise readable articles harder to follow in Brevier.

## What Changes

- Detect Skladchik-origin PDFs from the combined trailing footer signature (`skladchik.org` branding, “Что такое Клуб «Складчик»?” heading, and “этот контент куплен на сайте” notice) before applying specialized extraction cleanup.
- For detected Skladchik PDFs, improve text extraction so page boundaries and paragraph flow are retained.
- For detected Skladchik PDFs, remove standalone page numbers at page boundaries without dropping article numbers in the body.
- For detected Skladchik PDFs, remove the complete trailing promotional footer beginning at its `skladchik.org` branding while preserving article text.
- Keep ordinary extraction behavior for PDFs that do not match the Skladchik signature, regardless of whether they come from Yandex Disk or Cloud Mail.
- Add regression coverage for page numbers, paragraph flow, and optional promotional tails.

## Capabilities

### New Capabilities

### Modified Capabilities
- `public-cloud-shares`: Define cleaner PDF reading behavior for both supported public share providers.

## Impact

The shared PDF extraction path in `src/pdf.rs`, PDF fixtures and tests, fuzz coverage, and the `public-cloud-shares` specification. No provider API or Android interface change is expected.
