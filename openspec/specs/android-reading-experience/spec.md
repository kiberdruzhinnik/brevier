# android-reading-experience Specification

## Purpose

Defines Android reading controls and navigation gestures that let readers adjust the page presentation and move through articles without changing the Android-only product model.

## Requirements

### Requirement: Reader can choose paper appearance
The Android app SHALL offer light and dark paper tones, including neutral and warm choices, and SHALL use the current appearance when no preference is saved.

#### Scenario: Select a paper tone
- **WHEN** the reader chooses a paper tone in reading settings
- **THEN** the article background and text colors update and the choice persists after restart

### Requirement: Reader can adjust paragraph spacing
The Android app SHALL let the reader choose paragraph spacing, including compact, separated, and indented paragraph presentation.

#### Scenario: Change paragraph presentation
- **WHEN** the reader selects a paragraph presentation
- **THEN** article paragraphs use that presentation and retain readable separation

### Requirement: Text zoom supports larger reading sizes
The Android app SHALL provide text zoom settings through 300 percent.

#### Scenario: Increase zoom to 300 percent
- **WHEN** the reader selects 300 percent zoom
- **THEN** article text is rendered at that scale without clipping the reading controls

### Requirement: Reader can enable immersive reading
The Android app SHALL provide an off-by-default setting that hides system bars while reading and restores them when the reader leaves immersive reading.

#### Scenario: Enable immersive reading
- **WHEN** immersive reading is enabled and an article is open
- **THEN** system bars are hidden while reading and can be revealed through the system gesture

#### Scenario: Immersive reading defaults off
- **WHEN** no immersive reading preference has been saved
- **THEN** system bars remain visible

### Requirement: Tabs can be closed by swiping with undo
The Android tab list SHALL allow a reader to close a tab by swiping it away and offer a time-limited undo action.

#### Scenario: Undo a tab close
- **WHEN** the reader swipes a tab away and selects Undo before the action expires
- **THEN** the closed tab is restored

### Requirement: Link-opened tabs close on Back
The Android app SHALL close a tab opened from a link when Back is pressed before navigating away from the originating tab.

#### Scenario: Return from a link-opened tab
- **WHEN** the reader follows a link into a new tab and presses Back from that tab's initial page
- **THEN** the link-opened tab closes and the originating tab becomes active

### Requirement: Reader can configure page turning
The Android app SHALL offer independently configurable tap, horizontal swipe, and volume-button page-turn actions, all disabled by default, with a left-handed option for directional actions.

#### Scenario: Page turning defaults off
- **WHEN** no page-turn preference has been saved
- **THEN** tap, horizontal swipe, and volume-button page turning are disabled

#### Scenario: Turn a page by a configured action
- **WHEN** the reader enables a page-turn action and performs it while reading
- **THEN** the visible reading position advances or retreats by nearly one screen while keeping one line visible at the boundary

### Requirement: Search suggestions remain usable with the keyboard
The Android address and search interface SHALL identify when submitted text will be searched and keep suggestions visible without obscuring the address field while the keyboard is open.

#### Scenario: Search text with keyboard open
- **WHEN** the reader enters non-URL text while the keyboard is open
- **THEN** the interface identifies the search provider and positions suggestions so the address field remains visible
