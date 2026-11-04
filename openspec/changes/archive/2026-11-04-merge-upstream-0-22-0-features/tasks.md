# Tasks

## 1. Core reliability and extraction

- [x] 1.1 Port bounded decoding for large images and add regression coverage; verify with the media tests and a large-image fixture.
- [ ] 1.2 Add interrupted-render recovery without losing unrelated browsing state; verify recovery behavior after a simulated failed open/layout/render.
- [x] 1.3 Correct Android bitmap transfer for API 28 and older supported versions; verify the core/Android image protocol and a rendered image on API 28.
- [ ] 1.4 Port the regional homepage extraction correction with a saved real-markup fixture; run the extraction regression and corpus frozen comparison.
- [ ] 1.5 Expose supported User-Agent choices in Android settings and persist the selection; verify the setting survives restart and subsequent requests use it.

## 2. Android reading experience

- [ ] 2.1 Add paper tones, paragraph presentation, and zoom levels through 300 percent; verify persisted settings and layout at the minimum and maximum zoom.
- [ ] 2.2 Add off-by-default immersive reading and verify system bars restore on exit and navigation.
- [ ] 2.3 Add tab swipe-close with undo and close link-opened tabs on Back; verify restoration, originating-tab return, and existing Back order.
- [ ] 2.4 Add independently configurable tap, horizontal swipe, and volume page turning plus left-handed directional behavior; verify each action and disabled defaults.
- [ ] 2.5 Improve address/search keyboard layout and search-provider identification; verify suggestions do not obscure the address field.

## 3. Saved article pictures

- [ ] 3.1 Persist only already-downloaded article pictures with saved copies, enforce 32 MB per article, and exclude them from Android backup; verify offline reopening, limit enforcement, and no extra requests.

## 4. Release alignment and integration

- [x] 4.1 Set Cargo and lockfile versions to 0.22.0 and Android version code to 2200; verify Cargo metadata and generated APK version name/code.
- [x] 4.2 Run required Rust formatting, clippy, and tests; run the relevant parser fuzzer and frozen corpus checks, and record the results.
- [ ] 4.3 Build the release APK and install/open it on Android 12 or later; check a representative article, image rendering, settings persistence, and navigation.
- [ ] 4.4 Review the final change for preserved cloud-share, shared Markdown cache, toolbar-position, note-removal, Android minimum SDK, and Android-only distribution behavior.
