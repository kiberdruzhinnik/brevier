# Design

## Context

See `proposal.md` for motivation and `specs/product-distribution/spec.md` for the distribution contract. The current project site is the `site/` tree deployed by `.github/workflows/pages.yml`; repository metadata and historical release notes also point at its custom domain. `ROADMAP.md` is linked from the README, contributor and agent guidance, manifesto, site, and several source comments. F-Droid references and store assets are in Fastlane metadata, Android build comments/configuration, contributor and install guidance, release notes, and an archived OpenSpec change. The active remote is this fork's GitHub repository, `kiberdruzhinnik/brevier`.

The release workflow already signs Android APKs and verifies a second unsigned build for reproducibility. Android also uses a local Kotlin certificate-verifier module required by the Rust core's JNI integration.

## Goals / Non-Goals

**Goals:**

- Make the fork's GitHub repository and Releases the only maintained public project and APK surfaces.
- Remove the obsolete project site, roadmap, F-Droid publishing metadata, and their references.
- Keep Android installation continuity and the existing release integrity checks.

**Non-Goals:**

- Changing the Android application ID, Rust core, Android behavior, or the Linux CLI's internal parser-check role.
- Removing the local certificate-verifier module required by the Android runtime.
- Rewriting repository history or removing the upstream fork attribution.

## Decisions

- **Use GitHub Releases as the sole public Android distribution channel.** Update the existing `product-distribution` requirement; keep signed APK release and reproducibility jobs. The unsigned APK remains a verification input, not a distribution channel. Alternative: remove the unsigned build too; that would discard a useful integrity check without being required to stop F-Droid publishing.
- **Retire the project site instead of moving it.** Remove `site/` and the Pages workflow; retain the README as the repository's project information. Remove the Cargo homepage field, and point Cargo repository metadata and all active repository, release, and issue links at this fork. Put one upstream attribution in the README. Alternative: move the site to GitHub Pages; the user chose to remove the site and deployment.
- **Delete the roadmap and its references.** Remove current documentation links and source comments that cite it. Keep product behavior and durable constraints in the existing manifesto, agent guidance, and OpenSpec specs rather than replacing the roadmap with another planning document.
- **Remove F-Droid publication surfaces and specific accommodations.** Delete the complete `fastlane/` tree and F-Droid-only Android Gradle configuration/comments, and remove mentions from historical release notes and the archived change so repository searches find none. Keep generic release reproducibility and the local verifier source needed by Android, updating its comment to explain its runtime/version constraint rather than F-Droid.
- **Preserve the existing application ID.** Changing `io.github.gurov.brevier` would prevent the fork APK from updating an installation already using that ID. A distinct ID is outside this cleanup and would require a separate migration decision.

## Risks / Trade-offs

- [Readers lose the standalone site and F-Droid updates] → Put the fork attribution and GitHub Releases installation path prominently in the README; the spec makes the new channel explicit.
- [Removing or altering release checks could weaken APK provenance] → Keep the signed release and unsigned reproducibility comparison intact.
- [A leftover historical link or service mention keeps directing users upstream] → Search tracked files for the custom domain, roadmap references, and F-Droid names after cleanup, excluding unrelated uses of the word “roadmap” in external corpus data.
- [The same application ID can conflict with an upstream installation] → Preserve it for in-place update compatibility, as requested by the chosen default; reconsider only if side-by-side installation becomes a goal.

## Migration Plan

No data migration is needed. After implementation, GitHub Releases remains the APK source. Existing APKs keep the same application ID and signing path. Rollback consists of restoring the removed site, metadata, documentation, and workflow files from version control.
