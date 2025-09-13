mod types;
mod input;
// mod processing; // No longer needed as it's empty
mod analyze_strings; // Renamed from new_processing to reflect its new purpose
mod apply_emojis; // Declaring apply_emojis as a module
use types::{Args, Report, Ontology};
// use input::{parse_args, load_data}; // parse_args is used via Args::parse(), load_data is not used
use std::fs;
// use regex::Regex; // No longer directly used in main.rs
// use std::collections::HashMap; // No longer directly used in main.rs
use clap::Parser; // Added for Args::parse()

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let report_content = fs::read_to_string(&args.report_path)?;
    let report: Report = serde_json::from_str(&report_content)?;

    let ontology: Option<Ontology> = if let Some(path) = &args.ontology_path {
        let ontology_content = fs::read_to_string(path)?;
        Some(serde_json::from_str(&ontology_content)?)
    } else {
        None
    };

    let successful_repos = report.repositories.len();
    let failed_repos = report.failed_repositories.len();

    println!("--- Submodule Report Analysis ---");
    println!("Total successful repositories: {}", successful_repos);
    println!("Total failed repositories: {}", failed_repos);

    // Commenting out calls to missing functions
    // let lcp = processing::perform_lcp_analysis(&report);
    // processing::print_lcp_analysis(&lcp);

    // let repo_urls = processing::analyze_duplicate_urls(&report);
    // processing::print_duplicate_urls(&repo_urls);

    // let org_counts = processing::analyze_organizations(&report);
    // processing::print_organizations_analysis(&org_counts, &ontology);

    // let name_counts = processing::analyze_names(&report);
    // processing::print_names_analysis(&name_counts, &ontology);

    let suggested_rules = analyze_strings::analyze_strings(&report, &ontology)?;
    analyze_strings::print_suggested_rules_with_emojis(&suggested_rules, &ontology);

    Ok(())
}