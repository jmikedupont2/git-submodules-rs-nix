**CRQ: Remove Stabilized Edition2024 Feature and Unused Imports**

**Problem/Goal:**
The `cargo test` command was emitting warnings related to the `edition2024` feature being stabilized and unused imports in several Rust crates. These warnings clutter the test output and indicate unnecessary code/configuration. The goal is to eliminate these warnings to maintain a clean build and test environment.

**Proposed Solution:**
1. Remove the `cargo-features = ["edition2024"]` line from `Cargo.toml` files where it was present, as the feature is now stable.
2. Ensure `edition = "2024"` is correctly set in the `[package]` section of these `Cargo.toml` files.
3. Remove unused `use` statements from Rust source files as identified by `cargo test`. Specifically, `crq_parser::CRQ` and `WordPredicate` from `submodules/src/lib.rs`, and `NGramDecomposition` from `src/bin/ngram_reporter.rs`.

**Justification/Impact:**
Removing these warnings improves code quality and developer experience by reducing noise in the build and test output. It ensures that the project adheres to best practices for Rust development and avoids deprecated or unnecessary configurations. This also makes it easier to spot genuine warnings or errors in the future.
