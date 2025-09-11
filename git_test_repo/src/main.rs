use git2::Repository;
use std::env;

fn main() -> Result<(), git2::Error> {
    let repo_path = env::current_dir().unwrap();
    println!("Attempting to open repository at: {:?}", repo_path);
    let repo = Repository::open(&repo_path)?;
    println!("Successfully opened repository at: {:?}", repo.path());
    Ok(())
}