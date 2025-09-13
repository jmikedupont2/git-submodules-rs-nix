pub fn print_duplicate_urls(repo_urls: &HashMap<String, Vec<String>>) {
    let mut duplicate_urls_found = false;
    println!("\n--- Duplicate Repository URLs ---");
    for (url, paths) in repo_urls {
        if paths.len() > 1 {
            duplicate_urls_found = true;
            println!("URL: {}", url);
            for path in paths {
                println!("  - {}", path);
            }
        }
    }
    if !duplicate_urls_found {
        println!("No Duplicate Repository URLs Found ");
    }
}

pub fn analyze_duplicate_urls(report: &Report) -> HashMap<String, Vec<String>> {
    let mut repo_urls: HashMap<String, Vec<String>> = HashMap::new();
    for (url, info) in &report.repositories {
        repo_urls.entry(url.clone()).or_default().push(info.path.clone());
    }
    repo_urls
}

