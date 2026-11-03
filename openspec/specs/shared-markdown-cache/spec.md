# shared-markdown-cache Specification

## Purpose

Lets readers keep the current Markdown copy of each read article in an Android folder they choose and that other apps can access. When enabled, Brevier uses that folder as the source for cached article opens.

## Requirements

### Requirement: Shared Markdown caching is opt-in
Brevier SHALL leave shared-folder caching disabled by default and let readers enable it in Android settings by choosing a shared folder.

#### Scenario: First launch
- **WHEN** a reader opens settings before choosing a shared folder
- **THEN** shared-folder caching is disabled and Brevier continues using its private cache

#### Scenario: Choose a folder
- **WHEN** a reader enables shared-folder caching and selects a folder
- **THEN** Brevier remembers access to that folder across app restarts and shows that shared-folder caching is enabled

#### Scenario: Cancel folder selection
- **WHEN** a reader starts folder selection and cancels it
- **THEN** shared-folder caching remains disabled or retains its previously configured folder and state

### Requirement: Shared folder is the source of cached articles when enabled
When shared-folder caching is enabled, Brevier SHALL read and write cached article Markdown in the selected folder instead of using its private weekly page and image cache for those articles.

#### Scenario: Open a cached article
- **WHEN** a reader opens an article with a valid current copy in the selected folder
- **THEN** Brevier renders that Markdown copy without fetching or extracting the article again

#### Scenario: Open an uncached article
- **WHEN** a reader opens an article with no usable copy in the selected folder
- **THEN** Brevier fetches and extracts the article, then saves its Markdown copy in the selected folder

#### Scenario: Refresh a cached article
- **WHEN** a reader refreshes an article that has a copy in the selected folder
- **THEN** Brevier fetches and extracts the article and replaces its existing Markdown copy with the newer copy

#### Scenario: Shared folder access is unavailable
- **WHEN** the selected folder has been moved, its permission has been revoked, or it cannot be read or written
- **THEN** Brevier reports or handles the storage failure without crashing and continues the open using the network when possible

### Requirement: Shared cache stores Markdown only
Brevier SHALL store each cached article as a plain `.md` file and SHALL NOT write downloaded images or other assets into the selected folder.

#### Scenario: Cache an article containing images
- **WHEN** Brevier saves an article whose Markdown references images
- **THEN** the shared folder contains only the article's Markdown file and no downloaded image or asset files

### Requirement: Shared cache keeps one current copy per article
Brevier SHALL identify shared cached copies by article URL, ignoring URL fragments, and replace the existing copy for that article when it is refreshed.

#### Scenario: Open an article at a fragment
- **WHEN** a reader opens the same article at different URL fragments
- **THEN** Brevier uses the same cached Markdown copy for those opens

#### Scenario: Two articles have the same title
- **WHEN** Brevier caches different article URLs that have the same title
- **THEN** their Markdown copies remain distinct and neither overwrites the other
