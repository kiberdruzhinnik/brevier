# Design

## Context

The Cloud Mail integration in `src/hosts/cloud_mail.rs` shows how Brevier reads a public cloud-drive folder as an Atom listing and extracts text from a public PDF with the existing `pdf-extract` dependency. `fetch::readable` is shared by the CLI and Android through the Rust core. Requests use the existing TLS verifier and 8 MiB response limit.

Yandex public share URLs identify a share, but may not expose a filename in the URL. The integration therefore needs metadata before deciding whether the resource is a folder, a PDF, or an unsupported file. A folder entry is addressed by the original public share key plus its path within that share.

## Goals / Non-Goals

**Goals:**

- Add Yandex Disk public shares through the existing core reading path for both CLI and Android.
- Preserve public-share identity and nested paths when rendering folder entry links.
- Keep API requests unauthenticated, bounded by the existing response limits, and on the operating system's TLS trust path.
- Reuse the existing PDF extraction and feed-rendering behavior where it fits.

**Non-Goals:**

- Private Disk resources, OAuth, cookies, or any authenticated operation.
- Downloading non-PDF files, archives, or entire folders.
- Executing Yandex's JavaScript viewer or adding a cloud-specific dependency.

## Decisions

1. **Use Yandex's public resources API for share metadata and folder contents.** Send the original share URL as the public key and include the selected path for entries inside a shared folder. This handles file and folder shares whose names are absent from the browser URL. Parsing the JavaScript shell or scraping its embedded state was considered, but would couple reading to the web app rather than its public resource contract.

2. **Resolve a temporary download URL only after metadata identifies a PDF.** Request the public download link for that resource, require an HTTPS URL from the API response, fetch it under the existing body limit, and pass its bytes to the local PDF text conversion. Do not request a token or persist the temporary URL. Restrict share recognition to Yandex Disk public-link hosts and path forms; unrelated Yandex pages keep the normal HTML path.

3. **Convert directory metadata to the existing feed representation.** Build entries using the original share key plus each API entry path, escaped as URL query data, then render through Brevier's existing feed path. This keeps navigation link behavior consistent across CLI and Android without introducing a second folder UI.

4. **Keep provider parsing separate from Cloud Mail.** Add a Yandex host module for URL recognition, API URL construction, response validation, and Atom conversion. Share the generic PDF conversion helper if needed rather than making Yandex parsing depend on Cloud Mail's provider module. This keeps each provider's API contract independently testable.

5. **Treat API failures as ordinary read failures.** Private shares, denied downloads, malformed API responses, and expired temporary URLs use existing network, unsupported-content, or extraction errors. Do not retry through a browser session or silently fetch unrelated resources.

## Risks / Trade-offs

- [Yandex may change or rate-limit its public API] -> Keep endpoint and response parsing isolated and cover saved response shapes with tests; a live CLI check records the behavior of the supplied share at implementation time.
- [A returned download URL or redirect may be unavailable or have a non-PDF response] -> Require HTTPS, bound response size, validate the resulting bytes through the existing PDF extractor, and surface the existing failure path.
- [Large folder responses may exceed the reader's response cap] -> Keep the existing 8 MiB cap and report the request failure rather than turning folder reading into crawling or bulk download.

## Migration Plan

No stored data or migration is involved. Build the Rust core and Android debug APK from the updated source. Rollback is reverting the Yandex host integration and its dispatch path; Cloud Mail behavior remains covered by its existing tests.
