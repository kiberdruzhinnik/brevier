# Design

## Context

See proposal.md for the observed documents and motivation. PDF conversion is shared by the Yandex Disk and Cloud Mail adapters. The current classifier receives the provider filename and plain extracted text; Skladchik cleanup can then use page-level extraction and remove integer-only lines at page edges.

## Goals / Non-Goals

**Goals:**
- Recognize the dated article series when its provider filename and author text signature both match.
- Use the existing positioned PDF extraction path for recognized documents.
- Remove only standalone integer lines at the first or last nonblank line of each PDF page.
- Remove the inserted `Кримсон Аналитика – CrimsonAnalytics` branding phrase only after a PDF is identified as Skladchik-origin, without consuming surrounding article text.
- Join visual wraps of the initial article headline into one logical heading paragraph for identified Skladchik PDFs, preserving separation from the article body.
- Keep footer removal dependent on the existing Skladchik purchase-footer signature.

**Non-Goals:**
- Treat PDF creator/producer metadata, a date prefix alone, or page numbering alone as proof of Skladchik origin.
- Remove numbers from article body text or change ordinary extraction for unrecognized PDFs.
- Add provider-specific extraction behavior or dependencies.

## Decisions

- **Use a composite dated-series marker.** Require a provider filename prefix matching `YYYYMMDD -` and both recurring text phrases `ув. друзья` and `автор этих взволнованных строк`. Normalize case and whitespace before checking the phrases. This distinguishes the supplied series more narrowly than ABBYY metadata, which is absent from one comparison PDF, or a date prefix alone.
- **Reuse the shared Skladchik extraction route.** Both public share adapters already pass the provider filename and PDF bytes to the same extractor. Classification belongs there so both providers behave consistently and other PDFs retain ordinary extraction.
- **Keep page-number removal page-local and edge-only.** For each extracted page, remove an integer-only first or last nonblank line; preserve all other lines. This avoids stripping numbered references, years, or list items from the body.
- **Gate branding cleanup on existing Skladchik classification.** Normalize line wrapping and whitespace to recognize the known `Кримсон Аналитика – CrimsonAnalytics` insertion, then remove only that phrase. The phrase is not a classifier marker, and ordinary PDFs retain it as document text.
- **Join only the initial headline's visual wraps.** Treat consecutive prominent lines in the document's initial headline block as one logical heading by joining them with spaces. Keep the heading separated from the following body and do not merge later headings or ordinary PDF text.
- **Keep advertisement removal independently gated.** The dated-series marker enables layout recovery and page-number cleanup, but the promotional footer is removed only when the existing complete footer signature is present.

## Risks / Trade-offs

- [An unrelated dated article could contain both recurring phrases] → Require the filename date prefix and both normalized phrases together; keep cleanup limited to page-edge integers.
- [Provider metadata could omit or alter the original PDF filename] → Leave such a PDF on ordinary extraction unless its existing footer or filename marker identifies it.
- [An article body could begin or end with a standalone integer] → This heuristic may remove it when it sits at an extracted page edge; regression cases will cover body numbers away from edges and page-edge behavior.
- [A subtitle or separate heading could be mistaken for a wrapped headline] → Limit joining to the initial prominent headline block and preserve the break before the body.

## Migration Plan

No stored data or user settings change. The new classification affects only PDF extraction at read time. No rollback migration is required.
