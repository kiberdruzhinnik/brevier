# Design

## Context

See proposal.md for why APK sizes vary. The Android Gradle configuration currently shares one ABI list between native compilation and APK packaging, defaulting to `arm64-v8a` and `x86_64`. CI already passes `-Pabis=arm64-v8a` for both the signed release and its unsigned reproducibility build. Release code shrinking is enabled, and `android/core.sh` strips the Rust shared library before packaging.

## Goals / Non-Goals

**Goals:**

- Resolve ABI selection consistently for the requested build variant: release is arm64-only, while debug includes arm64 and x86_64 by default.
- Make an unsupported release ABI override fail before producing a misleading release artifact.
- Shrink unused Android resources in release builds without removing assets that the app loads by name.
- Make CI verify the release APK's ABI contents and record its final size.

**Non-Goals:**

- Change supported Android API levels or the Rust feature set.
- Remove bundled font assets or reduce their glyph coverage.
- Tune Rust optimization, LTO, or code generation without measurements showing a safe size benefit.
- Publish separate ABI APKs or an Android App Bundle.

## Decisions

### Select ABIs by requested build variant

Use arm64-v8a as the only default for release builds and retain arm64-v8a plus x86_64 as the default for debug builds. Keep the existing `-Pabis` mechanism for debug/test builds, but reject release selections other than arm64-v8a. Resolve the list once and feed the same value to both the Rust build task and Android packaging so the APK cannot contain a different set of native libraries than the build produced.

This keeps ordinary emulator builds convenient and makes `assembleRelease` predictable without relying on each caller to remember a command-line property. A global arm64-only default would also shrink APKs, but would make debug emulator builds fail unless every maintainer remembered to override it.

### Shrink Android resources only in release

Enable Android resource shrinking together with the existing R8 code shrinking. The bundled font files are assets loaded by filename and are outside Android's resource shrinker; keep them intact. Inspect the release APK's resource and asset entries and run the app through its main reading and settings flows to catch any resource that is discovered dynamically.

### Verify the artifact, not only the build command

Build release and debug APKs through the same paths contributors and CI use. Inspect the release ZIP entries and fail CI if any native ABI other than `arm64-v8a` is present. Inspect the debug APK to ensure both supported ABIs remain. Record compressed APK sizes before and after so the ABI reduction and any resource-shrinking gain are distinguishable; do not add a brittle absolute size limit.

## Risks / Trade-offs

- [Risk] Gradle task names or explicit ABI properties may resolve differently for combined debug and release invocations → [Mitigation] define and test the selection rules for release-only, debug-only, and combined tasks, and reject any release configuration that is not arm64-only.
- [Risk] Resource shrinking can remove resources accessed indirectly → [Mitigation] keep named font assets intact, inspect the packaged entries, and exercise reader, settings, and icon flows in the release APK.
- [Trade-off] Release APKs will no longer install on x86_64-only Android devices → [Mitigation] this is the confirmed release target; x86_64 remains supported in debug builds for emulator testing.

## Migration Plan

No user data migration is required. Make the Gradle variant selection deterministic, enable release resource shrinking, align the release and reproducibility CI commands with the new defaults, document the debug ABI override, and verify signed and unsigned release contents. Rollback by reverting the Gradle and workflow changes if resource shrinking or release ABI validation breaks a supported flow.
