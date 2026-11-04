# core-reading-reliability Specification

## Purpose

Keeps deterministic page reading reliable across supported devices by bounding image work, recovering from failed renders, preserving bitmap compatibility, and choosing the intended regional homepage and request identity.

## Requirements

### Requirement: Large images use bounded decoding memory
The reader SHALL reduce the memory required to decode large images before producing the final article image.

#### Scenario: Decode a large image
- **WHEN** an article includes an image larger than four megapixels
- **THEN** image decoding reduces its working size before final resampling and the reader remains responsive

### Requirement: Failed page rendering does not repeat on launch
The Android app SHALL remember when page opening or rendering was interrupted by a crash and SHALL avoid automatically reopening that page on the next launch.

#### Scenario: Recover after a rendering crash
- **WHEN** the app restarts after crashing while opening, laying out, or drawing a page
- **THEN** it opens without automatically repeating the failed page and lets the reader navigate or open a page explicitly

### Requirement: Regional homepages use the correct country domain
Homepage extraction SHALL recognize a regional homepage from its complete domain rather than treating a short site-name prefix as a country-level domain.

#### Scenario: Open a regional homepage
- **WHEN** a reader opens a regional homepage whose host includes a country-specific second-level domain
- **THEN** the reader extracts the requested regional homepage rather than redirecting to or extracting another region's homepage

### Requirement: Android request identity is configurable
The Android app SHALL let the reader save a visible-ASCII User-Agent string and enable it as an override, off by default, for subsequent page requests.

#### Scenario: Enable a custom User-Agent
- **WHEN** the reader enables the custom User-Agent setting with a valid string
- **THEN** subsequent page requests use that string and the preference persists after restart

#### Scenario: Keep the default User-Agent
- **WHEN** the reader has not enabled a valid custom string
- **THEN** requests continue to identify as Brevier
