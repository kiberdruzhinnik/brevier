# Tasks

## 1. Deterministic ABI selection

- [x] 1.1 Resolve native-build and packaging ABIs from the requested Gradle variants so release defaults to arm64-v8a only, debug defaults to arm64-v8a plus x86_64, and incompatible release overrides fail clearly; verify default release, default debug, and invalid release override invocations. Verified: default release contains only arm64-v8a (15,129,993 bytes), default debug contains arm64-v8a and x86_64 (30,371,844 bytes), and `assembleRelease -Pabis=x86_64` fails with the expected message.
- [x] 1.2 Document the release and emulator ABI behavior in the Android build instructions and verify the documented commands match the Gradle defaults. Verified `CONTRIBUTING.md` uses default `assembleDebug` for both ABIs and `assembleRelease` for arm64-only; both commands built with those contents.

## 2. Release shrinking

- [x] 2.1 Enable Android resource shrinking for release alongside existing R8 code shrinking, preserve named font assets, and verify the packaged resources and fonts remain available. Verified the release build runs `minifyReleaseWithR8` and `optimizeReleaseResources`; APK contains all 7 fonts, font license, and referenced drawables. Shrunk arm64 APK is 15,129,993 bytes, unchanged because no additional resources were removable.
- [x] 2.2 Install and exercise the release APK's reader, settings, and icons after resource shrinking; verify resource access succeeds and compare APK size with the arm64 release baseline. Verified on the fresh Android 12 AVD: signed arm64 APK installed and launched, reader intro/menu icons rendered, Settings opened with controls present, and logcat showed no fatal or missing-resource exceptions. APK remains 15,129,993 bytes.

## 3. Release artifact verification

- [x] 3.1 Update release and reproducibility CI to use the deterministic arm64-only default and fail if the signed APK contains any other native ABI; verify the unsigned reproducibility comparison still passes. Verified the workflow commands use defaults, its APK ABI assertion expects exactly arm64-v8a, and a local signed/unsigned APK entry-hash comparison matched every payload entry after excluding signing metadata. The CI `apksigcopier` command remains configured for the full signing comparison.
- [x] 3.2 Build signed release and debug APKs from the documented commands, verify the release signature and ABI contents, confirm debug includes both ABIs, and record compressed APK sizes. Verified release signature with `apksigner`; release contains arm64-v8a only (15,129,993 bytes), debug contains arm64-v8a and x86_64 (30,371,844 bytes).
