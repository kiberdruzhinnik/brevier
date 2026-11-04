# Tasks

## 1. Persist and expose toolbar position

- [x] 1.1 Add a top/bottom position to the core settings format with top as the fallback for missing or unknown values; verify loading and saving both values and old settings with focused store tests.
- [x] 1.2 Expose the position through the Android core settings bridge and verify Kotlin receives top by default and the persisted selection after reload.

## 2. Add the position setting

- [x] 2.1 Add a top/bottom control to Android Reading settings and persist changes immediately; verify each selection updates the toolbar and survives app restart.

## 3. Lay out toolbar and article at either edge

- [x] 3.1 Place the entire toolbar at the selected edge and the progress strip at the opposite edge, respecting system bar and keyboard insets; verify top and bottom placements in portrait and landscape.
- [x] 3.2 Update article reserved and covered space, scrolling, and visible reading offsets for both edges; verify first/last content remains reachable and progress reflects the visible article position.
- [x] 3.3 Make auto-hide move the toolbar outward from its selected edge and preserve reveal behavior; verify top slides upward and bottom slides downward during reading.
- [x] 3.4 Adapt toolbar-anchored menus, suggestions, contents, and other popups to open within the available screen area for both positions; verify none are clipped or obscured.

## 4. Validate integration

- [x] 4.1 Run the repository-required Rust formatting, lint, and test checks, plus frozen corpus validation for page-model changes; record the commands and results.
- [x] 4.2 Install the Android build on a device and verify settings persistence, reading area, progress strip, auto-hide, keyboard interaction, and anchored panels with each toolbar position; record device/version and observations.
