# Proposal

## Why

Some Skladchik PDFs begin with a promotional cover page that is not part of the article. Brevier currently extracts and presents every page, so readers see the ad before the article starts.

## What Changes

- Detect a Skladchik promotional cover from the extracted content of the first PDF page and omit that page only when the ad signature is present.
- Keep the first page and all later pages when the first page does not meet the ad detection criteria.
- Use the supplied 30-page PDF as a regression sample, including a check that article text on later pages is preserved.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `public-cloud-shares`: Specify conditional removal of an identified Skladchik ad cover page from public Cloud Mail and Yandex Disk PDFs.

## Impact

The shared PDF extraction and Skladchik cleanup path in `src/pdf.rs` will change; both public share providers use it. No provider API or dependency changes are expected. The supplied PDF is a test sample; whether it is stored in the repository should follow the project's corpus and licensing conventions.
