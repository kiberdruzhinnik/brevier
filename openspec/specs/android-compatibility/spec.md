# Android Compatibility Specification

## Purpose

Keeps Brevier usable on supported Android phones while it adopts the current Android target API and its system navigation behavior.

## Requirements

### Requirement: The Android 16 target preserves Android 12 support
Brevier SHALL target Android API 36 while remaining installable on Android API 24 and later, including Android 12.

#### Scenario: Install on Android 12
- **WHEN** a reader installs Brevier on a device running Android 12
- **THEN** the app installs and opens successfully

#### Scenario: Run with the Android 16 target
- **WHEN** Brevier runs on a device running Android 16
- **THEN** system Back input continues to reach Brevier

### Requirement: System Back preserves Brevier's navigation order
Brevier SHALL handle system Back by closing the topmost open interface or selection first, then stepping back through the current tab's pages, and finally leaving the app or returning to the app that opened it.

#### Scenario: Close an open interface before navigating
- **WHEN** a reader presses Back while a menu, shelf, tab list, settings, find bar, address bar, or text selection is open
- **THEN** Brevier closes or clears the topmost item before changing the tab's page

#### Scenario: Go back through page history
- **WHEN** no interface or selection is open and the current tab has a previous page
- **THEN** Back opens the previous page in that tab

#### Scenario: Leave from the first page
- **WHEN** no interface or selection is open and the current tab has no previous page
- **THEN** Brevier moves to the background or returns to the app that opened the link

### Requirement: Article images render on Android 9
The Android app SHALL render core-provided article image data correctly on Android 9 and earlier supported Android versions.

#### Scenario: Render an article image on Android 9
- **WHEN** an article image is displayed on Android 9
- **THEN** the bitmap is copied without a buffer-position or protocol mismatch failure
