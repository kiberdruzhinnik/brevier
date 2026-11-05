# AGENTS.md

For coding agents working on Brevier, and for the people who run them. It does not
repeat [CONTRIBUTING.md](CONTRIBUTING.md) — building, the corpus, packaging and releases
are there; read it too. This file is what an agent needs on top: what must not be built,
what is already decided, and what counts as having checked a change.

Brevier is built with agents in the workshop, and by default the product reads pages with
no model at all. A pull
request is welcome however it was written, as long as a person stands behind it: they
read the change, checked it the way this file asks, and say how.

## What Brevier is

A reading browser for Android without JavaScript. It fetches a page, reduces it to Markdown
on the reader's machine and sets it in the reader's type, not the site's — plus a mode for
reading the documentation of git repositories. A core library in Rust and an Android app
(`android/`, Kotlin over the same core). The Linux CLI remains in the repository for
maintainers' parser checks; it is not a public product.

What it promises is in [MANIFESTO.md](MANIFESTO.md). Where this file and the manifesto
disagree, the manifesto wins.

## Never

These are not open questions, and a patch that does any of them will not be merged:

- **Run JavaScript** — no engine, in any form, not "only for this site".
- **Apply a site's CSS.** The reader sets the type.
- **Put a model in the default reading path.** Extraction and conversion are deterministic
  rules in Rust, checked byte for byte against `corpus/expected/`. A model, if one ever
  helps — a small specialised one for pages the rules can't read, say — is a separate
  module that the reader turns on, with what it produces marked as its own; whether and
  how is the maintainer's design decision, not a patch to start with. (Models in the
  workshop — to mark the corpus, to find where rules cut living text — are fine.)
- **Hold a credential**: no cookies kept, no logins, no forms. Authenticated reading is the
  companion extension's job.
- **Work around TLS.** Trust is the operating system's (`rustls-platform-verifier`);
  `danger_accept_invalid_certs` and bundled roots are out. `cargo tree | grep webpki-roots`
  stays empty.
- **Crawl.** Requests are measured by reason: what reading a page takes, never walking a
  site or fetching pages nobody asked to read.
- **Rule extraction by site.** The open web is ruled by the shape of a page. Named
  per-host rules exist only in the table in `src/hosts.rs`, each row with a test or
  corpus pages.

## Decided — do not undo

Much of what looks superfluous was chosen, with a cost. Every module opens with a comment
saying why it is the way it is: read it before simplifying. A few that agents tend to
"fix":

- **Markdown is the internal representation**, not a file format: one renderer for the web
  and for `.md` files. What Markdown cannot hold is lost on purpose.
- **The core links no toolkit.** Android stays in `src/android.rs` and `android/`.
- **Android draws native text**, never a WebView, from the page model in `src/page.rs`.
- **Interface text is English and lives in the core** (`src/failure.rs`, `src/intro.rs`),
  so every front-end says the same thing.
- **Dependencies come without default features** when those bring weight; no `serde_json`
  in the core (`src/json.rs` exists for that).
- **The wording of MANIFESTO.md is the maintainer's.** Correct facts there; do not rewrite,
  reorder or soften.

## What counts as checked

Say in the pull request which of these you did and what you saw:

