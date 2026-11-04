# Reading Notes Specification

## Purpose

Lets readers keep notes and pictures beside the passages they annotate, then export those notes in a portable form.

## Requirements

### Requirement: Existing user notes are deleted on upgrade
Brevier SHALL delete previously stored user-created note records and their attached pictures when the updated app initializes.

#### Scenario: Upgrade with stored notes
- **WHEN** a reader opens the updated app with notes from an earlier version
- **THEN** the stored note records and note pictures are deleted while article copies, bookmarks, reading history, and reading progress remain available

### Requirement: Saving an article excludes user notes
Brevier SHALL save article content and article images without user-created note text, passage marks, or attached note pictures.

#### Scenario: Save an article after notes are removed
- **WHEN** a reader saves an article in the updated app
- **THEN** the saved article contains its article content and article images without user-created notes or note pictures
