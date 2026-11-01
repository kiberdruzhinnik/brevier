//! Public Cloud Mail folders and files are a JavaScript app shell. The public
//! API supplies folder entries and a temporary download URL for file contents.

use url::Url;

use crate::json::{self, Value};

/// The decoded share path in a public URL.
pub fn public_weblink(url: &str) -> Option<String> {
    let parsed = Url::parse(url).ok()?;
    if parsed.scheme() != "https" || parsed.host_str()? != "cloud.mail.ru" {
        return None;
    }
    let mut parts = parsed.path_segments()?;
    if parts.next()? != "public" {
        return None;
    }
    let parts: Vec<String> = parts.map(crate::address::percent_decoded).collect();
    (parts.len() >= 2).then(|| parts.join("/"))
}

/// API URL for a public Cloud Mail share, including a path into a folder.
/// Other Cloud Mail pages keep the ordinary HTML reading path.
pub fn folder_api(url: &str) -> Option<String> {
    let share = public_weblink(url)?;
    let mut api = Url::parse("https://cloud.mail.ru/api/v2/folder").ok()?;
    api.query_pairs_mut().append_pair("weblink", &share);
    Some(api.into())
}

/// Metadata request for a public file. The API response, not the share URL,
/// decides whether it is a PDF.
pub fn file_api(url: &str) -> Option<String> {
    let share = public_weblink(url)?;
    let mut api = Url::parse("https://cloud.mail.ru/api/v2/file").ok()?;
    api.query_pairs_mut().append_pair("weblink", &share);
    Some(api.into())
}

/// Readable outcome from Cloud Mail public metadata.
#[derive(Debug, PartialEq, Eq)]
pub enum Resource {
    Folder(String),
    Pdf(String),
    Unsupported(String),
}

/// Dispatcher request that returns the current public download shard.
pub fn dispatcher_api() -> String {
    let mut api = Url::parse("https://cloud.mail.ru/api/v2/dispatcher")
        .expect("Cloud Mail dispatcher URL is valid");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string();
    api.query_pairs_mut()
        .append_pair("api", "2")
        .append_pair("email", "")
        .append_pair("_", &now);
    api.into()
}

/// Download URL for the shared file, built from the current dispatcher shard.
pub fn download_url(url: &str, dispatcher_json: &str) -> Option<String> {
    let share = public_weblink(url)?;
    let root = json::parse(dispatcher_json)?;
    let body = root.get("body")?;
    let prefix = body.get("weblink_get")?.as_array().first()?.text("url")?;
    let mut direct = Url::parse(prefix).ok()?;
    let host = direct.host_str()?;
    if direct.scheme() != "https"
        || !(host.ends_with(".cloud.mail.ru")
            || host.ends_with(".datacloudmail.ru")
            || host.ends_with(".cldmail.ru"))
    {
        return None;
    }
    {
        let mut path = direct.path_segments_mut().ok()?;
        for part in share.split('/') {
            path.push(part);
        }
    }
    Some(direct.into())
}

/// Name if the metadata response describes a PDF file.
pub fn pdf_name(metadata_json: &str) -> Option<String> {
    let root = json::parse(metadata_json)?;
    if !matches!(root.get("status"), Some(Value::Number(n)) if n == "200") {
        return None;
    }
    let body = root.get("body")?;
    if body.text("type")? != "file" {
        return None;
    }
    let name = body.text("name")?;
    let is_pdf = name.to_ascii_lowercase().ends_with(".pdf")
        || body
            .text("mime")
            .or_else(|| body.text("mime_type"))
            .is_some_and(|mime| mime.eq_ignore_ascii_case("application/pdf"));
    is_pdf.then(|| name.to_owned())
}

/// Classify a successful file or folder response, including files whose share
/// URL has no extension and whose API response has no MIME field.
pub fn resource(metadata_json: &str, page_url: &str) -> Option<Resource> {
    let root = json::parse(metadata_json)?;
    if !matches!(root.get("status"), Some(Value::Number(n)) if n == "200") {
        return None;
    }
    let body = root.get("body")?;
    match body.text("type")? {
        "folder" => Some(Resource::Folder(atom(metadata_json, page_url)?)),
        "file" => {
            if let Some(name) = pdf_name(metadata_json) {
                return Some(Resource::Pdf(name));
            }
            Some(Resource::Unsupported(
                body.text("mime")
                    .or_else(|| body.text("mime_type"))
                    .unwrap_or("application/octet-stream")
                    .to_owned(),
            ))
        }
        _ => None,
    }
}

