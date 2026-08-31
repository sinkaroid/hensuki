use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

const FLARESOLVERR_IMAGE: &str = "ghcr.io/sinkaroid/matoi-flaresolverr:latest";
const CONTAINER_NAME: &str = "matoi-flaresolverr";
const FLARESOLVERR_ENDPOINT: &str = "http://127.0.0.1:8191/v1";

static SESSION_CACHE: LazyLock<Mutex<HashMap<String, (String, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Serialize)]
struct FlareSolverrRequest {
    cmd: String,
    url: String,
    #[serde(rename = "maxTimeout")]
    max_timeout: u32,
}

#[derive(Deserialize)]
struct FlareSolverrResponse {
    status: String,
    solution: Option<FlareSolverrSolution>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct FlareSolverrSolution {
    response: String,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    cookies: Option<Vec<FlareSolverrCookie>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlareSolverrCookie {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FlareSolverrResult {
    pub html: String,
    pub user_agent: String,
    pub cookie_header: String,
}

pub fn detect_container_cmd() -> Result<&'static str> {
    if Command::new("podman").arg("--version").output().is_ok() {
        Ok("podman")
    } else if Command::new("docker").arg("--version").output().is_ok() {
        Ok("docker")
    } else {
        Err(anyhow!("Neither podman nor docker CLI is available on host machine"))
    }
}

pub async fn ensure_flaresolverr_running(client: &reqwest::Client) -> Result<()> {
    if let Ok(res) = client.get(FLARESOLVERR_ENDPOINT).send().await {
        if res.status().is_success() || res.status().as_u16() == 405 || res.status().as_u16() == 400 {
            return Ok(());
        }
    }

    let container_cmd = detect_container_cmd()?;
    println!("[-] Launching {} container for FlareSolverr...", container_cmd);

    if container_cmd == "podman" {
        let _ = Command::new("podman")
            .args(["machine", "start"])
            .output();
    }

    let start_output = Command::new(container_cmd)
        .args(["start", CONTAINER_NAME])
        .output();

    if let Ok(out) = start_output {
        if out.status.success() {
            return wait_for_flaresolverr(client).await;
        }
    }

    let run_output = Command::new(container_cmd)
        .args([
            "run",
            "-d",
            "--name",
            CONTAINER_NAME,
            "-p",
            "127.0.0.1:8191:8191",
            FLARESOLVERR_IMAGE,
        ])
        .output()?;

    if !run_output.status.success() {
        let err = String::from_utf8_lossy(&run_output.stderr);
        anyhow::bail!("Failed to start {} container: {}", container_cmd, err);
    }

    wait_for_flaresolverr(client).await
}

async fn wait_for_flaresolverr(client: &reqwest::Client) -> Result<()> {
    for _ in 0..30 {
        if let Ok(res) = client.get(FLARESOLVERR_ENDPOINT).send().await {
            if res.status().is_success() || res.status().as_u16() == 405 || res.status().as_u16() == 400 {
                return Ok(());
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
    anyhow::bail!("Timed out waiting for FlareSolverr container on port 8191")
}

pub async fn fetch_via_flaresolverr(client: &reqwest::Client, url: &str) -> Result<FlareSolverrResult> {
    ensure_flaresolverr_running(client).await?;

    let payload = FlareSolverrRequest {
        cmd: "request.get".to_string(),
        url: url.to_string(),
        max_timeout: 60000,
    };

    let res = client
        .post(FLARESOLVERR_ENDPOINT)
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!("FlareSolverr endpoint returned status {}", res.status());
    }

    let resp_data: FlareSolverrResponse = res.json().await?;

    if resp_data.status != "ok" {
        let msg = resp_data.message.unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!("FlareSolverr challenge solving failed: {}", msg);
    }

    let solution = resp_data
        .solution
        .ok_or_else(|| anyhow!("FlareSolverr returned ok status but missing solution body"))?;

    let user_agent = solution
        .user_agent
        .unwrap_or_else(|| "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36".to_string());

    let mut cookie_pairs = Vec::new();
    if let Some(cookies) = solution.cookies {
        for c in cookies {
            cookie_pairs.push(format!("{}={}", c.name, c.value));
        }
    }

    let cookie_header = cookie_pairs.join("; ");

    Ok(FlareSolverrResult {
        html: solution.response,
        user_agent,
        cookie_header,
    })
}

pub async fn fetch_with_cf_cache(
    client: &reqwest::Client,
    url: &str,
) -> Result<(String, HashMap<String, String>)> {
    let domain = crate::utils::get_hostname(url);

    // 1. Check SESSION_CACHE first
    {
        let cache = SESSION_CACHE.lock().await;
        if let Some((cached_ua, cached_cookie)) = cache.get(&domain) {
            let mut req = client.get(url);
            if !cached_ua.is_empty() {
                req = req.header("User-Agent", cached_ua);
            }
            if !cached_cookie.is_empty() {
                req = req.header("Cookie", cached_cookie);
            }
            if let Ok(res) = req.send().await {
                if res.status().is_success() {
                    let html = res.text().await?;
                    let mut extra = HashMap::new();
                    if !cached_ua.is_empty() {
                        extra.insert("User-Agent".to_string(), cached_ua.clone());
                    }
                    if !cached_cookie.is_empty() {
                        extra.insert("Cookie".to_string(), cached_cookie.clone());
                    }
                    return Ok((html, extra));
                }
            }
        }
    }

    // 2. Cache miss or session expired -> call FlareSolverr once
    let fs_res = fetch_via_flaresolverr(client, url).await?;

    // 3. Cache the new session credentials
    {
        let mut cache = SESSION_CACHE.lock().await;
        cache.insert(domain, (fs_res.user_agent.clone(), fs_res.cookie_header.clone()));
    }

    let mut extra = HashMap::new();
    if !fs_res.user_agent.is_empty() {
        extra.insert("User-Agent".to_string(), fs_res.user_agent);
    }
    if !fs_res.cookie_header.is_empty() {
        extra.insert("Cookie".to_string(), fs_res.cookie_header);
    }

    Ok((fs_res.html, extra))
}

pub async fn get_cached_cf_headers(url: &str) -> HashMap<String, String> {
    let domain = crate::utils::get_hostname(url);
    let mut extra = HashMap::new();
    let cache = SESSION_CACHE.lock().await;
    if let Some((ua, cookie)) = cache.get(&domain) {
        if !ua.is_empty() {
            extra.insert("User-Agent".to_string(), ua.clone());
        }
        if !cookie.is_empty() {
            extra.insert("Cookie".to_string(), cookie.clone());
        }
    }
    extra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_container_cmd() {
        let cmd = detect_container_cmd();
        assert!(cmd.is_ok(), "Host should have podman or docker installed");
    }
}
