# Proposal

## Why

Android release APK size varies because local Gradle builds default to packaging both arm64-v8a and x86_64 native libraries, while the GitHub release workflow explicitly builds arm64-v8a only. Make release packaging consistent and keep emulator support in debug builds so a release APK does not unexpectedly double in size.

## What Changes

- Make release builds package arm64-v8a only by default and reject incompatible release ABI selections.
- Keep both arm64-v8a and x86_64 available to debug builds for phone and emulator testing.
- Enable and verify release resource shrinking where it safely removes unused app resources; retain existing R8 code shrinking and native-library stripping.
- Add a build check that inspects release APK ABI contents and reports artifact size, while confirming debug builds retain emulator support.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `android-compatibility`: Define the architecture included in release APKs and preserve supported debug ABIs.

## Impact

The Android Gradle configuration, release build workflow, and Android build documentation are affected. App runtime behavior and the Rust core API do not change. Release APKs become arm64-only; x86_64 remains available for debug and emulator builds.