- **Any change:** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test` (the page model's tests need
  `--features images,typeset`).
- **Extraction or conversion:** run the old and the new binary over the **same saved
  HTML** — download the corpus pages once, build the old binary from the base commit in a
  separate worktree and target directory, run both through `brevier --stdin <url> <
  page.html`, diff the outputs. A live run mixes your change with whatever the sites
  changed since. Every difference must be one you meant; update `corpus/expected/` only
  for those.
- **Extraction, conversion or the page model** (typesetting, the shelf, reading progress,
  find): `corpus/frozen.py snapshot` once, then `corpus/frozen.py --approve` on the
  base commit and `corpus/frozen.py` on yours. It lays every saved page out as the Android
  reader does and stops on a panic, a hang, a model that does not add up, or a
  difference from the approved output.
- **A parser** (`json.rs`, `feed.rs`, `address.rs`, `page.rs`, `typeset.rs`, the
  extraction path): run its target in `fuzz/` for a few minutes (see CONTRIBUTING, Fuzzing).
- **`--check`:** `corpus/check.sh`, offline.
- **The app:** tests do not cover it. Run it on a phone and say what you looked at. A
  screenshot helps; a description of a pixel measurement is better than "looks fine".
- **A new host rule:** a test against the host's real markup — a saved page in
  `tests/fixtures/`, or the parts the rule reads, condensed in the test.

## Android on the maintainer machine

Use the Android toolchain already installed for this repository; do not accept the
system-default Java version without checking it. The maintainer machine has JDK 17 at
`/usr/lib/jvm/java-17-openjdk-amd64` and the Android SDK at `~/Android/Sdk` (capital
`Sdk`). Before running Gradle, set the environment explicitly:

```sh
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export PATH="$JAVA_HOME/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
```

Confirm `java -version` reports 17. The app's `ndkVersion` in
`android/app/build.gradle.kts` is authoritative; use that NDK under the SDK if setting
`ANDROID_NDK_HOME` manually. Do not replace these paths with guesses such as
`~/Android/sdk`.

For Android app changes, attempt to run the app on the configured Android emulator and
inspect the changed behavior. Discover available AVDs with
`"$ANDROID_HOME/emulator/emulator" -list-avds`; use the existing Brevier AVD when
available. Do not refuse emulator testing without checking the emulator binary, AVD,
ADB, and KVM access first. If this execution environment does not expose `/dev/kvm` or
hardware acceleration, report that concrete limitation and its observed effect, then
complete any feasible build or emulator checks. Do not claim the app was tested on an
emulator unless it was actually launched and inspected.

After each code or Android change, build a fresh signed release APK with
`cd android && ./gradlew assembleRelease` using the environment above, then provide the
APK path to the maintainer. The signing properties file is at
`~/Android/keys/brevier-signing.properties`; set `BREVIER_SIGNING` to that path for the
Gradle invocation. The resulting APK is under `android/app/build/outputs/apk/release/`
and its name includes the app version and `arm64`. If the signed build fails, report the
actual blocker and provide the release build result; do not silently substitute an
unsigned APK. Treat all signing material as secret: never read out, print, log, copy into
the repository, or include in chat the properties contents, passwords, private key,
keystore contents, or other credentials. Refer only to the properties file path when
describing signing setup.

## Conventions

- **English on GitHub**: commits, pull requests, issues. A commit subject is a plain
  sentence about the change ("Read royallib books part by part (#20)"); the body says why,
  and what it cost.
- **Comments in the code** are mostly Russian; write yours in the language you are most
  exact in. Match the surrounding code: its comment density, naming and idiom. A comment
  says why, not what.
- **One concern per pull request**, small enough to read in one sitting.
- **Every commit of a pull request is signed off** (`git commit -s`) under the
  [Developer Certificate of Origin](DCO) — by the person submitting it. An agent does not
  sign off on anyone's behalf: the sign-off is that person's statement.

## Pitfalls

- **Some corpus sites change between runs** — a rotating footer, a one-time token, a
  relative date, a block served every other time. Run again before looking for the cause
  in the code.
- **Corpus runs use four jobs.** Eight bring 403s, and a readable page slides into a
  refusal.
- **`tests/wire.rs` fails wholesale after the checkout moves**: the binary's path is baked
  in at compile time. `touch tests/wire.rs`.
- **On Android, `optString` returns JSON `null` as the string "null"** — read nullable
  fields with `text()` (`Core.kt`).

## Claude Code

The maintainer keeps a private `CLAUDE.md`, which is gitignored. To give your Claude this
file, create your own `CLAUDE.md` in the checkout with the line `@AGENTS.md`.
