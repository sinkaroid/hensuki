use chrono::Local;
use std::time::Instant;

pub fn log_data(case: &str, note: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("{} - INFO - {} {}", now, case, note);
}

pub fn log_time(start: Instant) {
    let elapsed = start.elapsed();
    let total_secs = elapsed.as_secs();
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!(
        "{} - INFO - Task finished took {} min, {} sec",
        now, mins, secs
    );
}

pub fn get_hostname(url: &str) -> String {
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() >= 3 {
        format!("https://{}", parts[2])
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hostname() {
        assert_eq!(
            get_hostname("https://safebooru.org/index.php?page=post"),
            "https://safebooru.org"
        );
    }
}
