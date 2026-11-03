//! Public Yandex Disk shares are a JavaScript app shell. The tokenless public
//! resources API supplies metadata, folder entries, and temporary download URLs.

use url::Url;

use crate::json;

/// The canonical public share and optional path inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Share {
    pub public_url: String,
    pub path: Option<String>,
}

/// Readable outcome from public resource metadata.
#[derive(Debug, PartialEq, Eq)]
pub enum Resource {
    Folder(String),
    Pdf(String),
    Unsupported(String),
}

/// A validated page of a public folder listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FolderPage {
    pub total: usize,
    pub offset: usize,
    pub items: usize,
}

pub const FOLDER_PAGE_SIZE: usize = 1000;

/// Recognize a public Disk file or folder link, leaving unrelated Yandex pages alone.
pub fn share(url: &str) -> Option<Share> {
    let parsed = Url::parse(url).ok()?;
    if parsed.scheme() != "https" || parsed.host_str()? != "disk.yandex.ru" {
        return None;
    }
    let mut parts = parsed.path_segments()?;
    let kind = parts.next()?;
    if !matches!(kind, "d" | "i") {
        return None;
    }
    let id = parts.next()?;
    if id.is_empty() || parts.next().is_some() {
        return None;
    }
    let mut public = Url::parse("https://disk.yandex.ru/").ok()?;
    public.path_segments_mut().ok()?.push(kind).push(id);
    let path = parsed
        .query_pairs()
        .find(|(name, _)| name == "path")
        .map(|(_, value)| value.into_owned())
        .filter(|path| !path.is_empty());
    Some(Share {
        public_url: public.into(),
        path,
    })
}

/// Metadata endpoint for a public share or an entry inside a shared folder.
pub fn metadata_api(url: &str) -> Option<String> {
    metadata_page_api(url, FOLDER_PAGE_SIZE, 0)
}

/// Metadata endpoint for one page of a public folder.
pub fn metadata_page_api(url: &str, limit: usize, offset: usize) -> Option<String> {
    let mut api = Url::parse(&api_url("resources", url)?).ok()?;
    api.query_pairs_mut()
        .append_pair("limit", &limit.to_string())
        .append_pair("offset", &offset.to_string());
    Some(api.into())
}

/// Temporary download-link endpoint for a public PDF share.
pub fn download_api(url: &str) -> Option<String> {
    api_url("resources/download", url)
}

fn api_url(endpoint: &str, url: &str) -> Option<String> {
    let share = share(url)?;
    let mut api = Url::parse(&format!(
        "https://cloud-api.yandex.net/v1/disk/public/{endpoint}"
    ))
    .ok()?;
    {
        let mut query = api.query_pairs_mut();
        query.append_pair("public_key", &share.public_url);
        if let Some(path) = share.path {
            query.append_pair("path", &path);
        }
    }
    Some(api.into())
}

/// Name of a metadata response that describes a PDF file.
pub fn pdf_name(metadata: &str) -> Option<String> {
    let root = json::parse(metadata)?;
    if root.text("type")? != "file" {
        return None;
    }
    let name = root.text("name")?;
    let pdf_mime = root
        .text("mime_type")
        .is_some_and(|mime| mime.eq_ignore_ascii_case("application/pdf"));
    (pdf_mime || name.to_ascii_lowercase().ends_with(".pdf")).then(|| name.to_owned())
}

/// MIME type for an ordinary shared file, used by the existing refusal path.
pub fn file_mime(metadata: &str) -> Option<String> {
    let root = json::parse(metadata)?;
    (root.text("type")? == "file").then(|| {
        root.text("mime_type")
            .unwrap_or("application/octet-stream")
            .to_owned()
    })
}

/// Classify public metadata before the reader requests file bytes.
pub fn resource(metadata: &str, page_url: &str) -> Option<Resource> {
    if let Some(body) = atom(metadata, page_url) {
        return Some(Resource::Folder(body));
    }
    if let Some(name) = pdf_name(metadata) {
        return Some(Resource::Pdf(name));
    }
    file_mime(metadata).map(Resource::Unsupported)
}

/// Resolve an API-provided temporary URL only when it points to a Yandex Disk downloader.
pub fn download_url(response: &str) -> Option<String> {
    let root = json::parse(response)?;
    let href = Url::parse(root.text("href")?).ok()?;
    if href.scheme() != "https"
        || !matches!(
            href.host_str()?,
            "downloader.disk.yandex.ru" | "downloader.disk.yandex.net"
        )
    {
        return None;
    }
    Some(href.into())
}

/// Turn public directory metadata into Atom, which Brevier already renders as a link listing.
pub fn atom(metadata: &str, page_url: &str) -> Option<String> {
    atom_pages(&[metadata.to_owned()], page_url)
}

/// Validate the pagination metadata returned for one requested folder page.
pub fn folder_page(metadata: &str, expected_offset: usize) -> Option<FolderPage> {
    let root = json::parse(metadata)?;
    if root.text("type")? != "dir" {
        return None;
    }
    let embedded = root.get("_embedded")?;
    let total = integer(embedded.get("total")?)?;
    let offset = integer(embedded.get("offset")?)?;
    let items = embedded.get("items")?.as_array();
    if offset != expected_offset || offset > total || items.len() > total - offset {
        return None;
    }
    Some(FolderPage {
        total,
        offset,
        items: items.len(),
    })
}

