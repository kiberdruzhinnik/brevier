# Spec Delta

## MODIFIED Requirements

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
