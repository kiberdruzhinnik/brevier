# Design

## Context

See `proposal.md` for motivation and `specs/reading-notes/spec.md` for the behavior contract. User annotations are stored in `notes.tsv` beside app data, with attached pictures in a sibling `notes/` directory. Android exposes annotation actions through the Rust core bridge, reader view, shelf, menus, and settings. Article saves currently append annotations only when the reader explicitly saves an article through the system file picker; those external paths are not retained. The app's internal read archive stores article Markdown separately and does not append annotations. HTML article footnotes use a separate extraction path and remain part of reading.

## Goals / Non-Goals

**Goals:**
- Remove user-created note behavior from Android and the Rust core.
- Delete app-owned annotation data once when the updated app initializes.
- Keep the internal article archive, bookmarks, history, progress, article images, and article footnotes intact.
- Keep future article saves note-free.

**Non-Goals:**
- Finding or editing article files already exported to user-selected locations.
- Removing article-authored footnotes or other ordinary uses of the word “note.”
- Removing generic archive, Markdown, image, or ZIP support used by other features.

## Decisions

- **Delete only known app-owned annotation paths.** Remove `notes.tsv` and the note-picture directory under the app data directory during upgrade initialization. Do not scan user-selected storage: the app does not retain export paths, and scanning would risk altering unrelated user files.
- **Remove the annotation subsystem end to end.** Delete the Rust user-note module and bridge actions, then remove Kotlin note models, controls, render state, exports, and settings. Remove note attachment from article saving so its existing default save path writes only the article document and its own images.
- **Keep article footnotes separate.** The extraction and Markdown conversion footnote machinery is distinct from user annotations and stays in place, with tests protecting its current output.
- **Retire the obsolete capability requirements.** Remove note presentation, attachment, and export requirements from `reading-notes`; retain migration and note-free save behavior as the capability's remaining contract.

Alternatives considered:
- Keeping note data hidden for possible restoration would contradict the request to delete existing notes and leave unnecessary private data on the device.
- Scanning exported files for appended note sections could modify unrelated files, and the app has no recorded paths to limit such a scan.
- Removing every footnote-related code path because it shares “note” terminology would break article content and is outside the request.

## Risks / Trade-offs

- **[Risk]** A failed filesystem deletion could leave annotation data behind. **Mitigation:** make initialization cleanup explicit and test removal of both the record file and picture directory; surface or log cleanup failure according to existing storage error handling.
- **[Risk]** Broad removal may accidentally affect article footnotes or article images. **Mitigation:** keep extraction footnotes and ordinary article-save image handling separate, and retain regression coverage for both.
- **[Trade-off]** Old external exports retain annotations. **Mitigation:** document this boundary; Brevier cannot identify those files because their selected paths are not stored.

## Migration Plan

On the first launch after upgrade, remove the app-owned `notes.tsv` and `notes/` picture directory. Then initialize the app without the note subsystem. Leave the internal archive and all other app data in place. No rollback restoration is planned because note deletion is intentional; old external exports remain under the reader's control.
