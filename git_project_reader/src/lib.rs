//! A library for reading information from Git projects, including tracked files and git status.

//! A library for reading information from Git projects, including tracked files and git status.

use git2::Repository;
use std::path::Path;
use std::process::Command;
use std::io::{self, ErrorKind};
use std::os::unix::ffi::OsStrExt;

/// Represents the collected information about a Git project.
#[derive(Debug, Default)]
pub struct GitProjectInfo {
    /// List of files tracked by Git in the repository.
    pub tracked_files: Vec<String>,
    /// The porcelain output of `git status`.
    pub git_status_porcelain: Option<String>,
}

/// Collects information about a Git project at the given path.
///
/// This function attempts to open the Git repository at `repo_path` and then
/// retrieves the list of tracked files and the porcelain output of `git status`.
///
/// # Arguments
///
/// * `repo_path` - The absolute path to the Git repository's working directory.
///
/// # Returns
///
/// A `Result` containing `GitProjectInfo` if successful, or an `io::Error` if
/// the repository cannot be opened or Git commands fail.
pub fn collect_git_project_info(repo_path: &Path) -> io::Result<GitProjectInfo> {
    let repo = Repository::open(repo_path).map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
    let mut info = GitProjectInfo::default();

    // Get tracked files
    info.tracked_files = get_tracked_files(&repo)?;

    // Get git status
    info.git_status_porcelain = get_git_status(repo_path)?;

    Ok(info)
}

/// Retrieves a list of files tracked by Git in the given repository.
fn get_tracked_files(repo: &Repository) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let index = repo.index().map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

    for entry in index.iter() {
        if let Some(path_str) = Path::new(std::ffi::OsStr::from_bytes(&entry.path)).to_str() {
            files.push(path_str.to_string());
        }
    }
    Ok(files)
}

/// Executes `git status --porcelain` in the given repository path and returns its output.
fn get_git_status(repo_path: &Path) -> io::Result<Option<String>> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(repo_path)
        .output()?;

    if output.status.success() {
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        eprintln!("Error running git status in {}: {}", repo_path.display(), error_message);
        Err(io::Error::new(io::ErrorKind::Other, "Failed to get git status"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::PathBuf;

    // Helper to initialize a git repo in a temp directory
    fn init_repo() -> (PathBuf, Repository) {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        (dir.into_path(), repo)
    }

    #[test]
    fn test_get_tracked_files() {
        let (path, repo) = init_repo();
        let file_path = path.join("test_file.txt");
        fs::write(&file_path, "hello").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test_file.txt")).unwrap();
        index.write().unwrap();

        let tracked_files = get_tracked_files(&repo).unwrap();
        assert_eq!(tracked_files, vec!["test_file.txt"]);
    }

    #[test]
    fn test_get_git_status_clean() {
        let (path, repo) = init_repo();
        let file_path = path.join("test_file.txt");
        fs::write(&file_path, "hello").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test_file.txt")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[]).unwrap();

        let status = get_git_status(&path).unwrap();
        assert_eq!(status, Some("".to_string())); // Clean status is empty string
    }

    #[test]
    fn test_get_git_status_modified() {
        let (path, repo) = init_repo();
        let file_path = path.join("test_file.txt");
        fs::write(&file_path, "hello").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test_file.txt")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[]).unwrap();

        // Modify the file
        fs::write(&file_path, "world").unwrap();

        let status = get_git_status(&path).unwrap();
        assert!(status.unwrap().contains(" M test_file.txt"));
    }

    #[test]
    fn test_get_git_status_untracked() {
        let (path, _repo) = init_repo();
        let file_path = path.join("untracked_file.txt");
        fs::write(&file_path, "untracked").unwrap();

        let status = get_git_status(&path).unwrap();
        assert!(status.unwrap().contains("?? untracked_file.txt"));
    }

    #[test]
    fn test_collect_git_project_info() {
        let (path, repo) = init_repo();
        let file_path = path.join("info_test.txt");
        fs::write(&file_path, "info").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("info_test.txt")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit for info test", &tree, &[]).unwrap();

        let info = collect_git_project_info(&path).unwrap();
        assert_eq!(info.tracked_files, vec!["info_test.txt"]);
        assert_eq!(info.git_status_porcelain, Some("".to_string()));
    }
}
