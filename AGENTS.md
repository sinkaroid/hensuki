# Project Rules & Agent Guidelines: `hensuki`

This document defines the strict rules, architectural constraints, and behavioral requirements for any AI coding assistant working on the **hensuki** codebase.

---

## 1. Project Overview & Mandate
- **Project Name**: `hensuki`
- **Language & Edition**: Rust (Edition 2024)
- **Domain**: High-performance, asynchronous, concurrent imageboard scraper & bulk media downloader.
- **Repository Root**: `e:\Ferris\hensuki`

---

## 2. Strict Constraints & Non-Negotiables

### A. Zero Breaking Changes (Strict Behavioral Parity)
- All user-facing behavior, CLI prompts, input options, timeout duration (30 seconds), log output formats, filename extraction rules, and URL validation logic **MUST** strictly match the legacy Python behavior.
- Interactive Prompt Sequence:
  1. `[+] Please enter file you want to bulk download (e.g. file.txt): `
     - Automatically appends `.txt` if missing.
     - Timeout: 30 seconds -> Prints `Timeout occurred, kindly read the docs: https://github.com/sinkaroid/hensuki#usage` and exits cleanly.
  2. `[+] Please choose (1) Multiple posts or (2) Multiple pages: `
     - Accepts: `1`, `2`, `posts`, `post`, `pages`, `page`.
  3. `[+] Select type image results (1) Original size or (2) Smaller size: `
     - Accepts: `1`, `2`.

### B. Rust Stack & Architecture
- **Edition**: `2024`
- **Async Runtime**: `tokio` (multi-threaded async worker pool)
- **HTTP Client**: `reqwest` with native TLS/rustls and response streaming.
- **HTML Scraping & URL Extraction**: `scraper` CSS selectors + `regex`.
- **Progress Tracking**: `indicatif` progress bars for concurrent downloads.
- **Error Handling**: `anyhow::Result` with clear error context, no raw `panic!`.

### C. File System & Git Rules
- `/legacy/` directory is ignored and must **NEVER** be modified, deleted, or committed.
- All new source code must be placed in `src/`.
- File downloads must use async streaming (`tokio::io::copy`) to maintain near-zero RAM footprint.

---

## 3. Supported Booru Endpoints
The application must maintain compatibility with all supported booru engines:
- Gelbooru, Safebooru, Danbooru, Rule34, Tbib, Xbooru, Realbooru, Yandere, Konachan, Hypnohub, E621, E926.

---

## 4. Verification Requirements
- Every edit or addition must build cleanly with `cargo check` and `cargo clippy`.
- Automated unit tests (`cargo test`) must pass for URL validation and filename extraction.
