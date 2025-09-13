pub fn analyze_organizations(report: &Report) -> HashMap<String, usize> {
    let mut organizations: Vec<String> = Vec::new();
    let re_org = Regex::new(r"https://github.com/([^/]+)/.*" ).unwrap();

    for url in report.repositories.keys() {
        if let Some(captures) = re_org.captures(url) {
            organizations.push(captures[1].to_string());
        }
    }
    
    for failed_repo in &report.failed_repositories {
        if let Some(captures) = re_org.captures(&failed_repo.error) {
            organizations.push(captures[1].to_string());
        }
    }

    let mut org_counts: HashMap<String, usize> = HashMap::new();
    if !organizations.is_empty() {
        for org in organizations {
            *org_counts.entry(org).or_insert(0) += 1;
        }
    }
    org_counts
}


