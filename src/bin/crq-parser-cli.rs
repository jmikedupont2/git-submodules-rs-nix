use clap::Parser;
use std::fs;
use std::path::PathBuf;
use submodules::crq_parser::CRQ;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the CRQ markdown file
    #[clap(short, long, value_parser)]
    file: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let file_path = args.file;
    let crq_content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

    let file_name = file_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("Could not get file name from {}", file_path.display()))?;

    // Extract CRQ ID from filename (e.g., CRQ-003-...) -> CRQ-003
    let crq_id = file_name.splitn(2, '-').take(2).collect::<Vec<&str>>().join("-");

    let crq = CRQ::parse_crq_markdown(&crq_content, &crq_id)?;

    println!("CRQ ID: {}", crq.id);
    println!("Title: {}", crq.title);
    println!("Objective: {}", crq.objective);
    println!("Description: {}", crq.description);
    println!("Expected Outcome: {}", crq.expected_outcome);
    println!("Justification/Benefit: {}", crq.justification_benefit);
    if let Some(deps) = crq.dependencies {
        println!("Dependencies: {}", deps);
    }
    if let Some(progress) = crq.partial_progress_learnings {
        println!("Partial Progress/Learnings: {}", progress);
    }

    Ok(())
}
