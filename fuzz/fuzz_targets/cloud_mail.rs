//! Public Cloud Mail URLs and API JSON are untrusted parser input.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let _ = brevier::hosts::cloud_mail::public_weblink(&text);
    let _ = brevier::hosts::cloud_mail::file_api(&text);
    let _ = brevier::hosts::cloud_mail::folder_api(&text);
    let _ = brevier::hosts::cloud_mail::resource(
        &text,
        "https://cloud.mail.ru/public/fuzz/share",
    );
    let _ = brevier::hosts::cloud_mail::pdf_name(&text);
    let _ = brevier::hosts::cloud_mail::atom(
        &text,
        "https://cloud.mail.ru/public/fuzz/share",
    );
    let _ = brevier::hosts::cloud_mail::download_url(
        "https://cloud.mail.ru/public/fuzz/share/file.pdf",
        &text,
    );
});
