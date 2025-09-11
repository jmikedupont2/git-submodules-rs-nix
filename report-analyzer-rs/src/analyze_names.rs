pub fn analyze_names(report: &Report) -> HashMap<String, usize> {
    let mut all_names: Vec<String> = Vec::new();
    let re_repo_name = Regex::new(r"https://github.com/[^/]+/([^/]+).*?").unwrap();

    for (url, info) in &report.repositories {
        // Add repository name from URL
        if let Some(captures) = re_repo_name.captures(url) {
            all_names.push(captures[1].replace(".git", ""));
        }
        
        // Add submodule names
        for submodule in &info.submodules {
            all_names.push(submodule.name.clone());
            if let Some(nested_repo) = &submodule.nested_repo {
                if let Some(captures) = re_repo_name.captures(&nested_repo.url) {
                    all_names.push(captures[1].replace(".git", ""));
                }
            }
        }
    }

    let mut name_counts: HashMap<String, usize> = HashMap::new();
    if !all_names.is_empty() {
        for name in all_names {
            *name_counts.entry(name).or_insert(0) += 1;
        }
    }
    name_counts
}

