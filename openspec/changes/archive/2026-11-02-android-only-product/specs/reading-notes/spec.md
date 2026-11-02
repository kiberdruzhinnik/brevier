# Spec Delta

## MODIFIED Requirements

### Requirement: Notes stay beside the passages they annotate
Brevier SHALL show each note below the paragraph containing its annotated passage in the Android reader, regardless of available reading width.

#### Scenario: Show notes in a wide window
- **WHEN** a reader opens a page with notes in the Android app on a wide display
- **THEN** each note appears under its annotated paragraph rather than in a separate margin

#### Scenario: Show notes when the window is narrow
- **WHEN** a reader opens a page with notes in the Android app on a narrow display
- **THEN** each note appears under its annotated paragraph without preventing the reader from using the available width

#### Scenario: Read a page without notes
- **WHEN** a reader opens a page with no notes in the Android app
- **THEN** Brevier does not reserve empty space for a notes margin
