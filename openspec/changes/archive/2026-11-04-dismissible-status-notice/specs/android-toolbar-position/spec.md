# Spec Delta

## ADDED Requirements

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
