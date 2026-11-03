# Contributing to Brevier

Patches and bug reports are welcome. For product behavior, see the [README](README.md),
[manifesto](MANIFESTO.md). Brevier is an Android app over a
shared Rust core. The Linux CLI is kept in the repository for maintainers' parser checks
on Linux VPS hosts; it is not published or distributed as an app.

Working with a coding agent? Give it [AGENTS.md](AGENTS.md) as well.

## Languages and layout

Everything on GitHub — issues, pull requests, commit messages — is in English. Interface
text is in English and lives in the core (`src/failure.rs`, `src/intro.rs`). Comments in the
code are mostly in Russian; write yours in the language you are most exact in.

- `src/` — the Rust core: fetching, extraction, conversion to Markdown, feeds, repository
  reading, storage, and the page model. `src/android.rs` is its JNI entry point.
- `src/main.rs` — the Linux CLI used by maintainers and parser checks.
- `android/` — the native Android app in Kotlin, using the shared core through JNI.
- `corpus/` — saved regression expectations and scripts for checking parsers and extraction.
- `fuzz/` — fuzz targets for parser and page-model invariants.

## Build and check the core

Rust 1.88 or newer is needed. The Linux CLI is a development tool; build it from the
repository root:

```sh
cargo build --bin brevier
cargo run --bin brevier -- --stdin https://example.com/ < saved-page.html
cargo run --bin brevier -- --check --min 100 https://example.com/
```

For local core checks:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --features images,typeset
corpus/check.sh
```

The CLI's `--stdin` mode is useful for testing a parser against saved HTML without mixing
code changes with changes to a live site. `corpus/check.sh` runs offline against saved
expectations. Live corpus runs use four jobs; more often brings 403 responses.

## Build Android

The app needs JDK 17, Android SDK platform 35, and the NDK version named by
`ndkVersion` in `android/app/build.gradle.kts`. Rust for the shared core is pinned in
`android/rust-toolchain.toml`; the first build installs that toolchain and its Android
Rust targets.

```sh
export JAVA_HOME=… ANDROID_HOME=…
cd android
./gradlew assembleDebug -Pabis=arm64-v8a
```

Gradle builds the Rust core through `android/core.sh`. The APK is written under
`android/app/build/outputs/apk/debug/`. For a release-shaped unsigned APK:

```sh
./gradlew assembleRelease -Pabis=arm64-v8a -Punsigned
```

GitHub Releases is the only public APK channel. Release builds sign the Android APK; CI
also checks that an unsigned build has the same contents, preserving release
reproducibility. Keep the pinned Rust, NDK, and JDK versions aligned across local and CI
builds.

## Before a pull request

Every commit must be signed off by its author under the Developer Certificate of Origin.
Run `cargo fmt --check`, core Clippy and tests above. CI builds the Android core and app.

For extraction or conversion changes, run the old and new binaries over the same saved HTML
and inspect every output difference. For extraction, conversion, or page-model changes,
run `corpus/frozen.py snapshot`, approve the base output, then run it on the change. For a
parser change, run its fuzz target for a few minutes; for `--check`, run
`corpus/check.sh`.

## Fuzzing

Fuzz targets need nightly Rust and `cargo-fuzz`:

```sh
cargo +nightly fuzz run html -- -max_total_time=600 -max_len=65536 -timeout=10
```

Seed `fuzz/corpus/<target>/` with saved inputs. Corpus seeds and findings under
`fuzz/artifacts/` stay out of git. Turn each finding into a regression test next to the
code it exposed.

## Android releases

Releases are built from version tags by `.github/workflows/release.yml`. Before tagging,
update the version in `Cargo.toml` and `android/app/build.gradle.kts`, add release notes in
`packaging/notes/vX.Y.Z.md`. The release workflow builds and publishes the signed APK.
Do not add desktop packages or public CLI distribution; the Linux CLI is for internal
parser validation.

## License

Contributions are licensed under MIT OR Apache-2.0, at your option. The bundled fonts are
under the SIL Open Font License; `assets/fonts/OFL-NotoSans.txt` must travel with any
copy you distribute.
