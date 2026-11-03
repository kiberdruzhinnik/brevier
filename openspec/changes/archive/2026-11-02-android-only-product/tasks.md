# Tasks

## 1. Retire desktop and public CLI products

- [x] 1.1 Remove the GTK desktop application, its Cargo feature and target wiring, and Linux/Windows desktop packaging; verify the Android target and Linux CLI still compile without GTK.
- [x] 1.2 Remove Linux/Windows CLI release artifacts, Flatpak publication, crates.io publishing, and the public GitHub Action; verify release configuration exposes only Android artifacts and no CLI publication steps.
- [x] 1.3 Update README, website, packaging, and release documentation to describe Android as the only public app and remove retired desktop/CLI/Action install instructions; verify public-facing references are gone while Pages generation still succeeds using its internal Linux CLI.

## 2. Preserve internal parser checks and Android delivery

- [x] 2.1 Keep a Linux CLI build path for VPS and internal CI parser checks, including public-share parsing; verify it can be built and run against the saved Cloud Mail and Yandex share examples and the corpus check.
- [x] 2.2 Retain Android NDK and release build paths and update any affected build configuration; verify the Android release build produces an installable APK for the existing supported Android versions.
- [x] 2.3 Remove desktop-only and public Action CI jobs while retaining Linux core/parser checks and Android build checks; verify the resulting workflow definitions parse and the retained jobs run successfully.

## 3. Align reading notes with Android-only support

- [x] 3.1 Sync the revised Android inline note-placement requirement into `openspec/specs/reading-notes/spec.md`, preserving picture, export, and save requirements; verify the main spec no longer specifies a desktop margin.
- [x] 3.2 Update any product documentation describing notes in desktop windows and retain Android note documentation; verify repository searches show no user-facing desktop note-placement guidance.

## 4. Integration verification

- [x] 4.1 Run formatting, core/parser checks, corpus checks, and the Android APK build; inspect console output and confirm no desktop or public CLI artifacts are generated. Skip GTK UI and Android emulator tests as requested.
- [x] 4.2 Search workflows, release scripts, Cargo metadata, package manifests, README, and site content for retired public surfaces; verify remaining CLI references are internal Linux parser/site-generation uses.
