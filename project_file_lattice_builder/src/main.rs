//! This program conceptually constructs a "lattice of all the files in our project."
//! It simulates the process of identifying, processing, and classifying each file
//! according to the Lattice Idea Framework, treating files as "large objects"
//! that are mapped into the lattice structure.

use std::path::PathBuf;

// --- Simplified Lattice Types (Conceptual) ---
// These are minimal definitions needed for this conceptual demonstration.

/// Represents a boolean predicate associated with a word or concept.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WordPredicate(bool);

impl WordPredicate {
    /// Creates a new `WordPredicate` with the given boolean value.
    fn new(value: bool) -> Self { WordPredicate(value) }
}

/// Represents a classified instance (e.g., a file) within the conceptual lattice.
#[derive(Debug, Clone)]
pub struct Instance {
    /// A unique identifier for the instance (e.g., file path).
    pub id: String,
    /// A list of `WordPredicate`s extracted for this instance.
    pub predicates: Vec<WordPredicate>,
    /// The conceptual path in the lattice hierarchy where this instance is classified.
    pub classification_path: String, // Conceptual path in the lattice hierarchy
}

impl Instance {
    /// Creates a new `Instance` with the specified ID, predicates, and classification path.
    pub fn new(id: &str, predicates: Vec<WordPredicate>, classification_path: &str) -> Self {
        Self { id: id.to_string(), predicates, classification_path: classification_path.to_string() }
    }
}

/// Represents a simple classifier based on predicate presence.
struct PredicateClassifier {
    target_predicates: Vec<String>,
}

impl PredicateClassifier {
    /// Creates a new `PredicateClassifier` with a list of target predicates.
    ///
    /// All target predicates are converted to lowercase for case-insensitive matching.
    fn new(predicates: Vec<&str>) -> Self {
        Self { target_predicates: predicates.into_iter().map(|s| s.to_lowercase()).collect() }
    }

    /// Extracts WordPredicates from text based on the classifier's target predicates.
    fn extract_word_predicates(&self, text: &str) -> Vec<WordPredicate> {
        let lower_text = text.to_lowercase();
        self.target_predicates.iter()
            .map(|p| WordPredicate(lower_text.contains(p)))
            .collect()
    }
}

// --- Main Logic ---

/// Main entry point for the `project_file_lattice_builder` application.
///
/// This program conceptually constructs a "lattice of all the files in our project."
/// It simulates the process of identifying, processing, and classifying each file
/// according to the Lattice Idea Framework, treating files as "large objects"
/// that are mapped into the lattice structure.
///
/// It scans the current directory, extracts conceptual content from files,
/// applies word predicates, and classifies them into a simplified lattice hierarchy.
///
/// # Returns
///
/// `Ok(())` if the operation completes successfully, otherwise an `Err` containing
/// a boxed error.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Project File Lattice Builder ---");

    let project_root = PathBuf::from("."); // Current directory as project root

    // Define a global set of predicates for file classification
    let global_predicates = vec![
        "rust", "python", "javascript", "java", "c", "cpp",
        "toml", "md", "tex", "rs", "py", "js", "java",
        "config", "test", "doc", "example", "src", "bin", "lib",
        "lattice", "meme", "vibe", "math", "llm", "compiler", "submodule"
    ];
    let classifier = PredicateClassifier::new(global_predicates.iter().map(|&s| s).collect());

    println!("\nScanning project files and building conceptual lattice...");
    let mut classified_files: Vec<Instance> = Vec::new();

    // Recursively walk the directory
    for entry in walkdir::WalkDir::new(&project_root) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let file_extension = path.extension().unwrap_or_default().to_string_lossy().to_string();
            let file_path_str = path.to_string_lossy().to_string();

            // Conceptual content extraction (simplified: just use path and extension)
            let conceptual_content = format!("{} {}", file_path_str, file_extension);
            let predicates = classifier.extract_word_predicates(&conceptual_content);

            // Conceptual classification into a lattice path
            // This is a very simplified mapping. In a real system, this would be complex
            // based on predicate analysis and the generated lattice structure.
            let classification_path = if file_extension == "rs" {
                if file_path_str.contains("src/lattice") {
                    "layer_k_2/rust_lattice_code".to_string()
                } else if file_path_str.contains("src/") {
                    "layer_k_2/rust_source_code".to_string()
                } else {
                    "layer_k_2/rust_misc".to_string()
                }
            } else if file_extension == "md" {
                if file_path_str.contains("docs/crq/") {
                    "layer_k_3/crq_documentation".to_string()
                } else if file_path_str.contains("docs/memes/") {
                    "layer_k_3/meme_documentation".to_string()
                } else {
                    "layer_k_3/general_documentation".to_string()
                }
            } else if file_extension == "toml" {
                "layer_k_2/config_files".to_string()
            } else {
                "layer_k_2/other_files".to_string()
            };

            let instance = Instance::new(&file_path_str, predicates.clone(), &classification_path);
            classified_files.push(instance);

            println!("  File: '{}'\n    Predicates: {:?}\n    Classified to: {}", file_name, predicates, classification_path);
        }
    }

    println!("\n--- Conceptual Lattice of Project Files Concluded ---");
    println!("Total files conceptually classified: {}", classified_files.len());
    println!("This demonstrates how each file, as a large object, can be mapped into the lattice structure.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_predicate_new() {
        let p_true = WordPredicate::new(true);
        assert_eq!(p_true.0, true);

        let p_false = WordPredicate::new(false);
        assert_eq!(p_false.0, false);
    }

    #[test]
    fn test_instance_new() {
        let predicates = vec![WordPredicate::new(true), WordPredicate::new(false)];
        let instance = Instance::new("test_id", predicates.clone(), "test/path");

        assert_eq!(instance.id, "test_id");
        assert_eq!(instance.predicates.len(), 2);
        assert_eq!(instance.predicates[0].0, true); // rust
        assert_eq!(instance.predicates[1].0, false); // test
        assert_eq!(instance.classification_path, "test/path");
    }

    #[test]
    fn test_predicate_classifier_new() {
        let classifier = PredicateClassifier::new(vec!["Rust", "PYTHON", "Js"]);
        assert_eq!(classifier.target_predicates, vec!["rust", "python", "js"])
    }

    #[test]
    fn test_extract_word_predicates() {
        let classifier = PredicateClassifier::new(vec!["rust", "test", "config"]);

        let text1 = "This is a Rust project with some tests.";
        let predicates1 = classifier.extract_word_predicates(text1);
        assert_eq!(predicates1.len(), 3);
        assert_eq!(predicates1[0].0, true); // rust
        assert_eq!(predicates1[1].0, true); // test
        assert_eq!(predicates1[2].0, false); // config

        let text2 = "Just a plain text file.";
        let predicates2 = classifier.extract_word_predicates(text2);
        assert_eq!(predicates2.len(), 3);
        assert_eq!(predicates2[0].0, false);
        assert_eq!(predicates2[1].0, false);
        assert_eq!(predicates2[2].0, false);

        let text3 = "Configuration for the project.";
        let predicates3 = classifier.extract_word_predicates(text3);
        assert_eq!(predicates3.len(), 3);
        assert_eq!(predicates3[0].0, false);
        assert_eq!(predicates3[1].0, false);
        assert_eq!(predicates3[2].0, true); // config
    }
}