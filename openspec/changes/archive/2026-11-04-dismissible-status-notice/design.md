# Design

## Context

See proposal.md for the user problem. `MainActivity` owns one bottom-gravity `TextView` for ordinary status messages and the tab-close Undo action. The toolbar position and measured height are already available in the activity.

## Goals / Non-Goals

**Goals:** Keep the notice readable above the bottom toolbar, and let a horizontal swipe dismiss it.

**Non-Goals:** Change notice wording, timeout duration, or the operation associated with tapping Undo.

## Decisions

- Keep the notice in the activity root and derive its bottom margin from the selected toolbar edge. For a bottom toolbar, include the toolbar's measured height and the existing gap; for a top toolbar, retain the current bottom-edge placement. Reapply placement when toolbar position or size changes.
- Detect a horizontal swipe on the notice and hide the view when the gesture passes a distance or velocity threshold. Preserve click handling for the Undo action and avoid treating an ordinary tap as a swipe.
- Keep the existing timeout as a fallback. Dismissing the Undo notice hides its affordance; it does not undo the already completed tab close.

## Risks / Trade-offs

- [Swipe handling can intercept a tap or compete with parent gestures] → Require clear horizontal movement before dismissing and preserve click behavior for taps.
- [Toolbar height can change after layout] → Reposition the notice after toolbar layout changes as well as after a toolbar-position setting change.
