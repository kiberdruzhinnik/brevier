# Design

## Context

The weekly cache in `src/cache.rs` stores a structured page record and downloaded images in app-managed filesystem directories. Android's `open()` uses that cache before fetching, and the permanent archive is a separate store. Android folder selection yields a document tree URI, while core Rust storage expects filesystem paths. Manual Android file access already goes through `ContentResolver`.

See `proposal.md` for motivation and `specs/shared-markdown-cache/spec.md` for externally visible behavior.

## Goals / Non-Goals

**Goals:**
- Keep document-tree URI handling inside Android and pass only page content and source metadata across the Kotlin/Rust boundary.
- Preserve private cache behavior when the feature is disabled.
- Use a deterministic per-URL Markdown filename and make the document self-describing so no index or asset files are needed.

**Non-Goals:**
- Move the permanent reading archive or its history/search behavior into shared storage.
- Copy or rewrite image resources in the selected folder.
- Add shared-folder caching to the Linux CLI.

## Decisions

### Use Android's Storage Access Framework for the selected folder

Use the system document-tree picker and retain its granted URI permission. Store the URI and feature toggle with Android settings because the URI is a platform capability, not a filesystem path suitable for Rust `std::fs`. Kotlin will locate, read, replace, and create `.md` documents through `ContentResolver`/`DocumentsContract`.

Alternative considered: request broad shared-storage access and pass a path into Rust. This is not compatible with scoped storage and grants more access than the selected-folder behavior needs.

### Bridge Markdown through the Android core API

On a normal article open, Kotlin should ask Rust for the canonical article identity (URL without fragment) and query the selected folder for its corresponding Markdown file. A valid hit is supplied to Rust as cached Markdown plus its source and saved-time metadata, which Rust turns into a `Document` for the usual page model and history flow. A miss or unreadable/invalid copy follows the existing fetch-and-extract path. After a successful network open, Rust returns the document's Markdown and metadata for Kotlin to write to the selected folder.

The private Rust weekly cache remains active only when shared-folder caching is off. Refresh bypasses shared-cache lookup, and a successful fetch replaces the same URL's file. Failures to access shared storage must not prevent an online fetch or crash the app; they should be surfaced as a storage notice where appropriate.

Alternative considered: copy every SAF document into an app-private temporary path and reuse `Address::File`. That would treat a cached article as a local file, losing its original URL identity and normal article history semantics unless more adapter logic were added.

### Store one plain Markdown file per URL

Use a stable filename derived from the canonical URL (URL fragments excluded), with a `.md` extension. Include source URL, title, and saved time as Markdown front matter, followed by the extracted Markdown body. On refresh, replace the document with the same filename. The file contains no binary image data and no auxiliary index is required; the URL in front matter can validate a filename match and preserve source identity.

Alternative considered: dated host/title paths matching the permanent archive. They make historical copies easy to browse, but conflict with the agreed one-current-copy cache semantics and require scanning to find a URL's latest entry.

### Keep feature settings in Android

Default the shared-cache toggle off. The folder picker is launched from settings; cancellation leaves the existing state unchanged. Disabling the feature keeps the selected folder remembered for later re-enabling but routes cache operations back to the private cache. The selected URI remains platform-side; existing Rust `Settings` fields need not serialize it.

## Risks / Trade-offs

- [A document provider may be slow, unavailable, or revoke access] → Treat reads as misses, keep failures non-fatal, and fall back to network; allow the reader to reselect a folder in settings.
- [The shared Markdown file lacks the private cache's structured site navigation and image byte cache] → Rebuild the normal page model from Markdown and source metadata; do not promise offline image availability from this store.
- [URL-derived filenames can collide] → Verify the source URL stored in front matter before accepting a hit; use a collision-resistant digest and deterministic disambiguation if needed.
- [External apps can edit or delete cached copies] → Validate metadata and Markdown on read; treat invalid or absent documents as cache misses.

## Migration Plan

No data migration is needed. Existing private cache and permanent archive remain untouched. The feature starts disabled; selecting a folder begins writing shared Markdown for subsequently fetched articles. Disabling or revoking access returns cache reads to the private cache path.

## Open Questions

- None that change the agreed behavior. The concrete digest and front-matter validation limits can be selected during implementation.
