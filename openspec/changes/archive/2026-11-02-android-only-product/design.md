# Design

## Context

See `proposal.md` for motivation and `specs/product-distribution/spec.md` plus `specs/reading-notes/spec.md` for the required behavior. The Rust core is shared by the Android app and Linux CLI. Android builds the core through the NDK. The Linux CLI also supports corpus checks and Pages generation, so removing public distribution must not remove its internal build path. GTK, desktop packages, and their release jobs are separate from the Android build.

## Goals / Non-Goals

**Goals:**

- Separate internal Linux parser tooling from public product distribution.
- Keep Android build and release paths working while removing desktop distribution paths.
- Keep site generation and parser validation working where they rely on the Linux CLI.
- Align the reading-notes capability with the Android reader's inline placement.

**Non-Goals:**

- Removing parser/core functionality or the Linux CLI source used by maintainers.
- Changing Android compatibility requirements, app behavior beyond the notes requirement, or the public-cloud-share parsing contract.
- Replacing the project website or its internal content-generation workflow.

## Decisions

- **Keep the CLI buildable from the repository, remove its public delivery surfaces.** Keep its binary target and internal Linux CI/corpus/VPS usage. Remove release artifacts and installer paths for CLI binaries, crates.io publishing, and the public GitHub Action. Alternatives considered: delete the CLI entirely, which would break the stated VPS parser workflow; or leave public distribution in place, which conflicts with Android-only product scope.
- **Retire GTK and desktop packaging independently of the shared core.** Remove the GTK application and its build/package/release integration, including Linux and Windows desktop products and Flatpak publication. Retain the core library and Android's NDK build. This avoids coupling Android maintenance to desktop UI code while preserving parser code shared by Android and internal checks. Alternative considered: keep desktop code dormant, which leaves unnecessary build and maintenance surfaces.
- **Retain the website but revise its product-facing content.** Remove public desktop, CLI installation/download, and Action instructions. Keep Linux CLI usage inside the Pages workflow for generating or checking the site when needed. Alternative considered: retire the website with the desktop products; that is outside the requested product change and could remove useful project information.
- **Describe notes at the supported reader level.** Update the existing reading-notes requirement to specify placement below the annotated paragraph in Android, consistent with the app behavior. Preserve other note capabilities and storage requirements unchanged.
- **Keep internal verification Linux and Android-focused.** Retain relevant Linux core/parser checks and Android build/release checks; drop GTK, Windows, desktop packaging, and public Action checks that exist only for retired surfaces. Ensure release configuration no longer publishes retired artifacts or triggers a desktop release matrix.

## Risks / Trade-offs

- [An overlooked workflow or website page may continue to advertise retired products] → Search repository documentation, workflow files, release scripts, and site content for desktop, CLI distribution, crates.io, and Action references; remove or rewrite public-facing references while retaining internal usage.
- [Removing a shared Cargo feature or binary target could break Android or parser checks] → Keep core and Linux CLI build paths; verify Android and internal Linux checks after implementation.
- [Release automation may still publish stale assets or package metadata] → Review the complete release workflow and packaging directory, then inspect a generated release configuration before publishing.
- [The reading-notes spec could accidentally discard unrelated note behavior] → Replace only the placement requirement and preserve other requirements in the existing capability.

## Migration Plan

1. Remove desktop and public CLI publication paths, GTK integration, and retired public documentation while preserving internal Linux CLI uses.
2. Keep Android CI/release/F-Droid and Linux parser-check paths; update `reading-notes` main spec when syncing the change.
3. Validate repository references and workflow configuration, run the relevant Linux and Android checks, and produce the Android APK as the user-facing artifact.
4. Rollback by restoring the retired distribution/workflow files from version control if Android or internal parser checking is blocked; no data migration is required.
