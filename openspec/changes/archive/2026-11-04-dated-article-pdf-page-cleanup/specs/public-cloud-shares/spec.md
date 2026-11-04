# Spec Delta

## MODIFIED Requirements

### Requirement: Public Yandex Disk PDFs are read as text
Brevier SHALL extract text from an accessible public PDF share and present it as a text page in the CLI and Android app. It SHALL apply Skladchik-specific spacing recovery and formatting cleanup only after identifying the PDF as Skladchik-origin by its filename marker, raw extracted footer signature, or dated article-series signature.

#### Scenario: Read a public PDF
- **WHEN** a reader opens an accessible public Yandex Disk PDF share
- **THEN** Brevier displays the extracted PDF text with the document name as its title

#### Scenario: Recover missing spaces in a Skladchik PDF
- **WHEN** an accessible public Yandex Disk PDF is identified as Skladchik-origin
- **THEN** Brevier restores missing inter-word spaces while preserving existing spaces, punctuation, paragraph flow, and heading separation, and joins pages without visible `***` page-break markers

#### Scenario: Join the wrapped initial headline
- **WHEN** an identified Skladchik Yandex Disk PDF has its initial article headline laid out across consecutive visual lines
- **THEN** Brevier joins those headline lines with spaces into one logical heading paragraph, keeps a clear separation before the following body, and leaves later headings and ordinary PDFs unchanged

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible public Yandex Disk PDF is identified as Skladchik-origin by its filename marker, combined footer signature, or dated article-series signature
- **THEN** Brevier applies specialized spacing recovery, retains readable blank-line separation around headings and paragraphs, omits standalone page numbers at page boundaries, joins pages without visible `***` markers, and removes the complete promotional footer when its signature is present while preserving article text

#### Scenario: Identify the dated article series
- **WHEN** a Yandex Disk PDF filename begins with a `YYYYMMDD -` date prefix and its extracted text contains both the recurring `ув. друзья` salutation and `автор этих взволнованных строк` phrase, allowing case and whitespace differences
- **THEN** Brevier identifies it as Skladchik-origin even when the filename has no `[skladchik.org]` marker and its text has no purchase footer

#### Scenario: Remove page numbers only at page boundaries
- **WHEN** an identified Skladchik PDF has a line containing only an integer at the beginning or end of an extracted page
- **THEN** Brevier omits that page-boundary number and preserves standalone numbers elsewhere in the page text

#### Scenario: Remove inserted Crimson Analytics branding from Skladchik PDFs
- **WHEN** a Yandex Disk PDF is already identified as Skladchik-origin and its extracted article contains the inserted `Кримсон Аналитика – CrimsonAnalytics` branding text, allowing line-wrap and whitespace differences
- **THEN** Brevier removes only that branding text and preserves surrounding article text; this phrase alone does not identify a PDF as Skladchik-origin

#### Scenario: Apply Skladchik footer cleanup
- **WHEN** an accessible public Yandex Disk PDF contains the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier omits standalone page numbers at page boundaries and removes the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible public Yandex Disk PDF has neither a recognized Skladchik filename marker, the Skladchik purchase-footer signature, nor the dated article-series signature
- **THEN** Brevier uses ordinary PDF extraction behavior without Skladchik-specific spacing recovery or cleanup

#### Scenario: Preserve the branding phrase in non-Skladchik PDFs
- **WHEN** a Yandex Disk PDF is not identified as Skladchik-origin and its text contains `Кримсон Аналитика – CrimsonAnalytics`
- **THEN** Brevier preserves the phrase as ordinary document text

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article

### Requirement: Skladchik-specific PDF cleanup is gated by its footer signature
Brevier SHALL identify Skladchik PDFs using a recognized Skladchik filename marker, the Skladchik brand, club heading, and purchase notice together in a trailing promotional footer, or the dated article-series signature. Brevier SHALL inspect a footer signature before removing the promotional footer.

#### Scenario: Identify a Skladchik PDF from its filename
- **WHEN** the provider-reported PDF filename contains a recognized Skladchik marker, such as `[skladchik.org]`
- **THEN** Brevier identifies the PDF as Skladchik-origin and applies Skladchik-specific spacing recovery

#### Scenario: Identify a Skladchik PDF from its trailing footer
- **WHEN** raw extracted PDF text contains `skladchik.org` branding, “Что такое Клуб «Складчик»?” and “этот контент куплен на сайте” in the trailing promotional footer, allowing case, punctuation, and line-break differences
- **THEN** Brevier identifies the PDF as Skladchik-origin, applies Skladchik-specific spacing recovery, and removes the footer from its branded heading onward

