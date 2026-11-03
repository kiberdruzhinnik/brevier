# Design

## Context

See proposal.md - Why. Both public share providers pass PDF bytes through the shared extractor in `src/pdf.rs`. The current extractor returns flattened text, and its form-feed splitting does not reliably expose page boundaries. Cleanup is specific to Skladchik-origin PDFs, so the extractor must classify the content before selecting that path. PDF text is plain text in the reader, so the goal is to preserve useful paragraph structure rather than visual typography.

## Goals / Non-Goals

**Goals:**
- Identify Skladchik-origin PDFs from reliable content markers before applying specialized cleanup.
- For positively identified Skladchik PDFs, expose page boundaries so page-edge numbers can be removed narrowly, reflow line wrapping, and remove the recognized promotional tail.
- Preserve ordinary extraction for PDFs that are not identified as Skladchik.
- Apply the same classification and cleanup behavior to Yandex Disk and Cloud Mail PDFs.

**Non-Goals:**
- Reconstruct columns, font styling, tables, or exact visual layout.
- Remove numbers inside article paragraphs or promotional text that cannot be identified by the agreed marker.
- Change provider APIs, access behavior, or the Android reader interface.

## Decisions

1. **Classify from the observed combined footer before specializing.** The supplied Yandex Disk and both Cloud Mail shares include a trailing footer containing `skladchik.ORG` branding, the heading “Что такое Клуб «Складчик»?”, and “Этот контент куплен на сайте”. Normalize case, punctuation, and whitespace, then require all three markers in the trailing footer before selecting specialized cleanup. If classification is uncertain, retain ordinary extraction. Alternatives considered: applying cleanup to every PDF or classifying from a generic purchase notice alone, both rejected because ordinary documents can contain page numbers or similar phrases.

2. **Retain page-aware extraction through cleanup.** Use page-level extracted text (or an equivalent representation that explicitly carries page boundaries) instead of relying on form-feed characters in a flattened string. This lets cleanup remove isolated page numbers at page edges while leaving numbered article content untouched. Alternatives considered: regex removal over the complete flattened text, rejected because it cannot reliably distinguish a page number from a one-line numbered paragraph.

3. **Normalize line wrapping conservatively.** Join single line breaks within a paragraph with spaces, preserve blank-line paragraph boundaries, and avoid destructive rewriting of punctuation or inline spacing. Alternatives considered: flattening all whitespace, rejected because it erases paragraph structure.

4. **Remove the whole branded footer only after positive classification.** Once all three signature markers are confirmed in the trailing footer, truncate from the footer's `skladchik.org` branded heading, not from the final purchase notice, so the entire advertisement is removed. Regression cases should cover markers split across extracted lines and isolated marker mentions in article content, so matching does not truncate legitimate body text.

5. **Exercise cleanup with small deterministic PDFs and fuzz input.** Prefer minimal generated or committed fixtures that isolate page-edge numbers, paragraphs, and optional promotional tails. Extend the relevant fuzz target so malformed or unusual PDF text cannot panic or hang. Keep a real shared PDF sample only if a minimized fixture cannot reproduce the observed formatting issue.

## Risks / Trade-offs

- [PDF layout can be ambiguous] → Preserve blank-line boundaries and keep line reflow conservative; verify against representative documents from the reported share.
- [Classification may produce false positives or false negatives] → Require all three observed signature markers in a trailing footer, add positive and negative fixtures, and use ordinary extraction when the signature is uncertain.
- [A promotional marker may occur in article text or be split across lines] → Require tail-context matching and test both split markers and body mentions before truncating.
- [Page-level extraction may expose page text in a different order than the current flat output] → Compare representative documents and add a fixture that records intended paragraph and page-number behavior.
- [PDF fuzzing can be expensive] → Run the repository's PDF-related fuzz target for the duration prescribed in CONTRIBUTING and report any environment limit.

## Migration Plan

No data migration is needed. The cleanup applies when PDF text is extracted; previously cached or saved Markdown is not rewritten. Revert the shared extractor change to roll back.
