//! This program demonstrates how to use the conceptual `lattice_types` definitions
//! to represent and analyze "words as predicates" from a given text.
//! It applies basic n-gram analysis on these predicate representations.

use crate::lattice_types::{ValueType, Instance, LatticeLayer, Lattice, HasValueCount, LatticeLayerTrait};
use std::collections::HashMap;

// --- 1. Define a simple WordPredicate type ---
// In Model 1 (2-value type), a word is either present (1) or absent (0).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WordPredicate(bool);

impl HasValueCount for WordPredicate {
    fn value_count() -> u8 {
        2 // A bit: 0 or 1
    }
}

// --- 2. Function to tokenize text and convert to WordPredicates ---
fn text_to_word_predicates(text: &str, vocabulary: &HashMap<String, usize>) -> Vec<WordPredicate> {
    let mut predicates = Vec::new();
    let words: Vec<String> = text
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect();

    for word in words {
        // If the word is in our vocabulary, it's 'present' (true), otherwise 'absent' (false)
        predicates.push(WordPredicate(vocabulary.contains_key(&word)));
    }
    predicates
}

// --- 3. Function to generate n-grams of WordPredicates ---
fn generate_predicate_ngrams(predicates: &[WordPredicate], n: usize) -> Vec<Vec<WordPredicate>> {
    if predicates.len() < n {
        return Vec::new();
    }
    predicates.windows(n).map(|window| window.to_vec()).collect()
}

// --- Main Function: Demonstrates the analysis ---
fn main() {
    println!("\n--- Word Predicate Analyzer ---");

    // Sample text for analysis
    let sample_text = "The lattice is a meme, it is a vibe, but it is also math. To understand 1 repo we need another 1k to ground all the ideas of it.";

    // Define a conceptual vocabulary (words we care about as predicates)
    let mut vocabulary: HashMap<String, usize> = HashMap::new();
    vocabulary.insert("lattice".to_string(), 1);
    vocabulary.insert("meme".to_string(), 1);
    vocabulary.insert("vibe".to_string(), 1);
    vocabulary.insert("math".to_string(), 1);
    vocabulary.insert("repo".to_string(), 1);
    vocabulary.insert("understand".to_string(), 1);
    vocabulary.insert("ideas".to_string(), 1);
    vocabulary.insert("bit".to_string(), 1);
    vocabulary.insert("predicate".to_string(), 1);

    println!("Analyzing text: \"{}\"", sample_text);

    // Convert text to a sequence of WordPredicates
    let word_predicates = text_to_word_predicates(sample_text, &vocabulary);
    println!("\nWord Predicates (Presence in Vocabulary): {:?}", word_predicates);

    // Create a conceptual LatticeLayer for these predicates
    let mut predicate_layer = LatticeLayer::<WordPredicate>::new(ValueType::Bit);
    predicate_layer.add_instance(Instance::new("SampleTextPredicates", word_predicates.len() as u8, word_predicates.clone()));

    // Demonstrate n-gram analysis on these predicates
    let n_gram_sizes = vec![2, 3]; // Pairs and Triples

    for n in n_gram_sizes {
        let ngrams = generate_predicate_ngrams(&word_predicates, n);
        println!("\n--- {}-grams of Word Predicates ---", n);
        if ngrams.is_empty() {
            println!("  Not enough predicates to form {}-grams.", n);
        } else {
            for ngram in ngrams {
                println!("  {:?}", ngram);
            }
        }
    }

    // Conceptual integration with the Lattice structure
    let mut conceptual_lattice = Lattice::new("Text Analysis Lattice");
    conceptual_lattice.add_layer(predicate_layer);
    conceptual_lattice.describe();

    println!("\nThis program demonstrates how the 'word as predicate' concept can be applied");
    println!("using the lattice types to analyze textual data and identify patterns.");
}
