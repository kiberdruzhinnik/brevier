# Spec Delta

## ADDED Requirements

### Requirement: Skladchik-specific PDF cleanup is gated by its footer signature
Brevier SHALL apply specialized Skladchik PDF cleanup only when extracted content contains the Skladchik brand, club heading, and purchase notice together in a trailing promotional footer.

#### Scenario: Identify a Skladchik PDF from its trailing footer
- **WHEN** extracted PDF text contains `skladchik.org` branding, “Что такое Клуб «Складчик»?” and “этот контент куплен на сайте” in the trailing promotional footer, allowing case, punctuation, and line-break differences
- **THEN** Brevier identifies the PDF as Skladchik-origin and applies its specialized cleanup, removing the footer from the branded heading onward

#### Scenario: Do not classify an ordinary PDF from an isolated phrase
- **WHEN** a PDF lacks any part of the combined Skladchik brand, club heading, and purchase notice in a trailing footer
- **THEN** Brevier does not classify it as Skladchik-origin and preserves ordinary PDF extraction behavior

## MODIFIED Requirements

### Requirement: Public Yandex Disk PDFs are read as text
Brevier SHALL extract text from an accessible public PDF share and present it as a text page in the CLI and Android app. It SHALL apply Skladchik-specific formatting cleanup only after identifying the PDF as Skladchik-origin.

#### Scenario: Read a public PDF
- **WHEN** a reader opens an accessible public Yandex Disk PDF share
- **THEN** Brevier displays the extracted PDF text with the document name as its title

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible public Yandex Disk PDF has the combined Skladchik footer signature
- **THEN** Brevier applies Skladchik-specific cleanup to improve paragraph flow, omit standalone page numbers at page boundaries, and remove the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible public Yandex Disk PDF does not match the Skladchik content signature
- **THEN** Brevier uses the ordinary PDF extraction behavior and preserves its extracted content without Skladchik-specific cleanup

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article

### Requirement: Public Cloud Mail PDFs are recognized from API metadata
Brevier SHALL recognize an accessible Cloud Mail PDF using provider metadata, including the API-reported filename when MIME metadata is absent, and present extracted text with the document name as its title. It SHALL apply Skladchik-specific formatting cleanup only after identifying the PDF as Skladchik-origin.

#### Scenario: Read a PDF without MIME metadata
- **WHEN** a reader opens an accessible Cloud Mail PDF share whose metadata has a PDF filename but no MIME type
- **THEN** Brevier extracts and displays the PDF text with the document name as its title

#### Scenario: PDF link does not end in a PDF suffix
- **WHEN** a reader opens an accessible Cloud Mail PDF whose share URL does not end in `.pdf`
- **THEN** Brevier uses the API metadata to recognize and read the PDF

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible Cloud Mail PDF has the combined Skladchik footer signature
- **THEN** Brevier applies Skladchik-specific cleanup to improve paragraph flow, omit standalone page numbers at page boundaries, and remove the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible Cloud Mail PDF does not match the Skladchik content signature
- **THEN** Brevier uses the ordinary PDF extraction behavior and preserves its extracted content without Skladchik-specific cleanup

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public Cloud Mail PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article