fn integer(value: &json::Value) -> Option<usize> {
    match value {
        json::Value::Number(number) => number.parse().ok(),
        _ => None,
    }
}

/// Build a listing after fetching each required page in order.
pub fn folder_listing<F, E>(
    first: String,
    page_url: &str,
    mut fetch: F,
) -> Result<Option<String>, E>
where
    F: FnMut(usize) -> Result<String, E>,
{
    let mut pages = vec![first];
    let Some(first_page) = folder_page(&pages[0], 0) else {
        return Ok(None);
    };
    let mut collected = first_page.items;
    let total = first_page.total;
    if collected == 0 && total != 0 {
        return Ok(None);
    }
    while collected < total {
        let next = fetch(collected)?;
        let Some(page) = folder_page(&next, collected) else {
            return Ok(None);
        };
        if page.total != total || page.items == 0 {
            return Ok(None);
        }
        let Some(next_collected) = collected.checked_add(page.items) else {
            return Ok(None);
        };
        collected = next_collected;
        pages.push(next);
    }
    Ok(atom_pages(&pages, page_url))
}

fn atom_pages(metadata_pages: &[String], page_url: &str) -> Option<String> {
    let root = json::parse(metadata_pages.first()?)?;
    if root.text("type")? != "dir" {
        return None;
    }
    let title = root.text("name")?;
    let mut out = format!(
        "<feed xmlns=\"http://www.w3.org/2005/Atom\"><title>{}</title><link rel=\"self\" href=\"{}\" />",
        xml(title),
        xml(page_url),
    );
    for metadata in metadata_pages {
        let root = json::parse(metadata)?;
        let entries = root.get("_embedded")?.get("items")?.as_array();
        for entry in entries {
            let name = entry.text("name")?;
            let path = entry.text("path")?;
            let link = entry_url(page_url, path)?;
            out.push_str(&format!(
                "<entry><title>{}</title><link href=\"{}\" /></entry>",
                xml(name),
                xml(&link)
            ));
        }
    }
    out.push_str("</feed>");
    Some(out)
}

