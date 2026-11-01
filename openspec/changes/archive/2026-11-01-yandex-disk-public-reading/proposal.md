# Proposal

## Why

Public Yandex Disk shares currently open as JavaScript-driven pages that Brevier cannot read. Readers should be able to follow public folders and read public PDFs from Yandex Disk with the same deterministic, no-login behavior already provided for Cloud Mail shares.

## What Changes

- Read public Yandex Disk folder shares as listings of links to their entries, including nested folders.
- Read public PDF shares by resolving a temporary public download URL and extracting the PDF text locally into Brevier's Markdown page.
- Make the same public-share reading behavior available in the CLI and Android app.
- Keep unsupported file types, private shares, and failed or restricted downloads on the existing unsupported-content or extraction-error paths.

## Capabilities

### New Capabilities

- `public-cloud-shares`: Read-only browsing of public cloud-drive folders and local text extraction from supported public PDF shares.

### Modified Capabilities

None.

## Impact

- `src/hosts/`: add Yandex Disk public-link parsing and response conversion alongside the Cloud Mail host integration.
- `src/fetch.rs`: resolve Yandex Disk public folders and PDFs through read-only requests, reusing the existing local PDF extraction path where practical.
- `src/failure.rs` and `src/error.rs`: update unsupported-content wording only if needed to accurately describe supported public PDF sources.
- Verify the example share through the CLI and provide a built Android APK containing the feature.
- No new dependency is expected; `pdf-extract` is already used for Cloud Mail PDFs.
