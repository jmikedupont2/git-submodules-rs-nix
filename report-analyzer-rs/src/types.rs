use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use clap::Parser; // Removed 'command' and 'arg' as they are not types

#[derive(Serialize, Deserialize)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_repo: Option<RepoInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct RepoInfo {
    pub path: String,
    pub url: String,
    pub submodules: Vec<SubmoduleInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FailedRepoInfo {
    pub path: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub repositories: HashMap<String, RepoInfo>,
    #[serde(default)]
    pub failed_repositories: Vec<FailedRepoInfo>,
}

#[derive(Parser)] // Added Parser derive
pub struct Args {
    /// Path to the submodule report JSON file
    #[arg(long)]
    pub report_path: String,
    /// Optional: Path to an ontology JSON file for emoji mapping
    #[arg(long)]
    pub ontology_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)] // Added Serialize, Deserialize derive
pub struct Ontology(pub HashMap<String, String>);