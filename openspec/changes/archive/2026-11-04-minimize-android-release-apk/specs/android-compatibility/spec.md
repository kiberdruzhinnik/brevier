# Spec Delta

## ADDED Requirements

### Requirement: Android release APKs contain arm64 code only
Brevier SHALL build Android release APKs with arm64-v8a native code only and SHALL retain arm64-v8a and x86_64 support in debug builds.

#### Scenario: Build a release APK with default settings
- **WHEN** a maintainer builds the Android release variant without selecting an ABI
- **THEN** the APK contains the arm64-v8a native library and no x86_64 native library

#### Scenario: Request a non-arm64 release ABI
- **WHEN** a maintainer requests an x86_64 or multi-ABI Android release build
- **THEN** the build reports that release APKs support arm64-v8a only instead of producing a larger or incompatible release APK

#### Scenario: Build for an Android emulator
- **WHEN** a maintainer builds the Android debug variant with its default ABI selection
- **THEN** the debug APK retains arm64-v8a and x86_64 support for phone and emulator testing
