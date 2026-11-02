<p align="center">
  <img src="assets/brevier.svg" alt="" width="122">
</p>

# Brevier

An Android browser for reading. Brevier fetches a page, turns it into clean text on your
phone, and sets it in your typography — not the site's. It runs no JavaScript, applies no
site CSS, and needs no account.

- Articles without pop-ups, ads, and page scripts.
- Public PDFs shared through Cloud Mail or Yandex Disk, read as clean text.
- Documentation from Git repositories, feeds, and discussion threads.
- Notes with pictures, saved passages, and Markdown export.
- Native Android text, not a WebView.

The same Rust core handles fetching, extraction, conversion, and the page model. Pages are
converted to Markdown on your device and rendered as native text. Why it is built this way
is in the [manifesto](MANIFESTO.md); what comes next is in the [roadmap](ROADMAP.md). A
short tour is at [brevier.bizibah.com](https://brevier.bizibah.com/).

## Install

Brevier supports Android 7 and later on 64-bit ARM devices. Install it from
[F-Droid](https://f-droid.org/packages/io.github.gurov.brevier/) to receive updates there,
or get the APK from [GitHub Releases](https://github.com/gurov/brevier/releases/latest).
The APK from either source can update the same installation when signed with the release
key.

## Read

Open a link in Brevier, share a URL with it, or paste an address into the app. It reads
articles, public Cloud Mail and Yandex Disk PDF shares, RSS/Atom/JSON feeds, GitHub and
GitLab documentation, and discussion threads. Pages are fetched only when you ask to read
them; Brevier does not crawl sites or keep credentials.

The reader includes saved passages and notes, pictures attached to notes, history,
bookmarks, reading progress, find on page, feeds, a dark theme, and export of notes as
Markdown. Notes appear beneath the paragraph they annotate. Articles can be saved for
offline reading and shared through Android's system share sheet.

Brevier does not run JavaScript, use a site's CSS, submit forms, or sign into websites.
Pages that cannot be read are explained and can be opened in your browser.

## For maintainers

The Linux CLI remains in the source tree for parser checks on the Linux VPS and internal
CI. It is not distributed as a user application. Build and test instructions are in
[CONTRIBUTING.md](CONTRIBUTING.md); the parser regression corpus is in `corpus/`.

## Contributing

Bug reports and patches are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for the
Android and core build instructions, the [roadmap](ROADMAP.md), and issues labelled
`roadmap`.

## License

MIT OR Apache-2.0, at your option.
