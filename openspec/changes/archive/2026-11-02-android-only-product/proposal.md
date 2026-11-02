# Proposal

## Why

Brevier's desktop apps and public CLI distributions create release, packaging, and CI work outside the Android product the maintainer intends to support. Retiring those public surfaces leaves more effort for Android while preserving a Linux CLI for internal parser checks on the VPS.

## What Changes

- **BREAKING**: End user releases become Android-only; stop publishing Linux and Windows desktop apps and cross-platform CLI binaries.
- **BREAKING**: Stop publishing the standalone CLI crate and remove the public GitHub Action.
- Keep the Linux CLI in the repository for VPS parser validation, corpus checks, and internal Linux CI; do not advertise or release it as an end-user product.
- Remove GTK desktop builds, desktop packaging, and desktop-specific CI/release jobs while retaining Android build, release, and F-Droid distribution.
- Update product documentation and the project website to describe Android as the only supported app platform.
- Update reading-notes behavior to describe Android's inline note placement without a desktop margin.

## Capabilities

### New Capabilities
- `product-distribution`: Android is the only end-user app and release platform; the Linux CLI is retained for internal parser checks only.

### Modified Capabilities
- `reading-notes`: Specify Android inline note placement and remove the desktop notes-margin behavior.

## Impact

The Cargo package and CLI build configuration, GTK UI modules, CI and release workflows, Linux/Windows packaging, crates.io publication, GitHub Action files, README and website copy, and `reading-notes` spec. The Rust core and Linux CLI remain available to internal tests and VPS parser checks; Android continues to build the core through the NDK and ship through its existing release channels.
