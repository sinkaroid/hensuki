use hensuki::constant::Hensuki;
use hensuki::utils::{clean_html, get_hostname, get_size};

#[test]
fn test_hensuki_initialization() {
    let jgx = Hensuki::new();
    assert_eq!(jgx.supported.len(), 13);
    assert_eq!(jgx.expected_format.len(), 6);
    assert!(jgx.supported.contains(&"https://gelbooru.com".to_string()));
    assert!(jgx.supported.contains(&"https://yande.re".to_string()));
}

#[test]
fn test_filename_extraction() {
    // Normal URL
    let url = "https://safebooru.org/images/123/sample_456.jpg?123456";
    assert_eq!(Hensuki::extract_img_name(url), "sample_456.jpg");

    // Very long filename truncation (>200 chars)
    let long_stem = "a".repeat(250);
    let long_url = format!("https://example.com/{}.png", long_stem);
    let extracted = Hensuki::extract_img_name(&long_url);
    assert!(extracted.len() <= 200);
    assert!(extracted.ends_with(".png"));
}

#[test]
fn test_protocol_handling() {
    assert_eq!(
        Hensuki::proper_protocols("//yande.re/image.jpg"),
        "https://yande.re/image.jpg"
    );
    assert_eq!(
        Hensuki::change_protocol("http://lolibooru.moe/post"),
        "https://lolibooru.moe/post"
    );
}

#[test]
fn test_link_validation() {
    // Valid posts links
    assert!(Hensuki::validate_links("https://danbooru.donmai.us/posts/5555"));
    assert!(Hensuki::validate_links("https://gelbooru.com/index.php?page=post&s=view&id=1234"));
    assert!(Hensuki::validate_links("https://yande.re/post/show/1000"));
    assert!(Hensuki::validate_links("https://rule34.paheal.net/post/view/9999"));

    // Invalid page lists links
    assert!(!Hensuki::validate_links("https://danbooru.donmai.us/posts?tags=cat"));
    assert!(!Hensuki::validate_links("https://gelbooru.com/index.php?page=post&s=list&tags=all"));
}

#[test]
fn test_utils_helpers() {
    assert_eq!(get_hostname("https://e621.net/posts/123"), "https://e621.net");
    assert_eq!(clean_html("<div><span>Test</span></div>"), "Test");
    assert_eq!(get_size("non_existent_file_path.bin"), "0.00");
}
