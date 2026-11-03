# product-distribution Specification

## Purpose

Defines which Brevier applications are offered to end users and preserves a Linux-only CLI for maintainers' parser checks.

## Requirements

### Requirement: Android is the only supported end-user application
Brevier SHALL offer Android as its only installable application for end users and SHALL distribute the Android APK exclusively through GitHub Releases, with no project website or F-Droid channel.

#### Scenario: Get a Brevier app release
- **WHEN** a reader looks for Brevier project information or an app release
- **THEN** the project information is available in the GitHub repository and the Android APK is available from GitHub Releases, with no project website or F-Droid listing
- **AND** the release does not offer desktop application packages

### Requirement: The Linux CLI is reserved for internal parser validation
Brevier SHALL keep a Linux CLI for maintainers to check parser behavior on Linux hosts, but SHALL NOT distribute the CLI as a public product through binary releases, crates.io, or the GitHub Action.

#### Scenario: Check parser behavior on the VPS
- **WHEN** a maintainer checks a parser on the Linux VPS
- **THEN** the maintainer can build and run the CLI against a page or public share

#### Scenario: Inspect public Brevier releases
- **WHEN** a reader checks Brevier's public release channels
- **THEN** they find no CLI downloads, crates.io CLI installation, or Brevier GitHub Action
