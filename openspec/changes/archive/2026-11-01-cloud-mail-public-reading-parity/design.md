# Design

## Context

The Cloud Mail adapter in `src/hosts/cloud_mail.rs` builds public folder and file API URLs, converts folder JSON to Atom, resolves download shards, and recognizes PDF metadata. `src/fetch.rs` currently enters the file path only when the public URL ends in `.pdf`; the folder and PDF paths use optional parsing, so failures can continue to ordinary HTML extraction. The API may provide a filename without MIME metadata. The shared PDF extractor and Android bridge already consume the core's text page model.

## Goals / Non-Goals

**Goals:**
- Make one Cloud Mail resource classification path drive folder, PDF, unsupported, and unavailable outcomes.
- Exercise provider-controlled URLs and response parsing with saved fixtures, unit tests, and a seeded fuzz target.
- Preserve the existing transport limits, TLS verification, Atom listing renderer, and shared PDF extraction path.

**Non-Goals:**
- Execute Cloud Mail JavaScript, use credentials, or add a provider-specific PDF extractor.
- Add new runtime dependencies or alter Yandex Disk behavior.
- Run UI-feature checks or Android emulator/device tests; these are excluded by the requested verification scope.

## Decisions

1. **Classify from Cloud Mail API metadata, not the share URL suffix.** Use the provider's file/folder metadata to choose folder listing, PDF download, or unsupported content. Treat an API-reported `.pdf` filename as sufficient when MIME metadata is absent. This also allows file shares without a `.pdf` URL suffix. Keep URL structure parsing only for extracting the public weblink. The alternative—adding more suffix heuristics—cannot handle extensionless shares or reliably distinguish a folder from a file.

2. **Make public-share dispatch outcomes explicit.** Once a URL is recognized as a Cloud Mail public share, malformed, unavailable, inaccessible, and unsupported API responses must become existing Brevier read or unsupported-content errors rather than falling through to the provider shell. Continue using the existing access/error wording and do not add a login or credential flow. This limits behavior changes to public share routing while avoiding shell extraction as a false success.

3. **Keep the existing folder and document renderers.** Convert valid folder responses to Atom for the existing listing reader, preserve nested public links, and send validated PDF bytes to the shared local extractor. This keeps CLI and Android output aligned through the Rust core. A separate provider renderer would duplicate established behavior and risk front-end drift.

4. **Test parser boundaries with fixtures and fuzzing.** Save minimized real-shape folder, file, dispatcher, and malformed/error responses as fixtures; unit-test URL parsing, classification, Atom links, and download URL validation. Add a libFuzzer target that invokes the public Cloud Mail parsing functions on arbitrary UTF-8-lossy bytes and seed its corpus from the fixtures. Do not persist live temporary download URLs or share secrets in seeds.

## Risks / Trade-offs

- [Provider response shapes change] → Keep fixtures representative, treat absent or malformed fields as read failures, and fuzz each parser entry point.
- [API metadata calls add latency for files whose URL lacks a suffix] → Request only the metadata needed to classify the recognized public share, and reuse that response for download decisions.
- [Public share availability can vary by plan or owner settings] → Validate with the supplied share and report access failures using existing behavior; do not infer availability from a successful metadata response alone.
- [A filename ending in `.pdf` may contain non-PDF bytes] → Retain the existing PDF extractor's content validation and extraction error behavior.

## Migration Plan

No data migration is required. Implement the metadata-first routing and tests, run the specified console and Rust checks, build the Android debug APK, and inspect the APK output. Rollback is a source revert; no server-side state or persisted format changes.
