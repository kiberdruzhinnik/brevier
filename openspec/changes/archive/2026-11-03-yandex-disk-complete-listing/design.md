# Design

## Context

See proposal.md for the observed incomplete listing. `src/hosts/yandex_disk.rs` builds one public-resource metadata URL and turns one response's `_embedded.items` array into Atom. `src/fetch.rs` requests that URL once. The supplied share reports 44 total items but the API returns its default 20-item page; requesting `limit=1000` returns all 44.

## Goals / Non-Goals

**Goals:**
- Include every child entry reported for the requested public folder while preserving the existing Atom listing and URL behavior.
- Fetch further metadata only when the requested folder has entries beyond the current page.
- Avoid presenting a partial list when pagination fails or its metadata cannot be validated.

**Non-Goals:**
- Recursively enumerate subfolders. Each nested folder remains a separate reader-requested listing.
- Change PDF metadata/download handling, Cloud Mail listings, or general feed behavior.

## Decisions

- Request a page size of 1,000 and use the response's `_embedded.total` and `_embedded.offset` to determine whether more entries remain. The supplied folder accepts this size and returns all 44 entries in one request. If `total` exceeds the returned items, request subsequent pages at the next offset until the total is covered.
  - Alternative: keep the API default and fetch one page at a time. This is simpler but makes an avoidable extra request for each batch of 20 entries.
  - Alternative: request only one large page. This handles the supplied folder but silently truncates larger folders, so it does not meet the all-entries requirement.
- Keep pagination in the existing Yandex share fetch path and combine the validated page entries into the existing Atom listing. Preserve page order and the original folder URL as the feed's self link.
- Require page metadata and progress to be consistent with the requested offset and reported total. Propagate request errors and use the existing extraction failure for malformed or non-progressing pages rather than returning an incomplete listing.
- Make no request for pages after the reported total is collected. Each request is solely for the folder the reader opened; nested folders are not fetched until opened.

## Risks / Trade-offs

- A folder can change while pages are being fetched, making its total or offsets inconsistent. → Fail with the existing extraction error rather than display a partial or duplicated list.
- A very large folder can require multiple API calls and produce a large listing. → Request 1,000 entries per page, retain the existing per-response body limit, and stop as soon as the reported total is collected.

## Migration Plan

No migration is needed. Existing folder URLs and link formats remain unchanged.
