# android-toolbar-position Specification

## Purpose

Lets readers choose where Android's full navigation toolbar appears and keeps the reading progress indicator arranged at the opposite screen edge.

## Requirements

### Requirement: Reader can choose the toolbar position
The Android app SHALL let the reader place the full toolbar at the top or bottom of the screen, with the top position as the default.

#### Scenario: New installation uses the top position
- **WHEN** the app starts without a saved toolbar position
- **THEN** the full toolbar appears at the top of the screen

#### Scenario: Reader selects bottom position
- **WHEN** the reader selects the bottom toolbar position in settings
- **THEN** the full toolbar appears at the bottom of the screen

#### Scenario: Position persists after restart
- **WHEN** the reader changes the toolbar position and restarts the app
- **THEN** the selected position remains active

### Requirement: Toolbar position preserves usable reading area
The Android app SHALL keep article content clear of the toolbar and system insets at either toolbar position.

#### Scenario: Toolbar is at the top
- **WHEN** the toolbar is positioned at the top
- **THEN** article content does not render underneath the toolbar

#### Scenario: Toolbar is at the bottom
- **WHEN** the toolbar is positioned at the bottom
- **THEN** article content does not render underneath the toolbar

### Requirement: Progress strip occupies the opposite edge
When the reading progress strip is visible, the app SHALL display it at the screen edge opposite the toolbar and preserve its existing rule for showing only when article content exceeds the available screen height.

#### Scenario: Top toolbar with visible progress
- **WHEN** the toolbar is at the top and the article is taller than the available screen area
- **THEN** the progress strip appears at the bottom edge

#### Scenario: Bottom toolbar with visible progress
- **WHEN** the toolbar is at the bottom and the article is taller than the available screen area
- **THEN** the progress strip appears at the top edge

#### Scenario: Article fits the available screen
- **WHEN** the article content fits within the available screen area
- **THEN** the progress strip remains hidden at either toolbar position

### Requirement: Status notices stay clear of the toolbar
The Android app SHALL place status notices so they do not overlap the full navigation toolbar at either toolbar position.

#### Scenario: Bottom toolbar is visible
- **WHEN** a status notice appears while the full toolbar is positioned at the bottom
- **THEN** the notice appears above the toolbar

#### Scenario: Top toolbar is visible
- **WHEN** a status notice appears while the full toolbar is positioned at the top
- **THEN** the notice appears at the bottom edge, clear of the toolbar

### Requirement: Reader can dismiss a status notice with a swipe
The Android app SHALL allow the reader to dismiss a visible status notice by swiping it away.

#### Scenario: Swipe away a notice
- **WHEN** the reader swipes a visible status notice horizontally
- **THEN** the notice is hidden immediately

### Requirement: Auto-hidden toolbar moves away from reading content
When toolbar auto-hide is enabled, the toolbar SHALL slide off the screen edge where it is positioned and return when the reader invokes the toolbar.

#### Scenario: Top toolbar auto-hides
- **WHEN** the top-positioned toolbar is hidden during reading
- **THEN** it slides upward and can be shown again through the existing toolbar interaction

#### Scenario: Bottom toolbar auto-hides
- **WHEN** the bottom-positioned toolbar is hidden during reading
- **THEN** it slides downward and can be shown again through the existing toolbar interaction
