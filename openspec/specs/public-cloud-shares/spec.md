# public-cloud-shares Specification

## Purpose

Lets readers browse public cloud-drive folders and read supported public PDF files in Brevier without JavaScript, credentials, or a cloud-drive account.

## Requirements

### Requirement: Public Yandex Disk folders are readable listings
Brevier SHALL render an accessible public Yandex Disk folder as a listing of links to its shared entries, including subfolders, without executing JavaScript or holding credentials.

#### Scenario: Open a public folder
- **WHEN** a reader opens an accessible public Yandex Disk folder share
- **THEN** Brevier shows the folder entries as links

#### Scenario: Follow an entry in a nested folder
- **WHEN** a reader follows a folder or file link from a public folder listing
- **THEN** the link opens the corresponding entry within the same public share

### Requirement: Public Yandex Disk PDFs are read as text
Brevier SHALL extract text from an accessible public PDF share and present it as a text page in the CLI and Android app.

#### Scenario: Read a public PDF
- **WHEN** a reader opens an accessible public Yandex Disk PDF share
- **THEN** Brevier displays the extracted PDF text with the document name as its title

#### Scenario: PDF contains no extractable text
- **WHEN** a reader opens a public PDF whose contents yield no extractable text
- **THEN** Brevier reports its existing extraction failure rather than displaying an empty article

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
Brevier SHALL recognize an accessible Cloud Mail PDF using provider metadata, including the API-reported filename when MIME metadata is absent, and present extracted text with the document name as its title.

#### Scenario: Read a PDF without MIME metadata
- **WHEN** a reader opens an accessible Cloud Mail PDF share whose metadata has a PDF filename but no MIME type
- **THEN** Brevier extracts and displays the PDF text with the document name as its title

#### Scenario: PDF link does not end in a PDF suffix
- **WHEN** a reader opens an accessible Cloud Mail PDF whose share URL does not end in `.pdf`
- **THEN** Brevier uses the API metadata to recognize and read the PDF

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
