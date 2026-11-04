# Spec Delta

## Purpose

Preserves article pictures that have already been downloaded when readers save articles, so saved reading remains useful offline without fetching additional media.

## ADDED Requirements

### Requirement: Saved articles retain fetched pictures
When a reader saves an article, Brevier SHALL retain its already-downloaded article pictures with that saved article, subject to a maximum of 32 MB of pictures per article.

#### Scenario: Save an article with downloaded pictures
- **WHEN** the reader saves an article whose pictures have already been downloaded
- **THEN** the saved article includes those pictures up to the per-article limit and makes no additional network requests

#### Scenario: Article pictures exceed the limit
- **WHEN** the total downloaded pictures for an article exceed 32 MB
- **THEN** the saved copy retains no more than 32 MB of those pictures

### Requirement: Saved article pictures stay with the saved article
Article pictures retained for a saved article SHALL be available when that saved article is reopened and SHALL not be included in Android cloud backup.

#### Scenario: Reopen a saved article
- **WHEN** the reader opens a saved article while offline
- **THEN** retained article pictures remain available

#### Scenario: Android backup runs
- **WHEN** Android backs up application data
- **THEN** retained article pictures are excluded from that backup
