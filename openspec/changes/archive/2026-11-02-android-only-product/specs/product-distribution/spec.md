# Spec Delta

## Purpose

Defines which Brevier applications are offered to end users and preserves a Linux-only CLI for maintainers' parser checks.

## ADDED Requirements

### Requirement: Android is the only supported end-user application
Brevier SHALL offer Android as its only installable application for end users.

#### Scenario: Get a Brevier app release
- **WHEN** a reader gets a public Brevier application release
- **THEN** the release provides the Android app and does not offer desktop application packages

### Requirement: The Linux CLI is reserved for internal parser validation
Brevier SHALL keep a Linux CLI for maintainers to check parser behavior on Linux hosts, but SHALL NOT distribute the CLI as a public product through binary releases, crates.io, or the GitHub Action.

#### Scenario: Check parser behavior on the VPS
- **WHEN** a maintainer checks a parser on the Linux VPS
- **THEN** the maintainer can build and run the CLI against a page or public share

#### Scenario: Inspect public Brevier releases
- **WHEN** a reader checks Brevier's public release channels
- **THEN** they find no CLI downloads, crates.io CLI installation, or Brevier GitHub Action
