# public-cloud-shares Specification

## Purpose

Lets readers browse public cloud-drive folders and read supported public PDF files in Brevier without JavaScript, credentials, or a cloud-drive account.

## Requirements

### Requirement: Public Yandex Disk folders are readable listings
Brevier SHALL render an accessible public Yandex Disk folder as a listing of links to every shared entry, including subfolders, collecting all API pages without executing JavaScript or holding credentials.

#### Scenario: Open a public folder
- **WHEN** a reader opens an accessible public Yandex Disk folder share
- **THEN** Brevier shows the folder entries as links

#### Scenario: Follow an entry in a nested folder
- **WHEN** a reader follows a folder or file link from a public folder listing
- **THEN** the link opens the corresponding entry within the same public share

#### Scenario: List a folder with more entries than one API page
- **WHEN** an accessible public Yandex Disk folder contains more entries than the API's default page size
- **THEN** Brevier shows a link for every entry across all API pages

#### Scenario: A later page cannot be loaded
- **WHEN** Brevier cannot load or validate a later page of an accessible public Yandex Disk folder
- **THEN** Brevier reports its existing extraction failure instead of showing an incomplete listing

### Requirement: Public Yandex Disk PDFs are read as text
Brevier SHALL extract text from an accessible public PDF share and present it as a text page in the CLI and Android app. It SHALL apply Skladchik-specific spacing recovery and formatting cleanup only after identifying the PDF as Skladchik-origin by its filename marker or raw extracted footer signature.

#### Scenario: Read a public PDF
- **WHEN** a reader opens an accessible public Yandex Disk PDF share
- **THEN** Brevier displays the extracted PDF text with the document name as its title

#### Scenario: Recover missing spaces in a Skladchik PDF
- **WHEN** an accessible public Yandex Disk PDF is identified as Skladchik-origin
- **THEN** Brevier restores missing inter-word spaces while preserving existing spaces, punctuation, paragraph flow, and heading separation, and joins pages without visible `***` page-break markers

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible public Yandex Disk PDF is identified as Skladchik-origin by its filename marker or combined footer signature
- **THEN** Brevier applies specialized spacing recovery, retains readable blank-line separation around headings and paragraphs, omits standalone page numbers, joins pages without visible `***` markers, and removes the complete promotional footer when its signature is present while preserving article text

#### Scenario: Apply Skladchik footer cleanup
- **WHEN** an accessible public Yandex Disk PDF contains the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier omits standalone page numbers at page boundaries and removes the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible public Yandex Disk PDF has neither a recognized Skladchik filename marker nor the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier uses ordinary PDF extraction behavior without Skladchik-specific spacing recovery or cleanup

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article

### Requirement: Skladchik-specific PDF cleanup is gated by its footer signature
Brevier SHALL identify Skladchik PDFs using either a recognized Skladchik filename marker or the Skladchik brand, club heading, and purchase notice together in raw extracted text. Brevier SHALL inspect the footer signature before removing the promotional footer.

#### Scenario: Identify a Skladchik PDF from its filename
- **WHEN** the provider-reported PDF filename contains a recognized Skladchik marker, such as `[skladchik.org]`
- **THEN** Brevier identifies the PDF as Skladchik-origin and applies Skladchik-specific spacing recovery

#### Scenario: Identify a Skladchik PDF from its trailing footer
- **WHEN** raw extracted PDF text contains `skladchik.org` branding, “Что такое Клуб «Складчик»?” and “этот контент куплен на сайте” in the trailing promotional footer, allowing case, punctuation, and line-break differences
- **THEN** Brevier identifies the PDF as Skladchik-origin, applies Skladchik-specific spacing recovery, and removes the footer from its branded heading onward

#### Scenario: Do not classify an ordinary PDF from an isolated phrase
- **WHEN** a PDF has neither a recognized Skladchik filename marker nor the combined Skladchik brand, club heading, and purchase notice in a trailing footer
- **THEN** Brevier does not classify it as Skladchik-origin and preserves ordinary PDF extraction behavior

