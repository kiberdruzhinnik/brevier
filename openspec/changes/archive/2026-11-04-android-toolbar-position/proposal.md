# Proposal

## Why

Readers who prefer browser-style bottom controls cannot move Brevier's toolbar from the top. A setting will let each reader choose the toolbar position while preserving the current top position by default.

## What Changes

- Add a setting to place the entire Android toolbar at the top or bottom of the screen, defaulting to the top.
- Keep the reading progress strip at the edge opposite the toolbar, and retain its current visibility behavior for content that scrolls beyond the screen.
- When toolbar auto-hide is enabled, have it slide away from the reading content according to its selected position.

## Capabilities

### New Capabilities

- `android-toolbar-position`: User-selectable placement of the Android toolbar and corresponding reading progress strip.

### Modified Capabilities

None.

## Impact

Android settings and toolbar layout, article content insets, progress strip placement, and toolbar auto-hide behavior. No new dependencies or public APIs are expected.
