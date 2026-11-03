# Tasks

## 1. Identify Skladchik PDFs

- [x] 1.1 Add classification from provider-reported PDF filenames and raw extracted purchase-footer signatures; test both signals, matching variants, and ordinary PDFs without either signal.
- [x] 1.2 Verify footer classification runs before footer removal with a regression test where the purchase ad is the only Skladchik signal.

## 2. Recover spacing in identified PDFs

- [x] 2.1 Add a position-aware spacing path for identified Skladchik PDFs and test synthetic PDFs with missing inter-word gaps, existing spaces, kerning, line breaks, and page boundaries.
- [x] 2.2 Preserve ordinary PDF output byte for byte and verify non-Skladchik PDFs do not enter the specialized spacing path.
- [x] 2.3 Keep existing Skladchik page-number and promotional-footer cleanup working with recovered text; add regression coverage for the footer and page edges.
- [x] 2.4 Join extracted pages without visible `***` page-break markers while retaining paragraph breaks and readable blank-line spacing around headings; add multi-page regression coverage.

## 3. Validate real documents and parser safety

- [x] 3.1 Save the supplied Cloud Mail PDF bytes and the other supplied Skladchik sample PDFs once, run the same saved bytes through the previous and changed extractors, and inspect each difference for restored spaces and preserved article text.
- [x] 3.2 Run the PDF fuzz target for several minutes and verify it reports no panic or hang on malformed or unusual PDF input.
