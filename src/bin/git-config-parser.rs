use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .git/config file
    #[arg(long)]
    git_config: Option<PathBuf>,

    /// Path to the .gitmodules file
    #[arg(long)]
    git_modules: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitConfig {
    #[serde(default)]
    sections: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitModules {
    #[serde(default)]
    submodule: HashMap<String, SubmoduleConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubmoduleConfig {
    path: String,
    url: String,
    #[serde(default)]
    branch: Option<String>,
}

fn parse_git_config(content: &str) -> GitConfig {
    let mut config = GitConfig {
        sections: HashMap::new(),
    };
    let mut current_section: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line[1..line.len() - 1].to_string();
            current_section = Some(section_name.clone());
            config.sections.insert(section_name, HashMap::new());
        } else if let Some(section) = &current_section {
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                if let Some(section_map) = config.sections.get_mut(section) {
                    section_map.insert(key, value);
                }
            }
        }
    }
    config
}

fn parse_git_modules(content: &str) -> GitModules {
    let mut modules = GitModules {
        submodule: HashMap::new(),
    };
    let mut current_submodule_name: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line[1..line.len() - 1].to_string();
            if section_name.starts_with("submodule \"") && section_name.ends_with('"') {
                let submodule_name = section_name["submodule \"".len()..section_name.len() - 1].to_string();
                current_submodule_name = Some(submodule_name.clone());
                modules.submodule.insert(
                    submodule_name,
                    SubmoduleConfig {
                        path: String::new(),
                        url: String::new(),
                        branch: None,
                    },
                );
            } else {
                current_submodule_name = None; // Not a submodule section
            }
        } else if let Some(submodule_name) = &current_submodule_name {
            if let Some(submodule_config) = modules.submodule.get_mut(submodule_name) {
                if let Some(eq_pos) = line.find('=') {
                    let key = line[..eq_pos].trim();
                    let value = line[eq_pos + 1..].trim().to_string();
                    match key {
                        "path" => submodule_config.path = value,
                        "url" => submodule_config.url = value,
                        "branch" => submodule_config.branch = Some(value),
                        _ => {} // Ignore other keys for now
                    }
                }
            }
        }
    }
    modules
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut output = HashMap::new();

    if let Some(git_config_path) = args.git_config {
        let content = fs::read_to_string(&git_config_path)?;
        let parsed_config = parse_git_config(&content);
        output.insert("git_config".to_string(), serde_json::to_value(parsed_config)?);
    }

    if let Some(git_modules_path) = args.git_modules {
        let content = fs::read_to_string(&git_modules_path)?;
        let parsed_modules = parse_git_modules(&content);
        output.insert("git_modules".to_string(), serde_json::to_value(parsed_modules)?);
    }

    let json_output = serde_json::to_string_pretty(&output)?;
    println!("{}", json_output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_config_empty() {
        let content = "";
        let config = parse_git_config(content);
        assert!(config.sections.is_empty());
    }

    #[test]
    fn test_parse_git_config_only_sections() {
        let content = "[core]\n[remote \"origin\"]";
        let config = parse_git_config(content);
        assert!(config.sections.contains_key("core"));
        assert!(config.sections.contains_key("remote \"origin\""));
        assert!(config.sections["core"].is_empty());
        assert!(config.sections["remote \"origin\""].is_empty());
    }

    #[test]
    fn test_parse_git_config_with_comments_and_empty_lines() {
        let content = r###"# This is a comment
[core]
    repositoryformatversion = 0

# Another comment
[user]
    name = John Doe
    email = john.doe@example.com
"###;
        let config = parse_git_config(content);

        assert!(config.sections.contains_key("core"));
        assert_eq!(config.sections["core"]["repositoryformatversion"], "0");

        assert!(config.sections.contains_key("user"));
        assert_eq!(config.sections["user"]["name"], "John Doe");
        assert_eq!(config.sections["user"]["email"], "john.doe@example.com");
    }

    #[test]
    fn test_parse_git_config_multiple_sections() {
        let content = r###"[section1]
    key1 = value1
    key2 = value2
[section2]
    keyA = valueA
[section3]
    keyB = valueB
    keyC = valueC
"###;
        let config = parse_git_config(content);

        assert_eq!(config.sections.len(), 3);

        assert_eq!(config.sections["section1"]["key1"], "value1");
        assert_eq!(config.sections["section1"]["key2"], "value2");

        assert_eq!(config.sections["section2"]["keyA"], "valueA");

        assert_eq!(config.sections["section3"]["keyB"], "valueB");
        assert_eq!(config.sections["section3"]["keyC"], "valueC");
    }

    #[test]
    fn test_parse_git_config_basic() {
        let content = r###"[core]
    repositoryformatversion = 0
[remote "origin"]
    url = https://github.com/example/repo.git
    fetch = +refs/heads/*:refs/remotes/origin/*"###;
        let config = parse_git_config(content);

        assert!(config.sections.contains_key("core"));
        assert_eq!(config.sections["core"]["repositoryformatversion"], "0");

        assert!(config.sections.contains_key("remote \"origin\""));
        assert_eq!(config.sections["remote \"origin\""]["url"], "https://github.com/example/repo.git");
        assert_eq!(config.sections["remote \"origin\""]["fetch"], "+refs/heads/*:refs/remotes/origin/*");
    }

    #[test]
    fn test_parse_git_modules_empty() {
        let content = "";
        let modules = parse_git_modules(content);
        assert!(modules.submodule.is_empty());
    }

    #[test]
    fn test_parse_git_modules_single_submodule() {
        let content = String::from("[submodule \"sub1\"]\n    path = path/to/sub1\n    url = https://github.com/example/sub1.git\n");
        let modules = parse_git_modules(&content);
        assert!(modules.submodule.contains_key("sub1"));
        let sub1 = &modules.submodule["sub1"];
        assert_eq!(sub1.path, "path/to/sub1");
        assert_eq!(sub1.url, "https://github.com/example/sub1.git");
        assert!(sub1.branch.is_none());
    }

    #[test]
    fn test_parse_git_modules_multiple_submodules() {
        let content = String::from("[submodule \"sub1\"]\n    path = path/to/sub1\n    url = https://github.com/example/sub1.git\n[submodule \"sub2\"]\n    path = path/to/sub2\n    url = https://github.com/example/sub2.git\n    branch = main\n");
        let modules = parse_git_modules(&content);
        assert_eq!(modules.submodule.len(), 2);

        let sub1 = &modules.submodule["sub1"];
        assert_eq!(sub1.path, "path/to/sub1");
        assert_eq!(sub1.url, "https://github.com/example/sub1.git");
        assert!(sub1.branch.is_none());

        let sub2 = &modules.submodule["sub2"];
        assert_eq!(sub2.path, "path/to/sub2");
        assert_eq!(sub2.url, "https://github.com/example/sub2.git");
        assert_eq!(sub2.branch, Some("main".to_string()));
    }

    #[test]
    fn test_parse_git_modules_with_branch() {
        let content = String::from("[submodule \"sub3\"]\n    path = path/to/sub3\n    url = https://github.com/example/sub3.git\n    branch = dev\n");
        let modules = parse_git_modules(&content);
        assert!(modules.submodule.contains_key("sub3"));
        let sub3 = &modules.submodule["sub3"];
        assert_eq!(sub3.path, "path/to/sub3");
        assert_eq!(sub3.url, "https://github.com/example/sub3.git");
        assert_eq!(sub3.branch, Some("dev".to_string()));
    }
}
