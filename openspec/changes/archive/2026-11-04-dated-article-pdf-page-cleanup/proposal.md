# Proposal

## Why

Some PDFs in the dated Marahovsky article series lack both the `[skladchik.org]` filename marker and the Skladchik purchase footer. The generic extractor therefore leaves standalone page numbers in the article, while existing Skladchik extraction already removes them when the document is correctly identified.

## What Changes

- Identify this article series using the combination of a `YYYYMMDD -` PDF filename prefix and its recurring author text signature (`ув. друзья` and `автор этих взволнованных строк`).
- Apply the existing Skladchik-oriented layout recovery to identified series PDFs.
- Remove standalone integers only at page boundaries, preserving numbers in article text and leaving generic extraction unchanged for other PDFs.
- Remove the stray `Кримсон Аналитика – CrimsonAnalytics` branding text only from PDFs already identified as Skladchik-origin, preserving surrounding article text and leaving the phrase untouched in other PDFs.
- Join visual wraps of the initial Skladchik article headline with spaces into one logical heading, preserving the break before the body and leaving later headings unchanged.
- Apply the shared PDF behavior consistently to Yandex Disk and Cloud Mail public PDFs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `public-cloud-shares`: Extend Skladchik-origin detection with the dated article-series marker and specify page-boundary-only number cleanup.

## Impact

The shared Rust PDF extraction path in `src/pdf.rs` and its public share entry points may be affected. No new dependencies or Android-specific changes are expected.
