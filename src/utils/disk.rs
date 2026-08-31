use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub fn get_size(file_path: &str) -> String {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        let bytes = metadata.len();
        let mb = bytes as f64 / (1024.0 * 1024.0);
        format!("{:.2}", mb)
    } else {
        "0.00".to_string()
    }
}

pub fn clean_html(string: &str) -> String {
    let re = Regex::new(r"<[^>]*>").expect("Valid regex");
    let cleaned = re.replace_all(string, "");
    cleaned.trim().to_string()
}

pub async fn download_file_stream(
    client: &reqwest::Client,
    url: &str,
    dest_path: &str,
    headers_map: &HashMap<String, String>,
) -> Result<u64> {
    let mut header_map = HeaderMap::new();
    for (k, v) in headers_map {
        if let (Ok(hn), Ok(hv)) = (k.parse::<HeaderName>(), v.parse::<HeaderValue>()) {
            header_map.insert(hn, hv);
        }
    }

    let mut response = match tokio::time::timeout(
        Duration::from_secs(20),
        client.get(url).headers(header_map.clone()).send(),
    )
    .await
    {
        Ok(res) => res.with_context(|| format!("Failed to send GET request to {}", url))?,
        Err(_) => anyhow::bail!("Request timeout (20s) connecting to {}", url),
    };

    let mut retries = 0;
    while response.status().as_u16() == 429 && retries < 3 {
        retries += 1;
        tokio::time::sleep(Duration::from_millis(2000)).await;
        response = match tokio::time::timeout(
            Duration::from_secs(20),
            client.get(url).headers(header_map.clone()).send(),
        )
        .await
        {
            Ok(res) => res.with_context(|| format!("Failed to retry GET request to {}", url))?,
            Err(_) => anyhow::bail!("Retry request timeout (20s) connecting to {}", url),
        };
    }

    if !response.status().is_success() {
        anyhow::bail!("HTTP status {} for {}", response.status(), url);
    }

    let content_length = response.content_length();

    let pb = match content_length {
        Some(len) => {
            let pb = ProgressBar::new(len);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}")
                    .expect("Valid template")
                    .progress_chars("#>-"),
            );
            pb
        }
        None => {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {bytes} ({bytes_per_sec}) {msg}")
                    .expect("Valid template"),
            );
            pb
        }
    };

    pb.set_message(dest_path.to_string());

    let mut dest_file = File::create(dest_path)
        .await
        .with_context(|| format!("Failed to create output file {}", dest_path))?;

    let mut stream = response.bytes_stream();
    let mut total_bytes: u64 = 0;

    let write_res = async {
        loop {
            let chunk_opt = match tokio::time::timeout(Duration::from_secs(20), stream.next()).await {
                Ok(opt) => opt,
                Err(_) => anyhow::bail!("Stream chunk download timed out after 20s for {}", url),
            };

            match chunk_opt {
                Some(chunk_result) => {
                    let chunk = chunk_result.with_context(|| format!("Error reading stream chunk from {}", url))?;
                    dest_file.write_all(&chunk).await?;
                    let len = chunk.len() as u64;
                    total_bytes += len;
                    pb.inc(len);
                }
                None => break,
            }
        }
        dest_file.flush().await?;
        Ok::<u64, anyhow::Error>(total_bytes)
    }.await;

    pb.finish_and_clear();

    match write_res {
        Ok(bytes) => {
            if bytes == 0 {
                let _ = tokio::fs::remove_file(dest_path).await;
                anyhow::bail!("Downloaded file is 0 bytes for {}", url);
            }
            Ok(bytes)
        }
        Err(e) => {
            let _ = tokio::fs::remove_file(dest_path).await;
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size_nonexistent() {
        assert_eq!(get_size("nonexistent_file.xyz"), "0.00");
    }

    #[test]
    fn test_clean_html() {
        assert_eq!(clean_html("<div>Hello <b>World</b></div>"), "Hello World");
    }
}
