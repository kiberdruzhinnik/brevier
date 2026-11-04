# Spec Delta

## Purpose

Keeps the Rust core and Android application on one release version so builds, diagnostics, and published APK metadata identify the same Brevier release.

## ADDED Requirements

### Requirement: Core and Android release versions match
The Rust core package and Android application SHALL identify the release as version 0.22.0, and the Android version code SHALL correspond to that release as 2200.

#### Scenario: Inspect release version metadata
- **WHEN** a maintainer inspects the core package and Android APK metadata for the release
- **THEN** both identify version 0.22.0 and the APK version code is 2200
