
pub fn find_longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut lcp = strings[0].clone();
    for s in &strings[1..] {
        let mut new_lcp = String::new();
        for (c1, c2) in lcp.chars().zip(s.chars()) {
            if c1 == c2 {
                new_lcp.push(c1);
            } else {
                break;
            }
        }
        lcp = new_lcp;
        if lcp.is_empty() {
            break;
        }
    }
    lcp
}

pub fn perform_lcp_analysis(report: &Report) -> String {
    let mut all_paths_and_urls: Vec<String> = Vec::new();
    for (url, info) in &report.repositories {
        all_paths_and_urls.push(url.clone());
        all_paths_and_urls.push(info.path.clone());
        for submodule in &info.submodules {
            all_paths_and_urls.push(submodule.url.clone());
            all_paths_and_urls.push(submodule.path.clone());
            if let Some(nested_repo) = &submodule.nested_repo {
                all_paths_and_urls.push(nested_repo.url.clone());
                all_paths_and_urls.push(nested_repo.path.clone());
            }
        }
    }
    for failed_repo in &report.failed_repositories {
        all_paths_and_urls.push(failed_repo.path.clone());
    }
    find_longest_common_prefix(&all_paths_and_urls)
}

pub fn print_lcp_analysis(lcp: &String) {
    if !lcp.is_empty() {
        println!("\n--- Longest Common Prefix (LCP) ---");
        println!("LCP: {}", lcp);
        println!("Paths and URLs below are relative to LCP where applicable.");
    }
}
