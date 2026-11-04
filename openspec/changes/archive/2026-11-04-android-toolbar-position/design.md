# Design

## Context

See proposal.md for motivation and specs/android-toolbar-position/spec.md for the behavior contract. The Android toolbar currently overlays the article at the top, and the article reserves space there. The progress strip is the final row below the article and is hidden when the article fits. Toolbar auto-hide applies a negative vertical translation and reduces the article's covered top area. Settings are persisted through the Rust core and exposed to Kotlin as JSON.

## Goals / Non-Goals

**Goals:**

- Represent top/bottom placement as a persisted setting, with top as the fallback for existing installations.
- Place the full toolbar and progress strip at opposite ends of the usable window, respecting system bars and the keyboard.
- Keep article visibility, scroll progress, and auto-hide behavior correct at either position.

**Non-Goals:**

- Changing toolbar contents, progress calculation, or the condition that hides progress for short articles.
- Changing Linux CLI behavior or introducing a second toolbar layout for different Android form factors.

## Decisions

- Persist position with the existing core settings rather than Android-only preferences. This keeps settings loading and saving on the existing path; an absent value maps to top, so old settings retain current behavior.
- Keep the toolbar as an overlay at either edge. Move the progress strip to the opposite end of the vertical content column. This preserves the existing article layout model and avoids a new container hierarchy.
- Generalize the article's reserved/covered edge. At top, reserve and cover the top; at bottom, reserve and cover the bottom. Progress visibility and visible reading offsets must use the remaining viewport after that reservation.
- Apply auto-hide translation toward the nearest outside edge: negative Y at the top, positive Y at the bottom. The amount of hidden toolbar still controls how much of its edge is considered covered by the article.
- Keep transient panels anchored to the toolbar and verify their placement at both edges. Dropdowns or popups that currently assume a top toolbar must open into the available screen area when the toolbar is at the bottom.

## Risks / Trade-offs

- [Changing the reserved edge can shift scroll and progress calculations] → Cover top and bottom placement, scroll endpoints, jumps, and short/long articles in regression checks.
- [Bottom placement can conflict with keyboard and navigation insets] → Use the already applied window insets when placing the bar and content, including IME insets.
- [Transient panels may open off-screen when anchored below a bottom toolbar] → Review each toolbar-anchored popup and choose its opening direction based on toolbar position.
- [A malformed or absent stored value could make settings inconsistent] → Parse only known positions and fall back to top.

## Migration Plan

No migration is needed. Existing settings without a toolbar position use top. Saving the setting adds its value through the existing settings persistence path; reverting the app leaves the extra setting value harmless to older versions.