#### Scenario: Identify the dated article series without its purchase footer
- **WHEN** the provider-reported PDF filename begins with a `YYYYMMDD -` date prefix and raw extracted PDF text contains both `ув. друзья` and `автор этих взволнованных строк`, allowing case and whitespace differences
- **THEN** Brevier identifies it as Skladchik-origin and applies Skladchik-specific spacing recovery without requiring a Skladchik filename marker or purchase footer

#### Scenario: Do not classify an ordinary PDF from an isolated phrase
- **WHEN** a PDF has neither a recognized Skladchik filename marker nor the combined Skladchik footer signature nor the dated article-series signature
- **THEN** Brevier does not classify it as Skladchik-origin and preserves ordinary PDF extraction behavior

### Requirement: Public Cloud Mail PDFs are recognized from API metadata
Brevier SHALL recognize an accessible Cloud Mail PDF using provider metadata, including the API-reported filename when MIME metadata is absent, and present extracted text with the document name as its title. It SHALL apply Skladchik-specific spacing recovery and formatting cleanup only after identifying the PDF as Skladchik-origin by its filename marker, raw extracted footer signature, or dated article-series signature.

#### Scenario: Read a PDF without MIME metadata
- **WHEN** a reader opens an accessible Cloud Mail PDF share whose metadata has a PDF filename but no MIME type
- **THEN** Brevier extracts and displays the PDF text with the document name as its title

#### Scenario: PDF link does not end in a PDF suffix
- **WHEN** a reader opens an accessible Cloud Mail PDF whose share URL does not end in `.pdf`
- **THEN** Brevier uses the API metadata to recognize and read the PDF

#### Scenario: Recover missing spaces in a Skladchik PDF
- **WHEN** an accessible Cloud Mail PDF is identified as Skladchik-origin
- **THEN** Brevier restores missing inter-word spaces while preserving existing spaces, punctuation, paragraph flow, and heading separation, and joins pages without visible `***` page-break markers

#### Scenario: Join the wrapped initial headline
- **WHEN** an identified Skladchik Cloud Mail PDF has its initial article headline laid out across consecutive visual lines
- **THEN** Brevier joins those headline lines with spaces into one logical heading paragraph, keeps a clear separation before the following body, and leaves later headings and ordinary PDFs unchanged

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible Cloud Mail PDF is identified as Skladchik-origin by its filename marker, combined footer signature, or dated article-series signature
- **THEN** Brevier applies specialized spacing recovery, retains readable blank-line separation around headings and paragraphs, omits standalone page numbers at page boundaries, joins pages without visible `***` markers, and removes the complete promotional footer when its signature is present while preserving article text

#### Scenario: Identify the dated article series
- **WHEN** a Cloud Mail PDF filename begins with a `YYYYMMDD -` date prefix and its extracted text contains both the recurring `ув. друзья` salutation and `автор этих взволнованных строк` phrase, allowing case and whitespace differences
- **THEN** Brevier identifies it as Skladchik-origin even when its filename has no `[skladchik.org]` marker and its text has no purchase footer

#### Scenario: Remove page numbers only at page boundaries
- **WHEN** an identified Skladchik PDF has a line containing only an integer at the beginning or end of an extracted page
- **THEN** Brevier omits that page-boundary number and preserves standalone numbers elsewhere in the page text

#### Scenario: Remove inserted Crimson Analytics branding from Skladchik PDFs
- **WHEN** a Cloud Mail PDF is already identified as Skladchik-origin and its extracted article contains the inserted `Кримсон Аналитика – CrimsonAnalytics` branding text, allowing line-wrap and whitespace differences
- **THEN** Brevier removes only that branding text and preserves surrounding article text; this phrase alone does not identify a PDF as Skladchik-origin

#### Scenario: Apply Skladchik footer cleanup
- **WHEN** an accessible Cloud Mail PDF contains the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier omits standalone page numbers at page boundaries and removes the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible Cloud Mail PDF has neither a recognized Skladchik filename marker, the Skladchik purchase-footer signature, nor the dated article-series signature
- **THEN** Brevier uses ordinary PDF extraction behavior without Skladchik-specific spacing recovery or cleanup

#### Scenario: Preserve the branding phrase in non-Skladchik PDFs
- **WHEN** a Cloud Mail PDF is not identified as Skladchik-origin and its text contains `Кримсон Аналитика – CrimsonAnalytics`
- **THEN** Brevier preserves the phrase as ordinary document text

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public Cloud Mail PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article
