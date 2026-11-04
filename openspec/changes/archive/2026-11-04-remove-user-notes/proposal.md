# Proposal

## Why

User-created notes add a large feature surface to the Android interface and Rust core that is no longer wanted. Removing them should also remove stored annotations without sacrificing saved articles or article-authored footnotes.

## What Changes

- **BREAKING** Remove user-created notes, attached pictures, note controls, and note sharing/export from the Android interface and Rust core.
- Delete existing note records and their separately stored pictures from Brevier's app storage when the updated app starts.
- Keep saved articles and make future article exports contain article content and article images without user notes or note pictures.
- Leave old article files already exported to user-selected locations untouched; Brevier does not retain their paths.
- Preserve article-authored footnotes as part of normal extraction and reading.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `reading-notes`: Remove user-created note behavior and specify cleanup of stored notes and note content in saved article copies.

## Impact

Affected areas include `src/notes.rs`, Android note APIs and UI, Rust save/archive integration, `reading-notes` specifications, persisted app data (`notes.tsv` and note pictures), and saved article bundles that contain note sections or note images. Article footnote extraction and rendering remain supported.
