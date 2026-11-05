# Spec Delta

## ADDED Requirements

### Requirement: Signed Android release cold start
The signed ARM64 Android release SHALL reach the reader on cold start without an uncaught exception or native crash.

#### Scenario: Cold start
- **WHEN** the user cold starts the signed ARM64 release
- **THEN** the app reaches the reader without crashing

#### Scenario: OCR runtime remains lazy
- **WHEN** the user opens a PDF and has not selected OCR for any page
- **THEN** PDFium, Tesseract, and OCR language models are not initialized

### Requirement: Optional OCR recovers visible text from public PDFs
Brevier SHALL offer local OCR for public PDFs only when the reader explicitly enables it, and SHALL identify text produced by OCR.

#### Scenario: Regular extraction is the initial method
- **WHEN** a reader opens or reloads a public PDF
- **THEN** Brevier fetches the document and extracts text with its existing PDF method first
- **AND** it does not run OCR unless the reader selects OCR for a page

#### Scenario: Select OCR from the dots menu
- **WHEN** a reader opens the dots popup menu while viewing a PDF
- **THEN** the menu offers “Recognize PDF text (OCR)” while regular text is displayed
- **AND** selecting it runs OCR and displays the OCR result for that PDF
- **AND** while the OCR result is displayed, the menu offers “Use regular PDF text”
- **AND** selecting it restores the retained regular result
- **AND** the OCR action is not shown for non-PDF pages

#### Scenario: Recover the supplied visible definition with per-page OCR
- **WHEN** a reader selects OCR for the page containing the supplied definition
- **THEN** the article includes “термин «старые деньги» используется для обозначения богатых людей, чьи семьи были богаты в течение длительного времени” once, between the Cambridge Dictionary introduction and the following paragraph, identified as OCR-derived text

#### Scenario: Switch an OCR page back to regular extraction
- **WHEN** a reader switches a page that was OCR-processed back to its regular method
- **THEN** Brevier restores the regular extraction result retained for that page
- **AND** OCR-derived text is no longer displayed for that page

#### Scenario: Merge OCR and native text without duplication
- **WHEN** a reader selects OCR for a page with some text already available from the PDF text layer
- **THEN** Brevier preserves native text, adds OCR text for uncovered visible content in reading order, and does not duplicate overlapping text

#### Scenario: Recognize Russian text mixed with English words
- **WHEN** a reader selects OCR for a PDF page whose text is primarily Russian with some English words
- **THEN** Brevier recognizes both Russian and English text, preserves the words in reading order, and identifies the recognized content as OCR-derived

#### Scenario: Report the active loading stage
- **WHEN** a public PDF is loading or a page is being reprocessed
- **THEN** Brevier identifies the active stage as network fetching, regular text extraction, or OCR text extraction
- **AND** it reports OCR text extraction only while processing the page at the reader's request

#### Scenario: Continue reading when OCR fails
- **WHEN** OCR is requested but page rendering or recognition fails
- **THEN** Brevier preserves the regular extraction result for that page and follows its existing extraction failure behavior if no readable text remains
