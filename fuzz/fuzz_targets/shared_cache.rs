//! Front matter for plain shared Markdown cache records.
#![no_main]

use brevier::{Address, cache};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let address = Address::Web("https://example.com/article#section".to_owned());
    let _ = cache::read_shared(&address, &text);
});
