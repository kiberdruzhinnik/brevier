# Spec Delta

## ADDED Requirements

### Requirement: Article images render on Android 9
The Android app SHALL render core-provided article image data correctly on Android 9 and earlier supported Android versions.

#### Scenario: Render an article image on Android 9
- **WHEN** an article image is displayed on Android 9
- **THEN** the bitmap is copied without a buffer-position or protocol mismatch failure
