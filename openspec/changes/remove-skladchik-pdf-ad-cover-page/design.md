# Design

## Context

See proposal.md for the motivation. `src/pdf.rs` already has a shared page-level extraction path for identified Skladchik PDFs, and both Cloud Mail and Yandex Disk call the shared PDF renderer. Existing footer cleanup is based on full-document text and removes a trailing promotion; it does not classify or omit an opening ad page.

## Goals / Non-Goals

**Goals:**
- Make the ad decision from the extracted content of page one, not from the document filename or a phrase elsewhere in the article.
- Require the purchase notice plus additional cover-specific evidence before omitting the page.
- Leave the first page in place when extraction is incomplete or the ad match is uncertain.
- Apply the behavior consistently to both supported public PDF providers.

**Non-Goals:**
- OCR or model-based classification for scanned pages.
- Removing promotional pages after the first page or changing generic PDF page joining.
- Removing Skladchik branding or purchase references from article pages.

## Decisions

- **Use page-local evidence.** Classify the first extracted page independently. Normalize case, whitespace, and punctuation for matching so line wrapping does not defeat the signature. Require `куплено на SKLADCHIK.ORG` together with additional ad-specific cues identified from the supplied PDF; the purchase phrase by itself is insufficient. This keeps article text and filename markers from triggering page removal. The alternative of classifying from the full document is rejected because article pages may mention the same brand or phrase.
- **Reuse the shared PDF path.** Apply cover detection after page-level extraction in the common renderer, so Cloud Mail and Yandex Disk behave the same. If page-level extraction cannot establish the signature, retain the ordinary extracted text rather than guessing.
- **Keep later pages intact.** Remove only the first page from the page sequence. Preserve the remaining page order and content, including any article references to Skladchik.
- **Make the sample regression reproducible.** Use the attached 30-page PDF to identify and verify the cover signature. Keep a stable fixture or derived regression input for automated tests, and assert that article content from subsequent pages remains present. Do not require the full supplied PDF to be committed if a smaller fixture can preserve the relevant behavior.

## Risks / Trade-offs

- [PDF extraction may omit or rearrange cover text] → Preserve the first page unless the full page-local signature is observed; exercise the supplied sample in regression checks.
- [A future ad uses different wording] → Keep the detector narrow and add evidence-backed variants only with a sample regression.
- [The ad phrase appears in an article page] → Inspect only page one and require additional cover cues.

## Migration Plan

No stored data or API changes. The shared renderer changes at read time; reverting the code restores the current behavior.
