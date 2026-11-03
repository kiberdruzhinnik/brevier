//! PDF bytes and extracted page text are untrusted parser input.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = brevier::pdf::markdown(data, "fuzz.pdf");

    let text = String::from_utf8_lossy(data);
    let pages = text
        .split('\u{c}')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let _ = brevier::pdf::cleanup_extracted_pages(&pages);
});
