pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod lattice_model;

use lattice_model::{Lattice, LatticeLayer, Instance, PredicateClassifier, ValueType};

pub fn build_zos_lattice(files: &[(String, String, String)]) -> Lattice {
    let mut lattice = Lattice::new("ZOS Project Lattice");

    let global_predicates = vec![
        "rust", "python", "javascript", "java", "c", "cpp",
        "toml", "md", "tex", "rs", "py", "js", "java",
        "config", "test", "doc", "example", "src", "bin", "lib",
        "lattice", "meme", "vibe", "math", "llm", "compiler", "submodule"
    ];
    let classifier = PredicateClassifier::new(global_predicates.iter().map(|&s| s).collect());

    // Conceptual layers based on file types
    let mut rust_lattice_code_layer = LatticeLayer::<bool>::new(ValueType::Bit);
    let mut rust_source_code_layer = LatticeLayer::<bool>::new(ValueType::Bit);
    let mut rust_misc_layer = LatticeLayer::<bool>::new(ValueType::Bit);
    let mut config_files_layer = LatticeLayer::<bool>::new(ValueType::Bit);
    let mut other_files_layer = LatticeLayer::<bool>::new(ValueType::Bit);

    let mut crq_documentation_layer = LatticeLayer::<bool>::new(ValueType::ThreeValue);
    let mut meme_documentation_layer = LatticeLayer::<bool>::new(ValueType::ThreeValue);
    let mut general_documentation_layer = LatticeLayer::<bool>::new(ValueType::ThreeValue);

    for (file_path_str, file_extension, conceptual_content) in files {
        let predicates = classifier.extract_word_predicates(conceptual_content);

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

        let instance = Instance::new(file_path_str, predicates.len() as u8, predicates.into_iter().map(|wp| wp.0).collect());

        match classification_path.as_str() {
            "layer_k_2/rust_lattice_code" => rust_lattice_code_layer.add_instance(instance),
            "layer_k_2/rust_source_code" => rust_source_code_layer.add_instance(instance),
            "layer_k_2/rust_misc" => rust_misc_layer.add_instance(instance),
            "layer_k_2/config_files" => config_files_layer.add_instance(instance),
            "layer_k_2/other_files" => other_files_layer.add_instance(instance),
            "layer_k_3/crq_documentation" => crq_documentation_layer.add_instance(instance),
            "layer_k_3/meme_documentation" => meme_documentation_layer.add_instance(instance),
            "layer_k_3/general_documentation" => general_documentation_layer.add_instance(instance),
            _ => {},
        }
    }

    lattice.add_layer(rust_lattice_code_layer);
    lattice.add_layer(rust_source_code_layer);
    lattice.add_layer(rust_misc_layer);
    lattice.add_layer(config_files_layer);
    lattice.add_layer(other_files_layer);
    lattice.add_layer(crq_documentation_layer);
    lattice.add_layer(meme_documentation_layer);
    lattice.add_layer(general_documentation_layer);

    lattice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
