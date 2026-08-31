use regex::Regex;
use std::collections::HashMap;

pub const VERSION: &str = "3.0.2";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
pub const FROM_EMAIL: &str = "hey@sinkaroid.org";

#[derive(Debug, Clone)]
pub struct Hensuki {
    pub gelbooru: String,
    pub safebooru: String,
    pub danbooru: String,
    pub rule34: String,
    pub tbib: String,
    pub xbooru: String,
    pub realbooru: String,
    pub yandere: String,
    pub konachan: String,
    pub konachan_net: String,
    pub hypnohub: String,
    pub e621: String,
    pub e926: String,
    pub supported: Vec<String>,
    pub cloudflare: Vec<String>,
    pub expected_format: Vec<String>,
}

impl Default for Hensuki {
    fn default() -> Self {
        Self::new()
    }
}

impl Hensuki {
    pub fn new() -> Self {
        let gelbooru = "https://gelbooru.com".to_string();
        let safebooru = "https://safebooru.org".to_string();
        let danbooru = "https://danbooru.donmai.us".to_string();
        let rule34 = "https://rule34.xxx".to_string();
        let tbib = "https://tbib.org".to_string();
        let xbooru = "https://xbooru.com".to_string();
        let realbooru = "https://realbooru.com".to_string();
        let yandere = "https://yande.re".to_string();
        let konachan = "https://konachan.com".to_string();
        let konachan_net = "https://konachan.net".to_string();
        let hypnohub = "https://hypnohub.net".to_string();
        let e621 = "https://e621.net".to_string();
        let e926 = "https://e926.net".to_string();

        let supported = vec![
            gelbooru.clone(),
            safebooru.clone(),
            danbooru.clone(),
            rule34.clone(),
            tbib.clone(),
            xbooru.clone(),
            realbooru.clone(),
            yandere.clone(),
            konachan.clone(),
            konachan_net.clone(),
            hypnohub.clone(),
            e621.clone(),
            e926.clone(),
        ];

        let cloudflare = vec![danbooru.clone(), konachan.clone()];

        let expected_format = vec![
            ".jpg".to_string(),
            ".png".to_string(),
            ".gif".to_string(),
            ".webm".to_string(),
            ".mp4".to_string(),
            ".jpeg".to_string(),
        ];

        Self {
            gelbooru,
            safebooru,
            danbooru,
            rule34,
            tbib,
            xbooru,
            realbooru,
            yandere,
            konachan,
            konachan_net,
            hypnohub,
            e621,
            e926,
            supported,
            cloudflare,
            expected_format,
        }
    }

    pub fn is_cloudflare(&self, url: &str) -> bool {
        self.cloudflare.iter().any(|domain| url.starts_with(domain))
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), USER_AGENT.to_string());
        headers.insert("From".to_string(), FROM_EMAIL.to_string());
        headers
    }

    pub fn ensure_absolute_url(raw_url: &str, base_url: &str) -> String {
        let unescaped = raw_url.replace("&amp;", "&");
        let trimmed = unescaped.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        if trimmed.starts_with("//") {
            format!("https:{}", trimmed)
        } else if trimmed.starts_with('/') {
            format!("{}{}", crate::utils::get_hostname(base_url), trimmed)
        } else if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            format!("{}/{}", crate::utils::get_hostname(base_url), trimmed)
        } else {
            trimmed.to_string()
        }
    }

    pub fn extract_img_name(url: &str) -> String {
        let last_part = url.split('/').last().unwrap_or(url);
        let clean_name = last_part.split('?').next().unwrap_or(last_part);

        if clean_name.len() > 200 {
            if let Some(ext_idx) = clean_name.rfind('.') {
                let ext = &clean_name[ext_idx + 1..];
                let max_stem_len = 200 - ext.len() - 1;
                format!("{}.{}", &clean_name[..max_stem_len], ext)
            } else {
                clean_name[..200].to_string()
            }
        } else {
            clean_name.to_string()
        }
    }

    pub fn proper_protocols(url: &str) -> String {
        if url.starts_with("//") {
            format!("https:{}", url)
        } else {
            url.to_string()
        }
    }

    pub fn get_href_value(tag: &str) -> String {
        let re = Regex::new(r#"href="([^"]+)""#).unwrap();
        if let Some(caps) = re.captures(tag) {
            caps.get(1).map_or("", |m| m.as_str()).to_string()
        } else {
            String::new()
        }
    }

    pub fn change_protocol(url: &str) -> String {
        if url.starts_with("http://") {
            url.replace("http://", "https://")
        } else {
            url.to_string()
        }
    }

    pub fn proper_yandere_link(url: &str) -> String {
        let re_https = Regex::new(r".*https://").unwrap();
        let re_http = Regex::new(r".*http://").unwrap();
        if url.contains("https://") {
            re_https.replace(url, "https://").to_string()
        } else if url.contains("http://") {
            re_http.replace(url, "http://").to_string()
        } else {
            format!("https://{}", url)
        }
    }

    pub fn validate_links(line: &str) -> bool {
        let expected = [
            "/post/show/",
            "/posts/",
            "page=post&s=view",
            "page=post&amp;s=view",
            "/post/view/",
        ];

        expected.iter().any(|exp| line.contains(exp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_img_name() {
        assert_eq!(
            Hensuki::extract_img_name("https://safebooru.org/images/123/sample_foo.jpg?12345"),
            "sample_foo.jpg"
        );
    }

    #[test]
    fn test_ensure_absolute_url() {
        assert_eq!(
            Hensuki::ensure_absolute_url(
                "//safebooru.org/images/foo.jpg",
                "https://safebooru.org"
            ),
            "https://safebooru.org/images/foo.jpg"
        );
    }

    #[test]
    fn test_proper_yandere_link() {
        assert_eq!(
            Hensuki::proper_yandere_link("#pl https://yande.re/post/show/1255170"),
            "https://yande.re/post/show/1255170"
        );
    }

    #[test]
    fn test_proper_protocols() {
        assert_eq!(
            Hensuki::proper_protocols("//safebooru.org/images/foo.jpg"),
            "https://safebooru.org/images/foo.jpg"
        );
    }

    #[test]
    fn test_validate_links() {
        assert!(Hensuki::validate_links(
            "https://safebooru.org/index.php?page=post&s=view&id=123"
        ));
        assert!(Hensuki::validate_links(
            "https://danbooru.donmai.us/posts/123"
        ));
        assert!(!Hensuki::validate_links(
            "https://safebooru.org/index.php?page=post&s=list"
        ));
    }

    #[test]
    fn test_is_cloudflare() {
        let hsx = Hensuki::new();
        assert!(hsx.is_cloudflare("https://danbooru.donmai.us/posts/123"));
        assert!(hsx.is_cloudflare("https://konachan.com/post/show/123"));
        assert!(!hsx.is_cloudflare("https://safebooru.org/index.php?page=post"));
    }
}