/// Turn the public folder response into Atom, which Brevier already renders
/// as a link listing. Invalid, private, and non-folder responses are rejected.
pub fn atom(json_text: &str, page_url: &str) -> Option<String> {
    let root = json::parse(json_text)?;
    // Cloud Mail sends status as a number.
    if !matches!(root.get("status"), Some(Value::Number(n)) if n == "200") {
        return None;
    }
    let body = root.get("body")?;
    if body.text("type")? != "folder" {
        return None;
    }
    let title = body.text("name")?;
    let entries = body.get("list")?.as_array();
    let mut out = format!(
        "<feed xmlns=\"http://www.w3.org/2005/Atom\"><title>{}</title><link rel=\"self\" href=\"{}\" />",
        xml(title),
        xml(page_url),
    );
    for entry in entries {
        let (Some(name), Some(weblink)) = (entry.text("name"), entry.text("weblink")) else {
            continue;
        };
        let Some(link) = public_url(weblink) else {
            continue;
        };
        out.push_str(&format!(
            "<entry><title>{}</title><link href=\"{}\" /></entry>",
            xml(name),
            xml(&link),
        ));
    }
    out.push_str("</feed>");
    Some(out)
}

fn public_url(weblink: &str) -> Option<String> {
    let mut url = Url::parse("https://cloud.mail.ru/").ok()?;
    {
        let mut path = url.path_segments_mut().ok()?;
        path.clear().push("public");
        for part in weblink.split('/') {
            if part.is_empty() {
                return None;
            }
            path.push(part);
        }
    }
    Some(url.into())
}

fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_public_urls_and_builds_file_api_without_suffix() {
        assert_eq!(
            public_weblink("https://cloud.mail.ru/public/share-key/opaque-id"),
            Some("share-key/opaque-id".to_owned())
        );
        let api =
            Url::parse(&file_api("https://cloud.mail.ru/public/share-key/opaque-id").unwrap())
                .unwrap();
        assert_eq!(api.path(), "/api/v2/file");
        assert_eq!(api.query_pairs().next().unwrap().1, "share-key/opaque-id");
        for url in [
            "http://cloud.mail.ru/public/key/name.pdf",
            "https://example.com/public/key/name.pdf",
            "https://cloud.mail.ru/my",
        ] {
            assert_eq!(file_api(url), None);
        }
    }

    #[test]
    fn classifies_metadata_pdf_even_without_mime() {
        assert_eq!(
            resource(
                include_str!("../../tests/fixtures/cloud-mail-file.json"),
                "https://cloud.mail.ru/public/fixture/share/opaque-id"
            ),
            Some(Resource::Pdf("Research.PDF".to_owned()))
        );
        assert_eq!(
            resource(
                r#"{"status":200,"body":{"type":"file","name":"notes.txt","mime":"text/plain"}}"#,
                "https://cloud.mail.ru/public/fixture/share/notes.txt"
            ),
            Some(Resource::Unsupported("text/plain".to_owned()))
        );
        assert_eq!(
            resource(
                r#"{"status":200,"body":{"type":"file","name":"document","mime_type":"application/pdf"}}"#,
                "https://cloud.mail.ru/public/fixture/share/opaque"
            ),
            Some(Resource::Pdf("document".to_owned()))
        );
        assert_eq!(
            resource(
                include_str!("../../tests/fixtures/cloud-mail-error.json"),
                "https://cloud.mail.ru/public/fixture/share"
            ),
            None
        );
        assert_eq!(
            resource("not json", "https://cloud.mail.ru/public/fixture/share"),
            None
        );
    }

    #[test]
    fn folder_fixture_preserves_nested_public_links() {
        let atom = atom(
            include_str!("../../tests/fixtures/cloud-mail-folder.json"),
            "https://cloud.mail.ru/public/fixture/share",
        )
        .unwrap();
        assert!(atom.contains("<title>Articles</title>"));
        assert!(atom.contains("https://cloud.mail.ru/public/fixture/share/Research.pdf"));
        assert!(atom.contains("https://cloud.mail.ru/public/fixture/share/Notes"));
        assert!(matches!(
            resource(
                include_str!("../../tests/fixtures/cloud-mail-folder.json"),
                "https://cloud.mail.ru/public/fixture/share"
            ),
            Some(Resource::Folder(_))
        ));
    }

    #[test]
    fn download_resolver_accepts_only_https_provider_shards() {
        assert!(
            download_url(
                "https://cloud.mail.ru/public/fixture/share/Research.pdf",
                include_str!("../../tests/fixtures/cloud-mail-dispatcher.json")
            )
            .is_some()
        );
        for response in [
            r#"{"body":{"weblink_get":[{"url":"http://download.cldmail.ru/a"}]}}"#,
            r#"{"body":{"weblink_get":[{"url":"https://download.cldmail.ru.evil.example/a"}]}}"#,
            r#"{"body":{"weblink_get":[{"url":"https://evil.example/a"}]}}"#,
            "not json",
        ] {
            assert_eq!(
                download_url(
                    "https://cloud.mail.ru/public/fixture/share/file.pdf",
                    response
                ),
                None
            );
        }
    }
}
