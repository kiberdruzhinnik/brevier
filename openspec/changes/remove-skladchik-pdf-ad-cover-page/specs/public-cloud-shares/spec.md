# Spec Delta

## ADDED Requirements

### Requirement: Omit a detected Skladchik promotional cover page
Brevier SHALL omit the first page of a public PDF only when page-level text identifies that page as a Skladchik advertisement; an uncertain match SHALL preserve the page.

#### Scenario: Remove the supplied Skladchik ad cover
- **WHEN** a reader opens the supplied public PDF and its first page matches the Skladchik advertisement signature
- **THEN** Brevier omits that page and presents the article beginning on the following page

#### Scenario: Preserve an ordinary first page
- **WHEN** a public PDF's first page does not match the Skladchik advertisement signature
- **THEN** Brevier presents the first page with the rest of the document

#### Scenario: Preserve article pages after removing an ad cover
- **WHEN** Brevier omits a detected Skladchik ad cover
- **THEN** it preserves the text and order of every subsequent page, including article text that contains Skladchik branding or purchase references

#### Scenario: Do not remove a page on a weak or uncertain match
- **WHEN** the first page contains an isolated Skladchik phrase but lacks the evidence required to identify a promotional cover
- **THEN** Brevier preserves the first page
