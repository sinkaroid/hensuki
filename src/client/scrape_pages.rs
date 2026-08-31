use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::client::scrape_posts::parse_post_media;
use crate::constant::Hensuki;
use crate::utils::{clean_html, download_file_stream, get_hostname, get_size, log_data};

const PAGES_CONCURRENCY: usize = 3;
static RULE34_PAGE_LOCK: Mutex<()> = Mutex::const_new(());

pub async fn download_from_multiple_pages(file_path: &str, select_type: &str) -> Result<()> {
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

    // Input validation checks
    for line in &lines {
        if !hsx.supported.iter().any(|s| line.starts_with(s)) {
            anyhow::bail!("Unsupported site: {}", line);
        }
        if line.starts_with(&hsx.e926) {
            anyhow::bail!("{} Change this to e621 instead", line);
        }
        if Hensuki::validate_links(line) {
            anyhow::bail!(
                "Invalid list of pages {}, expected one or more pages not posts or galleries",
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
    let mut tags_count: Vec<String> = Vec::new();
    let select_type_owned = select_type.to_lowercase();

    for line in lines {
        let mut header_map = reqwest::header::HeaderMap::new();
        for (k, v) in hsx.get_headers() {
            if let (Ok(hn), Ok(hv)) = (k.parse::<reqwest::header::HeaderName>(), v.parse::<reqwest::header::HeaderValue>()) {
                header_map.insert(hn, hv);
            }
        }

        let (final_url, html_text) = if hsx.is_cloudflare(&line) {
            let (html, _) = crate::utils::fetch_with_cf_cache(&client, &line).await?;
            (line.clone(), html)
        } else {
            let res = client
                .get(&line)
                .headers(header_map)
                .send()
                .await?;

            if res.status().as_u16() == 403 || res.status().as_u16() == 404 || res.status().as_u16() == 429 {
                let (html, _) = crate::utils::fetch_with_cf_cache(&client, &line).await?;
                (line.clone(), html)
            } else if !res.status().is_success() {
                eprintln!("Error: {}", res.status());
                let _ = img_failed.fetch_add(1, Ordering::SeqCst);
                if res.status().as_u16() == 404 {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("failed.log") {
                        let _ = writeln!(f, "{}", line);
                    }
                }
                continue;
            } else {
                let u = res.url().to_string();
                let text = res.text().await?;
                (u, text)
            }
        };

        let document = Html::parse_document(&html_text);

        let title_sel = Selector::parse("title").unwrap();
        if let Some(title_elem) = document.select(&title_sel).next() {
            tags_count.push(clean_html(&title_elem.inner_html()));
        }

        let mut list_gallery: Vec<String> = Vec::new();

        if line.starts_with(&hsx.safebooru)
            || line.starts_with(&hsx.tbib)
            || line.starts_with(&hsx.xbooru)
            || line.starts_with(&hsx.hypnohub)
            || line.starts_with(&hsx.rule34)
        {
            let thumb_sel = Selector::parse("span.thumb a").unwrap();
            let host = get_hostname(&final_url).trim_end_matches('/').to_string();
            for a_elem in document.select(&thumb_sel) {
                if let Some(href) = a_elem.value().attr("href") {
                    let unescaped_href = href.replace("&amp;", "&");
                    let clean_href = if unescaped_href.starts_with('/') {
                        unescaped_href
                    } else {
                        format!("/{}", unescaped_href)
                    };
                    let full_link = format!("{}{}", host, clean_href);
                    list_gallery.push(full_link);
                }
            }
        } else if line.starts_with(&hsx.danbooru) {
            let preview_sel = Selector::parse("a.post-preview-link").unwrap();
            for a_elem in document.select(&preview_sel) {
                if let Some(href) = a_elem.value().attr("href") {
                    list_gallery.push(format!("{}{}", hsx.danbooru, href));
                }
            }
        } else if line.starts_with(&hsx.gelbooru) {
            let preview_sel = Selector::parse("article.thumbnail-preview a").unwrap();
            for a_elem in document.select(&preview_sel) {
                if let Some(href) = a_elem.value().attr("href") {
                    list_gallery.push(href.to_string());
                }
            }
        } else if line.starts_with(&hsx.realbooru) {
            let thumb_sel = Selector::parse("div.col.thumb a").unwrap();
            for a_elem in document.select(&thumb_sel) {
                if let Some(href) = a_elem.value().attr("href") {
                    list_gallery.push(href.to_string());
                }
            }
        } else if line.starts_with(&hsx.yandere)
            || line.starts_with(&hsx.konachan)
            || line.starts_with(&hsx.konachan_net)
        {
            let plid_sel = Selector::parse("span.plid").unwrap();
            for span in document.select(&plid_sel) {
                let text = span.text().collect::<Vec<_>>().join("");
                let link = Hensuki::proper_yandere_link(&Hensuki::change_protocol(&text));
                list_gallery.push(link);
            }
        } else if line.starts_with(&hsx.e621) || line.starts_with(&hsx.e926) {
            let article_sel = Selector::parse("article[data-file-url]").unwrap();
            for article in document.select(&article_sel) {
                if let Some(file_url) = article.value().attr("data-file-url") {
                    list_gallery.push(file_url.to_string());
                }
            }
        }

        let total_gallery = list_gallery.len();

        let gallery_tasks = list_gallery.into_iter().enumerate().map(|(idx, galeri)| {
            let current_task = idx + 1;
            let client_ref = client.clone();
            let hsx_ref = hsx.clone();
            let sel_type = select_type_owned.clone();
            let img_cnt = img_count.clone();
            let img_exist = img_already_exist.clone();
            let img_fail = img_failed.clone();
            let line_ref = line.clone();

            async move {
                let media_res = if line_ref.starts_with(&hsx_ref.e621) {
                    Ok((galeri.clone(), galeri.clone(), HashMap::new()))
                } else {
                    fetch_and_parse_gallery_with_retry(&client_ref, &hsx_ref, &galeri).await
                };

                match media_res {
                    Ok((image_original, image_small, mut extra_headers)) => {
                        let (raw_img, final_name) = match sel_type.as_str() {
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

                        let final_img = Hensuki::ensure_absolute_url(&raw_img, &galeri);

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
                            headers.insert("Referer".to_string(), galeri.clone());

                            let cached_hdrs = crate::utils::get_cached_cf_headers(&galeri).await;
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
                                        let host = galeri.split('/').nth(2).unwrap_or("unknown");
                                        log_data(
                                            &format!("{} / {} | {}", current_task, total_gallery, host),
                                            &format!("{} | Downloaded {} MB", final_name, get_size(&final_name)),
                                        );
                                    }
                                    Err(e) => {
                                        let err_str = e.to_string();
                                        if err_str.contains("404") {
                                            eprintln!("Error: {} failed to download: {}", galeri, e);
                                            let _ = img_fail.fetch_add(1, Ordering::SeqCst);
                                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("failed.log") {
                                                let _ = writeln!(f, "{}", galeri);
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
                            galeri, e
                        );
                        let _ = img_fail.fetch_add(1, Ordering::SeqCst);
                        if err_str.contains("404") {
                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("failed.log") {
                                let _ = writeln!(f, "{}", galeri);
                            }
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            }
        });

        futures_util::stream::iter(gallery_tasks)
            .buffer_unordered(PAGES_CONCURRENCY)
            .collect::<Vec<_>>()
            .await;

        log_data(
            &format!("Downloaded {} contents (Skipped: {}, Failed: {})", img_count.load(Ordering::SeqCst), img_already_exist.load(Ordering::SeqCst), img_failed.load(Ordering::SeqCst)),
            &format!("which is comes from: {:?} with {} pages", tags_count, tags_count.len()),
        );
    }

    Ok(())
}

async fn fetch_and_parse_gallery_with_retry(
    client: &reqwest::Client,
    hsx: &Hensuki,
    galeri: &str,
) -> Result<(String, String, HashMap<String, String>)> {
    let mut attempts = 0;
    loop {
        match fetch_and_parse_gallery(client, hsx, galeri).await {
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
                    &format!("{} | {}", galeri, err_str),
                );
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
            }
        }
    }
}

async fn fetch_and_parse_gallery(
    client: &reqwest::Client,
    hsx: &Hensuki,
    galeri: &str,
) -> Result<(String, String, HashMap<String, String>)> {
    let _guard = if galeri.contains("rule34.xxx") || galeri.contains("safebooru.org") {
        Some(RULE34_PAGE_LOCK.lock().await)
    } else {
        None
    };

    if galeri.contains("rule34.xxx") || galeri.contains("safebooru.org") {
        tokio::time::sleep(Duration::from_millis(400)).await;
    }

    let mut header_map = reqwest::header::HeaderMap::new();
    for (k, v) in hsx.get_headers() {
        if let (Ok(hn), Ok(hv)) = (k.parse::<reqwest::header::HeaderName>(), v.parse::<reqwest::header::HeaderValue>()) {
            header_map.insert(hn, hv);
        }
    }

    let (html_text, extra_headers) = if hsx.is_cloudflare(galeri) {
        crate::utils::fetch_with_cf_cache(client, galeri).await?
    } else {
        let response = match tokio::time::timeout(
            Duration::from_secs(15),
            client.get(galeri).headers(header_map.clone()).send(),
        )
        .await
        {
            Ok(res) => res?,
            Err(_) => anyhow::bail!("Timeout (15s) connecting to {}", galeri),
        };

        if response.status().as_u16() == 429 || response.status().as_u16() == 403 || response.status().as_u16() == 404 {
            let cf_res = crate::utils::fetch_with_cf_cache(client, galeri).await;
            if let Ok(data) = cf_res {
                data
            } else if response.status().as_u16() == 429 {
                anyhow::bail!("HTTP status 429 Too Many Requests for {}", galeri);
            } else {
                anyhow::bail!("HTTP status {} for {}", response.status(), galeri);
            }
        } else if !response.status().is_success() {
            anyhow::bail!("HTTP status {}", response.status());
        } else {
            let text = match tokio::time::timeout(Duration::from_secs(15), response.text()).await {
                Ok(t) => t?,
                Err(_) => anyhow::bail!("Timeout (15s) reading response text for {}", galeri),
            };
            (text, HashMap::new())
        }
    };

    let document = Html::parse_document(&html_text);
    let (image_original, image_small) = parse_post_media(hsx, galeri, &html_text, &document)?;

    Ok((image_original, image_small, extra_headers))
}