fn entry_url(page_url: &str, path: &str) -> Option<String> {
    if !path.starts_with('/') {
        return None;
    }
    let share = share(page_url)?;
    let mut url = Url::parse(&share.public_url).ok()?;
    url.query_pairs_mut().append_pair("path", path);
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
    fn recognizes_only_public_disk_links() {
        assert_eq!(
            share("https://disk.yandex.ru/d/abc"),
            Some(Share {
                public_url: "https://disk.yandex.ru/d/abc".to_owned(),
                path: None,
            })
        );
        assert_eq!(
            share("https://disk.yandex.ru/i/abc?path=%2FBooks%2Ffile.pdf"),
            Some(Share {
                public_url: "https://disk.yandex.ru/i/abc".to_owned(),
                path: Some("/Books/file.pdf".to_owned()),
            })
        );
        for url in [
            "http://disk.yandex.ru/d/abc",
            "https://yandex.ru/d/abc",
            "https://disk.yandex.ru/client/disk",
            "https://disk.yandex.ru/d/",
            "https://disk.yandex.ru/d/abc/extra",
        ] {
            assert_eq!(share(url), None, "{url}");
        }
    }

    #[test]
    fn builds_public_resource_urls_with_nested_path() {
        let metadata = metadata_api("https://disk.yandex.ru/d/abc?path=%2FBooks").unwrap();
        let parsed = Url::parse(&metadata).unwrap();
        assert_eq!(parsed.host_str(), Some("cloud-api.yandex.net"));
        assert_eq!(parsed.path(), "/v1/disk/public/resources");
        let query = parsed.query_pairs().collect::<Vec<_>>();
        assert_eq!(query[0].0, "public_key");
        assert_eq!(query[0].1, "https://disk.yandex.ru/d/abc");
        assert_eq!(query[1].0, "path");
        assert_eq!(query[1].1, "/Books");
        assert_eq!(query[2], ("limit".into(), "1000".into()));
        assert_eq!(query[3], ("offset".into(), "0".into()));
        let next =
            Url::parse(&metadata_page_api("https://disk.yandex.ru/d/abc", 1000, 20).unwrap())
                .unwrap();
        assert!(
            next.query_pairs()
                .any(|(key, value)| key == "offset" && value == "20")
        );

        let download = Url::parse(
            &download_api("https://disk.yandex.ru/d/abc?path=%2FBooks%2Fa.pdf").unwrap(),
        )
        .unwrap();
        assert_eq!(download.path(), "/v1/disk/public/resources/download");
    }

    #[test]
    fn collects_all_pages_and_preserves_order_and_escaped_links() {
        let first = r#"{"type":"dir","name":"Folder","_embedded":{"total":3,"offset":0,"items":[{"name":"A & B","path":"/A & B"},{"name":"Second","path":"/Second"}]}}"#.to_owned();
        let listing = folder_listing(first, "https://disk.yandex.ru/d/share", |offset| {
            assert_eq!(offset, 2);
            Ok::<_, ()>(r#"{"type":"dir","name":"Folder","_embedded":{"total":3,"offset":2,"items":[{"name":"Third","path":"/Third"}]}}"#.to_owned())
        }).unwrap().unwrap();
        assert_eq!(listing.matches("<entry>").count(), 3);
        assert!(listing.find("A &amp; B").unwrap() < listing.find("Second").unwrap());
        assert!(listing.find("Second").unwrap() < listing.find("Third").unwrap());
        assert!(listing.contains("path=%2FA+%26+B"));
    }

    #[test]
    fn rejects_bad_or_inconsistent_folder_pagination() {
        for json in [
            r#"{"type":"dir","_embedded":{"items":[]}}"#,
            r#"{"type":"dir","_embedded":{"total":2,"offset":1,"items":[{}]}}"#,
            r#"{"type":"dir","_embedded":{"total":1,"offset":0,"items":[{},{}]}}"#,
        ] {
            assert_eq!(folder_page(json, 0), None);
        }
        let first = r#"{"type":"dir","name":"Folder","_embedded":{"total":2,"offset":0,"items":[{"name":"A","path":"/A"}]}}"#.to_owned();
        assert_eq!(folder_listing(first, "https://disk.yandex.ru/d/share", |_| {
            Ok::<_, ()>(r#"{"type":"dir","name":"Folder","_embedded":{"total":3,"offset":1,"items":[{"name":"B","path":"/B"}]}}"#.to_owned())
        }).unwrap(), None);
        let first = r#"{"type":"dir","name":"Folder","_embedded":{"total":2,"offset":0,"items":[{"name":"A","path":"/A"}]}}"#.to_owned();
        assert_eq!(
            folder_listing(first, "https://disk.yandex.ru/d/share", |_| Err("network")),
            Err("network")
        );
    }

    #[test]
    fn creates_links_to_children_in_the_same_public_share() {
        let metadata = include_str!("../../tests/fixtures/yandex-folder.json");
        let feed = atom(metadata, "https://disk.yandex.ru/d/share?path=%2FBooks").unwrap();
        assert!(feed.contains("<title>Books &amp; Papers</title>"));
        assert!(
            feed.contains("https://disk.yandex.ru/d/share?path=%2FBooks%2FChildren+%26+Us.pdf")
        );
        assert!(feed.contains("https://disk.yandex.ru/d/share?path=%2FBooks%2FSubfolder"));
    }

    #[test]
    fn identifies_pdfs_and_rejects_unsafe_download_urls() {
        assert_eq!(
            pdf_name(include_str!("../../tests/fixtures/yandex-pdf.json")).as_deref(),
            Some("research.PDF")
        );
        assert_eq!(
            pdf_name(r#"{"type":"file","name":"notes.txt","mime_type":"text/plain"}"#),
            None
        );
        assert_eq!(
            file_mime(r#"{"type":"file","name":"notes.txt","mime_type":"text/plain"}"#).as_deref(),
            Some("text/plain")
        );
        assert!(download_url(include_str!("../../tests/fixtures/yandex-download.json")).is_some());
        for response in [
            r#"{"href":"http://downloader.disk.yandex.ru/a"}"#,
            r#"{"href":"https://evil.example/a"}"#,
            r#"{"href":"https://downloader.disk.yandex.ru.evil.example/a"}"#,
            "not json",
        ] {
            assert_eq!(download_url(response), None);
        }
    }

    #[test]
    fn malformed_folder_entries_fail_the_listing() {
        let metadata = r#"{"type":"dir","name":"Folder","_embedded":{"items":[{"name":"ok","path":"/ok"},{"name":"bad","path":"relative"},{"path":"/missing-name"}]}}"#;
        assert_eq!(atom(metadata, "https://disk.yandex.ru/d/share"), None);
        assert_eq!(
            atom(
                r#"{"type":"file","name":"x.pdf"}"#,
                "https://disk.yandex.ru/d/share"
            ),
            None
        );
    }

    #[test]
    fn metadata_selects_folder_pdf_or_existing_unsupported_type() {
        assert!(matches!(
            resource(
                include_str!("../../tests/fixtures/yandex-folder.json"),
                "https://disk.yandex.ru/d/share"
            ),
            Some(Resource::Folder(_))
        ));
        assert_eq!(
            resource(
                include_str!("../../tests/fixtures/yandex-pdf.json"),
                "https://disk.yandex.ru/i/share"
            ),
            Some(Resource::Pdf("research.PDF".to_owned()))
        );
        assert_eq!(
            resource(
                r#"{"type":"file","name":"notes.txt","mime_type":"text/plain"}"#,
                "https://disk.yandex.ru/i/share"
            ),
            Some(Resource::Unsupported("text/plain".to_owned()))
        );
        assert_eq!(
            resource(r#"{"type":"file"}"#, "https://disk.yandex.ru/i/share"),
            Some(Resource::Unsupported("application/octet-stream".to_owned()))
        );
    }
}
