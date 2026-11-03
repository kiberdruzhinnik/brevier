# Proposal

## Why

Some Skladchik PDFs extract with words concatenated because the PDF text extractor misses small gaps between glyphs. This makes otherwise readable articles difficult to follow; the supplied Cloud Mail PDF reproduces the problem.

## What Changes

- Recover missing inter-word spaces in PDFs identified as Skladchik-origin.
- Identify Skladchik PDFs using either a recognized filename marker or the existing purchase-footer signature in raw extracted text, before that footer is removed.
- Join extracted pages without visible `***` page-break markers.
- Preserve readable heading and paragraph separation with blank-line padding.
- Keep the spacing recovery out of ordinary PDFs and retain the existing Skladchik footer and page-number cleanup.

## Capabilities

### New Capabilities

### Modified Capabilities
- `public-cloud-shares`: define Skladchik PDF identification and readable spacing behavior for public PDFs from either cloud provider.

## Impact

The shared Rust PDF extraction path and its tests; Cloud Mail and Yandex Disk PDF title metadata; the `public-cloud-shares` behavior contract. Android and the Linux maintainer CLI use the same core.
