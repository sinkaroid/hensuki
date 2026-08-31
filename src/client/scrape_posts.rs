use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::constant::Hensuki;
use crate::utils::{download_file_stream, get_size, log_data};

const POSTS_CONCURRENCY: usize = 3;
static RULE34_POST_LOCK: Mutex<()> = Mutex::const_new(());

pub async fn download_from_multiple_posts(file_path: &str, select_type: &str) -> Result<()> {
    let hsx = Hensuki::new();

    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .map_err(|e| anyhow!("Failed to open file {}: {}", file_path, e))?;

    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let l = line?.trim().to_string();
        if !l.is_empty() {
            lines.push(l);
        }
    }

    let total_lines = lines.len();

    // Input validation checks
    for line in &lines {
        if !hsx.supported.iter().any(|s| line.starts_with(s)) {
            anyhow::bail!("Unsupported site {}", line);
        }
        if line.starts_with(&hsx.e926) {
            anyhow::bail!("{} Change this to e621 instead", line);
        }
        if !Hensuki::validate_links(line) {
            anyhow::bail!(
                "Invalid links {}, expected posts or galleries links, not pages",
                line
            );
        }
    }

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(30))
        .build()?;

    let img_count = Arc::new(AtomicUsize::new(0));
    let img_already_exist = Arc::new(AtomicUsize::new(0));
    let img_failed = Arc::new(AtomicUsize::new(0));

    let select_type_owned = select_type.to_lowercase();

    let tasks = lines.into_iter().enumerate().map(|(idx, line)| {
        let current_task = idx + 1;
        let client_ref = client.clone();
        let hsx_ref = hsx.clone();
        let sel_type = select_type_owned.clone();
        let img_cnt = img_count.clone();
        let img_exist = img_already_exist.clone();
        let img_fail = img_failed.clone();

        async move {
            match process_single_post_with_retry(&client_ref, &hsx_ref, &line, &sel_type).await {
                Ok((final_img, final_name, mut extra_headers)) => {
                    let file_is_valid = if let Ok(meta) = std::fs::metadata(&final_name) {
                        meta.len() > 0
                    } else {
                        false
                    };

                    if file_is_valid {
                        let exist_count = img_exist.fetch_add(1, Ordering::SeqCst) + 1;
                        log_data(
                            &format!("Skipping... File already exists {}", final_name),
                            &exist_count.to_string(),
                        );
                    } else {
                        let _ = std::fs::remove_file(&final_name);
                        let mut headers = hsx_ref.get_headers();
                        headers.insert("Referer".to_string(), line.clone());

                        let cached_hdrs = crate::utils::get_cached_cf_headers(&line).await;
                        for (k, v) in cached_hdrs {
                            headers.insert(k, v);
                        }
                        for (k, v) in extra_headers.drain() {
                            headers.insert(k, v);
                        }

                        let mut dl_success = false;
                        let mut dl_attempts = 0;

                        while !dl_success && dl_attempts < 5 {
                            dl_attempts += 1;
                            match download_file_stream(&client_ref, &final_img, &final_name, &headers).await {
                                Ok(_) => {
                                    dl_success = true;
                                    let _ = img_cnt.fetch_add(1, Ordering::SeqCst);
                                    let host = line.split('/').nth(2).unwrap_or("unknown");
                                    log_data(
                                        &format!("{} / {} | {}", current_task, total_lines, host),
                                        &format!("{} | Downloaded {} MB", final_name, get_size(&final_name)),
                                    );
                                }
                                Err(e) => {
                                    let err_str = e.to_string();
                                    if err_str.contains("404") {
                                        eprintln!("Error: {} {} failed to download: {}", final_name, final_img, e);
                                        let _ = img_fail.fetch_add(1, Ordering::SeqCst);
                                        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("failed.log") {
                                            let _ = writeln!(f, "{}", line);
                                        }
                                        break;
                                    }
                                    log_data(
                                        &format!("Retrying media download in 1.5s (attempt {})", dl_attempts),
                                        &format!("{} | {}", final_name, err_str),
                                    );
                                    tokio::time::sleep(Duration::from_millis(1500)).await;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let err_str = e.to_string();
                    eprintln!(
                        "{} Skipping with no continue because {} | Check failed.log for more info",
                        line, e
                    );
                    let _ = img_fail.fetch_add(1, Ordering::SeqCst);
                    if err_str.contains("404") {
                        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("failed.log") {
                            let _ = writeln!(f, "{}", line);
                        }
                    }
                }
            }
        }
    });

    futures_util::stream::iter(tasks)
        .buffer_unordered(POSTS_CONCURRENCY)
        .collect::<Vec<()>>()
        .await;

    log_data(
        &format!("Downloaded {} contents", img_count.load(Ordering::SeqCst)),
        &format!("Skipped: {} | Failed: {}", img_already_exist.load(Ordering::SeqCst), img_failed.load(Ordering::SeqCst)),
    );

    Ok(())
}

async fn process_single_post_with_retry(
    client: &reqwest::Client,
    hsx: &Hensuki,
    line: &str,
    select_type: &str,
) -> Result<(String, String, HashMap<String, String>)> {
    let mut attempts = 0;
    loop {
        match process_single_post(client, hsx, line, select_type).await {
            Ok(res) => return Ok(res),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("404") || err_str.contains("builder error") || err_str.contains("Invalid") {
                    return Err(e);
                }
                attempts += 1;
                let jitter = (attempts * 400) % 1500;
                let backoff_ms = if err_str.contains("429") { 2500 + jitter as u64 } else { 1000 + jitter as u64 };
                log_data(
                    &format!("Retrying fetch in {:.1}s (attempt {})", backoff_ms as f64 / 1000.0, attempts),
                    &format!("{} | {}", line, err_str),
                );
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
            }
        }
    }
}

async fn process_single_post(
    client: &reqwest::Client,
    hsx: &Hensuki,
    line: &str,
    select_type: &str,
) -> Result<(String, String, HashMap<String, String>)> {
    let _guard = if line.contains("rule34.xxx") || line.contains("safebooru.org") {
        Some(RULE34_POST_LOCK.lock().await)
    } else {
        None
    };

    if line.contains("rule34.xxx") || line.contains("safebooru.org") {
        tokio::time::sleep(Duration::from_millis(400)).await;
    }

    let mut header_map = reqwest::header::HeaderMap::new();
    for (k, v) in hsx.get_headers() {
        if let (Ok(hn), Ok(hv)) = (k.parse::<reqwest::header::HeaderName>(), v.parse::<reqwest::header::HeaderValue>()) {
            header_map.insert(hn, hv);
        }
    }

    let (html_text, extra_headers) = if hsx.is_cloudflare(line) {
        crate::utils::fetch_with_cf_cache(client, line).await?
    } else {
        let response = match tokio::time::timeout(
            Duration::from_secs(15),
            client.get(line).headers(header_map).send(),
        )
        .await
        {
            Ok(res) => res?,
            Err(_) => anyhow::bail!("Timeout (15s) connecting to {}", line),
        };

        if response.status().as_u16() == 429 || response.status().as_u16() == 403 || response.status().as_u16() == 404 {
            let cf_res = crate::utils::fetch_with_cf_cache(client, line).await;
            if let Ok(data) = cf_res {
                data
            } else if response.status().as_u16() == 429 {
                anyhow::bail!("HTTP status 429 Too Many Requests for {}", line);
            } else {
                anyhow::bail!("HTTP status {} for {}", response.status(), line);
            }
        } else if !response.status().is_success() {
            anyhow::bail!("HTTP status {}", response.status());
        } else {
            let text = match tokio::time::timeout(Duration::from_secs(15), response.text()).await {
                Ok(t) => t?,
                Err(_) => anyhow::bail!("Timeout (15s) reading response text for {}", line),
            };
            (text, HashMap::new())
        }
    };

    let document = Html::parse_document(&html_text);
    let (image_original, image_small) = parse_post_media(hsx, line, &html_text, &document)?;

    let (raw_img, final_name) = match select_type {
        "1" | "o" | "" | "original" | "ori" => {
            let name = Hensuki::extract_img_name(&image_original);
            (image_original, name)
        }
        "2" | "s" | "smaller" | "small" => {
            let name = Hensuki::extract_img_name(&image_small);
            (image_small, name)
        }
        _ => anyhow::bail!("Invalid type"),
    };

    let final_img = Hensuki::ensure_absolute_url(&raw_img, line);

    if final_img.is_empty() {
        anyhow::bail!("Empty image URL extracted for {}", line);
    }

    Ok((final_img, final_name, extra_headers))
}

pub fn parse_post_media(
    hsx: &Hensuki,
    line: &str,
    _html_text: &str,
    document: &Html,
) -> Result<(String, String)> {
    let mut image_original = String::new();
    let mut image_small = String::new();

    if line.starts_with(&hsx.safebooru)
        || line.starts_with(&hsx.tbib)
        || line.starts_with(&hsx.xbooru)
        || line.starts_with(&hsx.hypnohub)
        || line.starts_with(&hsx.rule34)
    {
        if line.starts_with(&hsx.hypnohub) || line.starts_with(&hsx.rule34) {
            let re_orig = Regex::new(r#"<a(.+?)>\s*Original image"#).unwrap();
            let link_list_sel = Selector::parse("div.link-list li").unwrap();
            let lis: Vec<String> = document.select(&link_list_sel).map(|e| e.html()).collect();
            let first_html = lis.join("\n");
            let clean_html = first_html.replace(" Original image", "Original image");

            if let Some(cap) = re_orig.captures(&clean_html) {
                let tag = cap.get(0).map_or("", |m| m.as_str());
                image_original = Hensuki::get_href_value(tag);
            }

            let content_img_sel = Selector::parse("div.content img").unwrap();
            if let Some(img) = document.select(&content_img_sel).next() {
                if let Some(src) = img.value().attr("src") {
                    image_small = Hensuki::proper_protocols(src.split('?').next().unwrap_or(src));
                }
            }
        } else {
            let re_orig = Regex::new(r#"<li>(.+?)>Original image"#).unwrap();
            let sidebar_sel = Selector::parse("div.sidebar li").unwrap();
            let lis: Vec<String> = document.select(&sidebar_sel).map(|e| e.html()).collect();
            let first_html = lis.join("\n");

            if let Some(cap) = re_orig.captures(&first_html) {
                let tag = cap.get(0).map_or("", |m| m.as_str());
                image_original = Hensuki::get_href_value(tag);
            }

            let content_img_sel = Selector::parse("div.content img").unwrap();
            if let Some(img) = document.select(&content_img_sel).next() {
                if let Some(src) = img.value().attr("src") {
                    image_small = Hensuki::proper_protocols(src.split('?').next().unwrap_or(src));
                }
            }
        }

        if image_original.is_empty() {
            let source_sel = Selector::parse("div.content source").unwrap();
            if let Some(src_elem) = document.select(&source_sel).next() {
                if let Some(src) = src_elem.value().attr("src") {
                    image_original = Hensuki::proper_protocols(src.split('?').next().unwrap_or(src));
                }
            }
        }
    } else if line.starts_with(&hsx.danbooru) {
        let sel = Selector::parse("section#post-options li#post-option-download a").unwrap();
        if let Some(a_elem) = document.select(&sel).next() {
            let a_html = a_elem.html();
            let href = Hensuki::get_href_value(&a_html);
            image_original = href.split('?').next().unwrap_or(&href).to_string();
        }
    } else if line.starts_with(&hsx.gelbooru) {
        let re_orig = Regex::new(r#"<li>(.+?)>Original image"#).unwrap();
        let aside_sel = Selector::parse("section.aside li").unwrap();
        let lis: Vec<String> = document.select(&aside_sel).map(|e| e.html()).collect();
        let first_html = lis.join("\n");

        if let Some(cap) = re_orig.captures(&first_html) {
            let tag = cap.get(0).map_or("", |m| m.as_str());
            image_original = Hensuki::get_href_value(tag);
        }

        let pic_img_sel = Selector::parse("picture img").unwrap();
        if let Some(img) = document.select(&pic_img_sel).next() {
            if let Some(src) = img.value().attr("src") {
                image_small = src.to_string();
            }
        }
    } else if line.starts_with(&hsx.realbooru) {
        let sel = Selector::parse("div[style*='text-align: right'] a").unwrap();
        if let Some(a_elem) = document.select(&sel).next() {
            if let Some(href) = a_elem.value().attr("href") {
                image_original = href.to_string();
            }
        }
    } else if line.starts_with(&hsx.yandere)
        || line.starts_with(&hsx.konachan)
        || line.starts_with(&hsx.konachan_net)
    {
        let re_orig = Regex::new(r#"<li><a class="original(.+?)>View larger version"#).unwrap();
        let sidebar_sel = Selector::parse("div.sidebar li").unwrap();
        let lis: Vec<String> = document.select(&sidebar_sel).map(|e| e.html()).collect();
        let first_html = lis.join("\n");

        if let Some(cap) = re_orig.captures(&first_html) {
            let tag = cap.get(0).map_or("", |m| m.as_str());
            image_original = Hensuki::get_href_value(tag);
        }

        if image_original.is_empty() {
            let highres_sel = Selector::parse("a#highres, a.original-file-unchanged, a.original-file-changed, div.sidebar a[href*='/image/']").unwrap();
            for a_elem in document.select(&highres_sel) {
                if let Some(href) = a_elem.value().attr("href") {
                    if href.contains("/image/") {
                        image_original = href.to_string();
                        break;
                    }
                }
            }
        }

        let content_img_sel = Selector::parse("img#image, img.image, div.content img[alt], div.content img").unwrap();
        for img in document.select(&content_img_sel) {
            if let Some(src) = img.value().attr("src") {
                if !src.contains("/images/bam/") && !src.contains("jlist") && !src.contains("/ads/") {
                    image_small = src.to_string();
                    break;
                }
            }
        }
    } else if line.starts_with(&hsx.e621) {
        let download_sel = Selector::parse("div#image-download-link a").unwrap();
        if let Some(a_elem) = document.select(&download_sel).next() {
            if let Some(href) = a_elem.value().attr("href") {
                image_original = href.to_string();
            }
        }
        let img_sel = Selector::parse("img#image.fit-window").unwrap();
        if let Some(img) = document.select(&img_sel).next() {
            if let Some(src) = img.value().attr("src") {
                image_small = src.to_string();
            }
        }
    }

    if image_original.contains("/images/bam/") || image_original.contains("jlist") {
        image_original = String::new();
    }
    if image_small.contains("/images/bam/") || image_small.contains("jlist") {
        image_small = String::new();
    }

    if image_original.is_empty() && !image_small.is_empty() {
        image_original = image_small.clone();
    }
    if image_small.is_empty() && !image_original.is_empty() {
        image_small = image_original.clone();
    }

    if image_original.is_empty() && image_small.is_empty() {
        anyhow::bail!("NoneType / Failed to parse image links");
    }

    Ok((image_original, image_small))
}
