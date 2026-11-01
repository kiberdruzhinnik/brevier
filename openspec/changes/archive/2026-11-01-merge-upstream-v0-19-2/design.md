# Design

## Context

The branch shares upstream history through the plain-Markdown archive change. It already contains that archive behavior plus later Cloud Mail and Yandex Disk support. Upstream changes after the shared point add a desktop notes margin, note pictures and export, responsive layout fixes, Android API 36 back handling, and ZIP 9. The existing note model anchors text to quotes in `notes.tsv`; the core has no toolkit dependency, and GTK and Android each render the shared page model in their own native UI.

## Goals / Non-Goals

**Goals:**

- Integrate the upstream v0.19.0, v0.19.1, and v0.19.2 changes that are not already present.
- Preserve the current cloud-share parser behavior and its OpenSpec requirements.
- Keep Android `minSdk = 24`; Android 12 remains supported while the app targets API 36.
- Keep existing notes, page copies, archive behavior, and back-navigation order working.

**Non-Goals:**

- Import notes from Markdown or synchronize notes between devices; upstream explicitly leaves import for later.
- Read or extract ZIP archives. ZIP support remains for writing saved pages and note exports.
- Change the core to depend on GTK or Android UI toolkits.

## Decisions

1. **Integrate from the shared upstream history, not by replacing the fork with the release tree.** Apply all changes after the common archive commit that are missing locally. Reconcile `Cargo.toml`, lockfiles, README, and packaging source metadata with both sets of dependencies and existing Cloud Mail/Yandex Disk documentation. Keep the current public-cloud-share spec and its behavior intact.

2. **Extend the existing quote-anchored notes model.** Preserve quote, surrounding context, page address, and note text as the basis for locating notes. Store attached image bytes alongside the existing notes data, and make note edits, removals, and page/all-note exports keep image references consistent. Retain existing notes and their archive copies when opening data written by this version.

3. **Keep presentation native in each front end.** GTK places notes in a responsive margin when there is room and under the related paragraph when there is not; Android keeps notes under paragraphs and offers images through the system photo picker. Shared note placement and export data stay in the core, while each front end owns its controls and layout.

4. **Keep the Android minimum and raise only the target.** Continue to build with `minSdk = 24` and `compileSdk`/`targetSdk = 36`. Use the back-invoked dispatcher on Android 13 and later and the existing `onBackPressed` path on older versions, with one shared handler preserving the current order of overlays, tab history, and leaving the app.

5. **Use ZIP 9 only for writing.** Update the direct dependency and lockfiles for the upstream advisory fix. Do not add ZIP reading, which the application does not need.

6. **Do not reapply the plain-Markdown archive feature.** It is already present at the shared commit; port only any upstream changes that modify it after that point.

## Risks / Trade-offs

- **[Risk]** Image references can become stale when notes are edited or removed. **Mitigation:** keep image storage and note updates in one core operation, and cover persistence, deletion, and export with tests.
- **[Risk]** README, dependency metadata, and lockfiles contain both upstream and downstream edits. **Mitigation:** merge those files deliberately and verify the locked dependency graph, source package list, and Cloud Mail/Yandex Disk documentation afterward.
- **[Risk]** Target API 36 changes system Back delivery on Android 16. **Mitigation:** centralize Back behavior and verify the legacy path remains active below Android 13; test installation on Android 12 without raising the minimum.
- **[Risk]** The notes margin can constrain narrow or tiled windows. **Mitigation:** move notes below their passages when the available width is insufficient and keep article centering independent of an empty margin.

## Migration Plan

No database conversion is intended. Existing quote-anchored notes and saved article copies must remain readable. New note images are stored with the notes and included in exports; an older app may not display newly attached pictures, but Android installability and the existing notes format remain unchanged.