### Requirement: Unsupported or inaccessible Yandex Disk shares fail safely
Brevier SHALL preserve its existing unsupported-content and extraction-error behavior for non-PDF files, private shares, and public shares whose contents cannot be downloaded.

#### Scenario: Open a public non-PDF file
- **WHEN** a reader opens a public Yandex Disk file that is not a PDF
- **THEN** Brevier reports the existing unsupported-content response for that file type

#### Scenario: Open a private, restricted, or unavailable share
- **WHEN** public metadata or file content cannot be obtained for a Yandex Disk share
- **THEN** Brevier reports an existing access, unsupported-content, or extraction error without requesting credentials

### Requirement: Public Cloud Mail folders are readable listings
Brevier SHALL render an accessible public Cloud Mail folder as a listing of links to its shared entries, including subfolders, without executing JavaScript or holding credentials.

#### Scenario: Open a public folder
- **WHEN** a reader opens an accessible public Cloud Mail folder share
- **THEN** Brevier shows the folder entries as links

#### Scenario: Follow an entry in a nested folder
- **WHEN** a reader follows a folder or file link from a public Cloud Mail folder listing
- **THEN** the link opens the corresponding entry within the same public share

### Requirement: Public Cloud Mail PDFs are recognized from API metadata
Brevier SHALL recognize an accessible Cloud Mail PDF using provider metadata, including the API-reported filename when MIME metadata is absent, and present extracted text with the document name as its title. It SHALL apply Skladchik-specific spacing recovery and formatting cleanup only after identifying the PDF as Skladchik-origin by its filename marker or raw extracted footer signature.

#### Scenario: Read a PDF without MIME metadata
- **WHEN** a reader opens an accessible Cloud Mail PDF share whose metadata has a PDF filename but no MIME type
- **THEN** Brevier extracts and displays the PDF text with the document name as its title

#### Scenario: PDF link does not end in a PDF suffix
- **WHEN** a reader opens an accessible Cloud Mail PDF whose share URL does not end in `.pdf`
- **THEN** Brevier uses the API metadata to recognize and read the PDF

#### Scenario: Recover missing spaces in a Skladchik PDF
- **WHEN** an accessible Cloud Mail PDF is identified as Skladchik-origin
- **THEN** Brevier restores missing inter-word spaces while preserving existing spaces, punctuation, paragraph flow, and heading separation, and joins pages without visible `***` page-break markers

#### Scenario: Apply specialized extraction to a Skladchik PDF
- **WHEN** an accessible Cloud Mail PDF is identified as Skladchik-origin by its filename marker or combined footer signature
- **THEN** Brevier applies specialized spacing recovery, retains readable blank-line separation around headings and paragraphs, omits standalone page numbers, joins pages without visible `***` markers, and removes the complete promotional footer when its signature is present while preserving article text

#### Scenario: Apply Skladchik footer cleanup
- **WHEN** an accessible Cloud Mail PDF contains the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier omits standalone page numbers at page boundaries and removes the complete promotional footer beginning at its branded heading while preserving article text

#### Scenario: Keep ordinary extraction for other PDFs
- **WHEN** an accessible Cloud Mail PDF has neither a recognized Skladchik filename marker nor the Skladchik purchase-footer signature in its raw extracted text
- **THEN** Brevier uses ordinary PDF extraction behavior without Skladchik-specific spacing recovery or cleanup

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public Cloud Mail PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article

### Requirement: Unsupported or inaccessible Cloud Mail shares fail safely
Brevier SHALL preserve its existing unsupported-content and extraction-error behavior for non-PDF files and inaccessible, malformed, or unavailable public shares without falling through to the provider's JavaScript shell or requesting credentials.

#### Scenario: Open a public non-PDF file
- **WHEN** a reader opens a public Cloud Mail file that is not a PDF
- **THEN** Brevier reports the existing unsupported-content response for that file type

#### Scenario: Open an inaccessible or malformed share
- **WHEN** provider metadata or file content cannot be obtained or parsed for a Cloud Mail share
- **THEN** Brevier reports an existing access, unsupported-content, or extraction error without requesting credentials or rendering the JavaScript shell
