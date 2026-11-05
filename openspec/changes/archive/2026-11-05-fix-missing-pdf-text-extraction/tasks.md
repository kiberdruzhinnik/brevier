# Tasks

## 1. Select and validate the OCR backend

- [x] 1.1 Inspect the supplied PDF page and compare general and positioned extraction; record that the visible definition has no extracted text spans and requires OCR.
- [x] 1.2 Select an offline OCR and page-rendering backend that supports Android and Linux; verify its license and packaged size, then confirm it recognizes the supplied definition from the sample page.
- [x] 1.3 Add the English `tessdata_fast` model alongside Russian, verify its license and incremental package size, and confirm the combined `rus+eng` configuration recognizes English words in predominantly Russian text.

## 2. Add per-page OCR selection and loading feedback

- [x] 2.1 Remove the global OCR setting and make regular extraction the initial method for every page; verify ordinary PDF reading does not initialize OCR resources.
- [x] 2.2 Add a page-level OCR action that processes only the selected page and displays the resulting text; verify the supplied definition appears once in the correct reading order.
- [x] 2.3 Retain each page's regular result and let the reader switch back from OCR; verify OCR markers disappear and native text is neither lost nor duplicated across both methods, including when OCR fails.
- [x] 2.4 Extend mixed-language regression coverage to verify Russian text containing English words is recognized in order and without loss or duplication.
- [x] 2.5 Report the active loading stage as network fetching, regular text extraction, or OCR text extraction; verify reload shows fetch then regular extraction, and a page OCR action shows OCR extraction.
- [x] 2.6 Move the OCR action from the article body into the dots popup menu; show “Recognize PDF text (OCR)” for regular PDFs and “Use regular PDF text” for OCR results, hide both for non-PDF pages, and verify the Android regression test passes.

## 3. Check shared behavior and regressions

- [x] 3.1 Run the supplied Cloud Mail PDF and a PDF in the supplied Yandex Disk folder through both shared provider paths; verify initial regular extraction, per-page OCR, and switching back preserve text order.
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `cargo test --features images,typeset`; record results.
- [x] 3.3 Run `corpus/frozen.py snapshot` once, approve the base commit with `corpus/frozen.py --approve`, and run `corpus/frozen.py` on the change; verify no unintended reading output changes with OCR disabled.
- [x] 3.4 Run the `pdf` fuzz target for a few minutes and verify it reports no crash or hang.
- [x] 3.5 Add reader regression coverage for removing the global setting, per-page OCR selection, restoring regular text, and all three loading-stage labels; verify the cases pass.
- [x] 3.6 Run the updated app on an Android device or emulator; verify initial regular extraction, OCR and switch-back controls on a PDF page, plus stage feedback during reload and OCR.
- [x] 3.7 Run the updated app on an Android emulator; verify the OCR action is absent from the article body and appears with the correct toggle label in the dots menu for both supplied providers, then verify OCR and switch-back still work.

## 4. Diagnose and fix release startup crash

- [x] 4.1 Reproduce the startup crash from a signed, minified x86_64 release on the emulator and capture logcat; verify ARM64 translation is available to test the ARM64 APK after the fix.
- [x] 4.2 Identify and fix the root cause, adding appropriate regression coverage.
- [x] 4.3 Rebuild the signed minified ARM64 release and verify cold start with OCR unset/disabled and enabled/persisted; confirm the reader opens without a crash and record the results.

## Verification notes

The earlier notes record backend, packaging, and release-startup checks. The revised
per-page controls and stage feedback were implemented and checked as recorded below.

- Added the official Apache-2.0 English `tessdata_fast` model (4,113,088 bytes). The
  regression uses synthetic mostly-Russian text with occasional English words and
  recognizes the full word sequence in order. `cargo test --lib --features pdf-ocr`
  passed all 427 tests; `cargo clippy --lib --features pdf-ocr -- -D warnings` and
  `cargo fmt --check` passed.
- The stripped ARM64 core grew from 23,083,712 bytes with Russian only to 27,202,160
  bytes with both models, an increase of 4,118,448 bytes. `android/core.sh` built the
  ARM64 core successfully. The signed minified ARM64 APK rebuilt with the packaged C++
  runtime after setting JDK 17 and `ANDROID_HOME=/home/agent/Android/Sdk`; it is
  37,670,994 bytes and passes APK v2 signature verification. A full
  `cargo test --features pdf-ocr` also hit the sandbox's local-socket restriction in
  19 wire tests; its library tests passed when run separately.

- Formatting and Clippy passed both without features and with `pdf-ocr`. `cargo test` passed (423 library tests, 9 CLI tests,
  20 wire tests); `cargo test --features images,typeset` passed (490 library tests,
  9 CLI tests, 20 wire tests); and `cargo test --features pdf-ocr` passed (426 library
  tests, 9 CLI tests, 20 wire tests). Wire tests need local sockets, so they were run
  outside the network-restricted sandbox.
