use regex::Regex;
use std::collections::HashMap;

use crate::types::{Report, Ontology};
use super::apply_emojis::apply_emoji_ontology; // Corrected import path

// New private function to collect all tokens
fn collect_all_tokens(report: &Report, tokenizer_re: &Regex) -> Vec<String> {
    let mut all_tokens: Vec<String> = Vec::new();

    // Collect tokens from successful repositories
    for (url, info) in &report.repositories {
        // Tokenize URL
        for part in tokenizer_re.split(url) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
        // Tokenize path
        for part in tokenizer_re.split(&info.path) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
        // Tokenize submodule names and nested repo URLs
        for submodule in &info.submodules {
            for part in tokenizer_re.split(&submodule.name) {
                if !part.is_empty() {
                    all_tokens.push(part.to_lowercase());
                }
            }
            if let Some(nested_repo) = &submodule.nested_repo {
                for part in tokenizer_re.split(&nested_repo.url) {
                    if !part.is_empty() {
                        all_tokens.push(part.to_lowercase());
                    }
                }
            }
        }
    }

    // Collect tokens from failed repositories (from error messages)
    for failed_repo in &report.failed_repositories {
        for part in tokenizer_re.split(&failed_repo.error) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
    }
    all_tokens
}

// New private function to count token frequencies
fn count_token_frequencies(tokens: &[String]) -> HashMap<String, usize> {
    let mut token_counts: HashMap<String, usize> = HashMap::new();
    for token in tokens {
        *token_counts.entry(token.to_string()).or_insert(0) += 1;
    }
    token_counts
}

// New private function to generate and analyze n-grams
fn generate_and_analyze_ngrams(
    all_tokens: &[String],
    ontology: &Option<Ontology>,
) -> Vec<(String, usize)> {
    let mut suggested_rules: Vec<(String, usize)> = Vec::new();
    let n_gram_sizes = vec![1, 2, 3, 5, 7, 11, 13, 17, 19];
    let max_iterations = 20; // A practical limit to prevent infinite loops

    let mut current_emoji_tokens: Vec<String> = all_tokens
        .iter()
        .map(|token| apply_emoji_ontology(token, ontology))
        .collect();

    for iteration in 1..=max_iterations {
        println!("\n--- Iteration {} ---", iteration);
        let previous_emoji_tokens = current_emoji_tokens.clone(); // For convergence check
        let mut next_iteration_tokens: Vec<String> = Vec::new(); // Tokens for the next iteration

        for &n in &n_gram_sizes {
            if current_emoji_tokens.len() >= n {
                let mut n_grams: Vec<String> = Vec::new();
                for i in 0..current_emoji_tokens.len() - (n - 1) {
                    let mut current_n_gram = String::new();
                    for j in 0..n {
                        current_n_gram.push_str(&current_emoji_tokens[i + j]);
                        if j < n - 1 {
                            current_n_gram.push_str(" "); // Add space between emoji tokens
                        }
                    }
                    n_grams.push(current_n_gram);
                }

                let mut n_gram_counts: HashMap<String, usize> = HashMap::new();
                for n_gram in n_grams {
                    *n_gram_counts.entry(n_gram).or_insert(0) += 1;
                }

                let mut sorted_n_gram_counts: Vec<(&String, &usize)> =
                    n_gram_counts.iter().collect();
                sorted_n_gram_counts.sort_by(|a, b| b.1.cmp(a.1));

                println!(
                    "\n--- Most Frequently Mentioned {}-grams (Iteration {}) ---",
                    n, iteration
                );
                for (n_gram, count) in sorted_n_gram_counts.iter().take(10) {
                    let compressed_n_gram = apply_emoji_ontology(n_gram, ontology);
                    println!("{}: {}", compressed_n_gram.replace(" ", ""), count); // Remove spaces for final output

                    // Collect suggestions
                    if **n_gram != compressed_n_gram { // Fixed type mismatch
                        suggested_rules.push((n_gram.to_string(), **count));
                    }

                    // For the next iteration, we want the compressed version of the n-gram
                    // if it was actually compressed, otherwise the original n-gram.
                    // We also need to ensure we're not adding duplicates or single emojis that are already compressed.
                    next_iteration_tokens.push(compressed_n_gram.replace(" ", ""));
                }
            } else {
                println!(
                    "\n--- Not enough tokens to generate {}-grams (Iteration {}) ---",
                    n, iteration
                );
            }
        }

        // Check for convergence
        if previous_emoji_tokens == next_iteration_tokens || iteration >= max_iterations {
            break;
        }
        current_emoji_tokens = next_iteration_tokens;
    }
    suggested_rules
}

pub fn analyze_strings(report: &Report, ontology: &Option<Ontology>) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    let tokenizer_re = Regex::new(r"[^a-zA-Z0-9]+")?;
    let all_tokens = collect_all_tokens(report, &tokenizer_re);

    if !all_tokens.is_empty() {
        let token_counts = count_token_frequencies(&all_tokens);

        let mut sorted_token_counts: Vec<(&String, &usize)> = token_counts.iter().collect();
        sorted_token_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Tokens ---");
        for (token, count) in sorted_token_counts.iter().take(20) {
            println!("{}: {}", apply_emoji_ontology(token, ontology), count);
        }
    }

    let suggested_rules = generate_and_analyze_ngrams(&all_tokens, ontology);

    Ok(suggested_rules)
}

pub fn print_suggested_rules_with_emojis(suggested_rules: &Vec<(String, usize)>, ontology: &Option<Ontology>) {
    if !suggested_rules.is_empty() {
        let mut sorted_suggestions = suggested_rules.clone();
        sorted_suggestions.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n--- Suggested New Ontology Rules (with Emojis) ---");
        for (n_gram, count) in sorted_suggestions.iter().take(5) {
            let emoji_n_gram = apply_emoji_ontology(n_gram, ontology);
            println!("\"{}\": \"{}\"", n_gram.replace(" ", ""), emoji_n_gram.replace(" ", "")); // Count: {}", n_gram.replace(" ", ""), emoji_n_gram.replace(" ", ""), count);
        }
    }
}