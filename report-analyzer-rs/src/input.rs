use crate::types::{Args, Report, Ontology};
use std::fs;
// use std::path::PathBuf; // Removed unused import
use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn load_data(args: &Args) -> Result<(Report, Option<Ontology>), Box<dyn std::error::Error>> {
    let report_content = fs::read_to_string(&args.report_path)?;
    let report: Report = serde_json::from_str(&report_content)?;

    let ontology: Option<Ontology> = if let Some(path) = &args.ontology_path {
        let ontology_content = fs::read_to_string(path)?;
        Some(serde_json::from_str(&ontology_content)?)
    } else {
        None
    };

    Ok((report, ontology))
}