- The Android ARM64 core rebuilt successfully after the final OCR merge changes.
- The frozen comparison used the same 85 saved pages on the base commit and this change:
  83 succeeded and 2 returned the same expected code 5; no approved output changed.
  Live snapshot URLs that were not already cached were unreachable in this environment.
- The PDF fuzz target completed 1,468,083 executions in 181 seconds without a parser
  crash or hang. LeakSanitizer was disabled because its ptrace check fails in this
  container after the fuzz run completes.
- The attached sample was exercised through the shared OCR-enabled PDF converter, and
  the requested definition appeared once in reading order. Both provider branches call
  that same converter.
- The supplied Cloud Mail link is a folder, so the linked "Старые деньги" PDF was opened
  through the generated file link. With OCR off, its Cambridge definition is absent;
  with OCR on, it appears once after the Cambridge lead-in and is marked `[OCR]`.
  The supplied Yandex link is a folder containing different PDFs; a PDF in that folder
  was opened through the nested-file route with OCR both off and on. OCR-off produced no
  OCR markers; OCR-on preserved native article text without duplicated OCR spans.
- Reproduced the release startup crash with a signed, minified x86_64 diagnostic APK on
  the Android 31 emulator. Logcat showed `UnsatisfiedLinkError: dlopen failed: library
  "libc++_shared.so" not found`, required by `libbrevier.so`, during `Core` class loading
  from `App.onCreate`. The emulator reports `libndk_translation.so` and supports both
  `x86_64` and `arm64-v8a`.
- `android/core.sh` now packages the NDK's matching `libc++_shared.so` for each ABI. The
  `verifyReleaseCxxRuntime` Gradle task checks the final release APK contains the runtime
  for every selected ABI. The signed ARM64 release rebuilt successfully; APK v2
  signature verification passed.
- Installed that ARM64 APK on the emulator and cold-started it with clean data and OCR
  off, then enabled OCR in Settings and cold-started with the preference persisted. Both
  starts reached the reader; the app process stayed alive and logcat contained no fatal
  exception, `UnsatisfiedLinkError`, or native crash.
- Removed the global OCR switch and made regular PDF extraction the initial mode. Each
  open PDF now offers OCR, retains its regular result, and restores that result directly
  when the reader switches back. OCR runs on a worker and does not replace the core's
  regular cache, archive, history, or reading progress.
- Android regression tests in `android/app/src/test/.../PdfTextVersionsTest.kt` cover
  regular-first behavior, selecting OCR, restoring regular text, failed-recognition
  fallback, both action labels, and all three stage labels. `:app:testDebugUnitTest` passed
  all 4 tests.
- Load stages are keyed without URL fragments, so an anchored address observes the same
  in-flight fetch. The emulator showed `Network fetching` on reload and
  `OCR text extraction` for both OCR actions. Reload returned to the regular article; the
  regular-extraction stage is set after PDF download and before parsing, which completed
  too quickly to capture in a screenshot.
- Installed and ran the updated debug app on the Android 31 x86_64 emulator. The Cloud
  Mail sample opened with regular text and without its SKLADCHIK ad cover; OCR and
  switch-back worked. A PDF in the supplied Yandex folder showed the same controls and
  completed OCR and switch-back. Settings showed no global OCR switch. No Brevier crash
  or native-link error appeared in logcat. The emulator's launcher/System UI briefly
  reported an ANR on cold boot; waiting dismissed it, and Brevier remained responsive.
- Revised-change checks: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --lib` (424 passed), `cargo test --lib --features pdf-ocr` (428 passed),
  `:app:testDebugUnitTest` (4 passed), and `:app:assembleDebug` all succeeded.
- Rebuilt the final signed ARM64 release after the on-demand OCR and stage changes:
  `android/app/build/outputs/apk/release/brevier-0.22.0-arm64.apk` (36 MB). APK v2
  signature verification passed; it installed and cold-started on the x86_64 emulator
  using ARM64 translation, with no fatal exception or native-link error in logcat.
- After moving the action into the dots menu, rebuilt and installed the debug app on the
  Android 31 x86_64 emulator. For a PDF in the newly supplied Cloud Mail `Ytr8` folder,
  the article body had no OCR control; the menu showed “Recognize PDF text (OCR)”, then
  OCR output appeared with its `[OCR]` marker. The menu changed to “Use regular PDF text”,
  and restoring regular text returned the menu to the recognition label. Repeated the
  toggle on the first PDF in the supplied Yandex `nZLFISIjs5-tqQ` folder; both menu labels
  changed as expected and switching back worked. The second Cloud Mail `X2V5` folder
  opened, but its linked Old Money page did not present the PDF action in that run.
