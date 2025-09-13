use clap::Parser;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use url::Url;
use sanitize_filename::sanitize;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Paths to markdown files containing URLs to spider
    #[arg(short, long, value_parser, num_args = 1.., conflicts_with = "url_file")]
    markdown_files: Vec<PathBuf>,

    /// Path to a file containing URLs to spider (one URL per line)
    #[arg(long)]
    url_file: Option<PathBuf>,

    /// Output directory for the corpus
    #[arg(short, long, default_value = "corpus/web_sources")]
    output_dir: PathBuf,
}

fn extract_urls_from_markdown(file_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut urls = Vec::new();

    // Regex to find URLs in Markdown reference links: [text](url) or raw URLs
    let re = Regex::new(r"(?i)\[[^\]]+\]\((https?://[^)]+\.[a-z]{2,6}(?:/[^)]*)?)\)|(https?://[^\s)]+\.[a-z]{2,6}(?:/[^\s)]*)?)")?;

    for cap in re.captures_iter(&content) {
        if let Some(url_match) = cap.get(1).or_else(|| cap.get(2)) {
            urls.push(url_match.as_str().to_string());
        }
    }
    Ok(urls)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut all_urls: Vec<String> = Vec::new();
    if let Some(url_file_path) = args.url_file {
        println!("Reading URLs from: {}", url_file_path.display());
        let content = fs::read_to_string(&url_file_path)?;
        all_urls.extend(content.lines().map(|s| s.to_string()));
    } else {
        for md_file in &args.markdown_files {
            println!("Extracting URLs from: {}", md_file.display());
            let extracted = extract_urls_from_markdown(md_file)?;
            all_urls.extend(extracted);
        }
    }

    fs::create_dir_all(&args.output_dir)?;

    let client = Client::new();

    for url_str in all_urls {
        println!("Fetching: {}", url_str);
        let url = Url::parse(&url_str)?;
        let response = client.get(&url_str).send()?;

        if response.status().is_success() {
            let content_type = response.headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok());

            let file_name = sanitize(url.host_str().unwrap_or("unknown").to_string() + &url.path().replace('/', "_")) + ".txt";
            let output_path = args.output_dir.join(file_name);

            if let Some(ct) = content_type {
                if ct.contains("text/html") {
                    let html_content = response.text()?;
                    let document = Html::parse_document(&html_content);
                    let selector = Selector::parse("p, h1").unwrap(); // Extract text from paragraph and heading tags
                    let text_content: String = document.select(&selector)
                        .map(|element| element.text().collect::<String>())
                        .collect::<Vec<String>>()
                        .join("\n");
                    fs::write(&output_path, text_content)?;
                    println!("Successfully wrote HTML content to: {}", output_path.display());
                } else if ct.contains("application/pdf") {
                    // Handle PDF by just noting it, as direct text extraction is complex
                    println!("  Skipping PDF: {}", url_str);
                    fs::write(&output_path, "PDF content from: ".to_string() + &url_str)?;
                } else {
                    // For other content types, just save raw bytes if desired, or skip
                    println!("  Skipping unsupported content type ({}) : {}", ct, url_str);
                    fs::write(&output_path, format!("Unsupported content type ({}) from: {}", ct, url_str))?;
                }
            } else {
                println!("  No content type, skipping: {}", url_str);
                fs::write(&output_path, format!("No content type from: {}", url_str))?;
            }
        } else {
            eprintln!("Failed to fetch {}: Status {}", url_str, response.status());
            println!("DEBUG: Fetch failed for {}: Status {}", url_str, response.status());
        }
        std::thread::sleep(std::time::Duration::from_secs(1)); // Be polite
    }

    println!("Corpus building complete in {}", args.output_dir.display());
    Ok(())
}