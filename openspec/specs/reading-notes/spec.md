# Reading Notes Specification

## Purpose

Lets readers keep notes and pictures beside the passages they annotate, then export those notes in a portable form.

## Requirements

### Requirement: Notes stay beside the passages they annotate
Brevier SHALL show each note near its annotated passage and adapt its placement to the available reading width.

#### Scenario: Show notes in a wide window
- **WHEN** a reader opens a page with notes in a window wide enough for a margin
- **THEN** Brevier places each note beside the line where its passage begins

#### Scenario: Show notes when the window is narrow
- **WHEN** the available width is too small for a notes margin
- **THEN** Brevier places each note under its annotated paragraph without preventing the reader from narrowing the window

#### Scenario: Read a page without notes
- **WHEN** a reader opens a page with no notes in a wide window
- **THEN** Brevier keeps the article centered instead of reserving an empty notes margin

### Requirement: Notes can keep pictures
Brevier SHALL let readers attach pictures to notes and display those pictures with the note without adding them to the article.

#### Scenario: Add a picture to a note
- **WHEN** a reader adds a picture to a note
- **THEN** Brevier keeps and displays the picture with that note

#### Scenario: Open a note picture
- **WHEN** a reader opens a picture attached to a note
- **THEN** Brevier shows the picture at a larger size

### Requirement: Notes can be exported with their passages and pictures
Brevier SHALL export one page's notes or all notes as Markdown with each note's passage quote, a link back to the passage, note text, and attached pictures.

#### Scenario: Export notes as a ZIP
- **WHEN** a reader exports notes in ZIP format
- **THEN** the archive contains Markdown notes and their picture files

#### Scenario: Export notes as Markdown files
- **WHEN** a reader exports notes in Markdown format
- **THEN** Brevier writes a Markdown file and an adjacent `images/` directory containing its picture files

#### Scenario: Share notes from Android
- **WHEN** a reader shares one page's notes or all notes from Android
- **THEN** Brevier sends the same Markdown notes and attached pictures through the system share flow

### Requirement: Saving an article includes its note pictures
Brevier SHALL include pictures attached to a page's notes when the reader saves that article.

#### Scenario: Save an article with note pictures
- **WHEN** a reader saves an article that has notes with pictures
- **THEN** the saved article bundle contains the note pictures
