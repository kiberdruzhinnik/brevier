# Proposal

## Why

Status notices such as the message that an article came from cache are placed at the bottom edge of the screen. When the reader moves the toolbar to the bottom, it can cover the notice and make it harder to read or act on. Readers also need a direct way to dismiss a notice before its timeout.

## What Changes

- Place status notices above the toolbar when the toolbar is at the bottom, while keeping them at the bottom edge when the toolbar is at the top.
- Allow a reader to swipe a status notice away.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `android-toolbar-position`: specify status notice placement relative to the toolbar and swipe dismissal.

## Impact

- Android status notice layout and touch handling in `MainActivity.kt`.
- The `android-toolbar-position` behavior specification.
