# Spec Delta

## ADDED Requirements

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

## REMOVED Requirements

### Requirement: Notes stay beside the passages they annotate
**Reason**: User-created passage annotations are removed from Brevier.
**Migration**: Existing annotations are deleted when the updated app initializes.

### Requirement: Notes can keep pictures
**Reason**: User-created notes and their attached pictures are removed.
**Migration**: Existing note picture files are deleted from Brevier's app storage when the updated app initializes. Article images are unaffected.

### Requirement: Notes can be exported with their passages and pictures
**Reason**: Note export and sharing are part of the removed user-created notes feature.
**Migration**: Existing files already exported to user-selected locations remain untouched; Brevier does not retain their paths.

### Requirement: Saving an article includes its note pictures
**Reason**: Saved articles no longer include user-created note content or pictures.
**Migration**: Future article saves contain only article content and article images. Existing externally saved files remain untouched.
