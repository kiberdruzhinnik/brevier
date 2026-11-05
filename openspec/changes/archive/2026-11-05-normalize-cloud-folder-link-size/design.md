# Design

## Context

Cloud provider folder responses are converted into Atom and then pass through the shared feed renderer. That renderer represents linked entries as level-two Markdown headings, which the Android reader typesets larger than regular text. See proposal.md and the `public-cloud-shares` spec delta for the desired behavior.

## Goals / Non-Goals

**Goals:**
- Keep folder entry links in regular text while preserving the folder title as a page heading.
- Keep ordinary feed rendering unchanged.

**Non-Goals:**
- Change typography for site feeds or discussion feeds.
- Change public share extraction, navigation, or provider APIs.

## Decisions

- Identify provider folder listings at the feed-to-Markdown boundary using the source URL already supplied to `from_feed`. Render their entry titles as inline Markdown links in regular paragraphs rather than heading links. This keeps provider handling scoped to the cloud share path and avoids changing the shared rendering of ordinary feeds.
- Preserve the existing folder title heading and link destinations. The listing remains a feed document; only the Markdown block style for its entries changes.
- Add focused coverage for both Cloud Mail and Yandex Disk URLs, plus a regular feed case to guard against a global typography change. This is preferable to changing all feed entry headings, which would alter unrelated feed presentation.

## Risks / Trade-offs

- A provider URL classification error could affect the wrong feed. Keep the match limited to the exact public share hosts and cover both recognized hosts and an ordinary feed in tests.

## Migration Plan

No migration is required. The rendering change takes effect when the app loads a public folder listing; rollback is the corresponding code revert.
