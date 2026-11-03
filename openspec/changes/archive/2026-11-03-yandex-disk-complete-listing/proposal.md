# Proposal

## Why

Public Yandex Disk folders with more than 20 entries currently appear incomplete because Brevier reads only the API's default first page. Readers need the listing to include every accessible entry in the folder they opened.

## What Changes

- Request larger pages for public Yandex Disk folder metadata and follow the API's offset and total metadata until all entries are collected.
- Preserve the existing folder listing links, order, and safe behavior when metadata cannot be read.

## Capabilities

### New Capabilities

### Modified Capabilities
- `public-cloud-shares`: Require public Yandex Disk folder listings to include all entries across API pages.

## Impact

Yandex Disk public-resource URL construction and metadata-to-listing conversion in `src/hosts/yandex_disk.rs`, the Yandex fetch path in `src/fetch.rs`, and focused host/API fixtures and tests.
