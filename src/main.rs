use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::process::exit;
use std::time::Instant;

use hensuki::client::{download_from_multiple_pages, download_from_multiple_posts};
use hensuki::utils::log_time;

#[derive(Parser, Debug)]
#[command(name = "hensuki")]
#[command(author = "sinkaroid <hey@sinkaroid.org>")]
#[command(version = "3.0.2")]
#[command(about = "A fast, concurrent imageboard scraper and bulk media downloader", long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    mode: Option<String>,

    #[arg(short, long)]
    select_type: Option<String>,
}

async fn read_prompt(prompt_str: &str, timeout_secs: u64) -> String {
    print!("{}", prompt_str);
    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Flush error: {}", e);
    }

    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let _ = tx.send(input.trim().to_string());
        }
    });

    match tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), rx).await {
        Ok(Ok(val)) => val,
        _ => {
            println!("Timeout occurred, kindly read the docs: https://github.com/sinkaroid/hensuki#usage");
            exit(0);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut file = match cli.file {
        Some(f) => f,
        None => {
            read_prompt("[+] Please enter file you want to bulk download (e.g. file.txt): ", 30).await
        }
    };

    if !file.ends_with(".txt") {
        file.push_str(".txt");
    }

    let download_by = match cli.mode {
        Some(m) => m,
        None => {
            read_prompt("[+] Please choose (1) Multiple posts or (2) Multiple pages: ", 30).await
        }
    };

    let select_type = match cli.select_type {
        Some(t) => t,
        None => {
            read_prompt("[+] Select type image results (1) Original size or (2) Smaller size: ", 30).await
        }
    };

    let start = Instant::now();

    match download_by.to_lowercase().as_str() {
        "1" | "posts" | "post" => {
            if let Err(e) = download_from_multiple_posts(&file, &select_type).await {
                eprintln!("Error: {}", e);
            } else {
                log_time(start);
            }
        }
        "2" | "pages" | "page" => {
            if let Err(e) = download_from_multiple_pages(&file, &select_type).await {
                eprintln!("Error: {}", e);
            } else {
                log_time(start);
            }
        }
        _ => {
            println!("Invalid request");
            exit(0);
        }
    }

    Ok(())
}
