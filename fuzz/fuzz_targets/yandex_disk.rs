//! Public Yandex Disk URLs and API JSON are untrusted parser input.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let _ = brevier::hosts::yandex_disk::share(&text);
    let _ = brevier::hosts::yandex_disk::metadata_api(&text);
    let _ = brevier::hosts::yandex_disk::download_api(&text);
    let _ = brevier::hosts::yandex_disk::pdf_name(&text);
    let _ = brevier::hosts::yandex_disk::file_mime(&text);
    let _ = brevier::hosts::yandex_disk::download_url(&text);
    let _ = brevier::hosts::yandex_disk::atom(&text, "https://disk.yandex.ru/d/fuzz");
});
