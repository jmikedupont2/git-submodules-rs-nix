# Current Development Plan

This document outlines the immediate next steps for the project, focusing on the `submodule-collector` and its reporting capabilities, as well as development environment enhancements.

## 1. Generate Comprehensive Submodule Report
The `submodule-collector` has been updated to improve error reporting. The next step is to run it to generate a comprehensive JSON report of all Git repositories and their submodules within the main project scope.

*   **Action:** Execute the `submodule-collector` from the appropriate root directory, ensuring the output JSON includes both successfully processed repositories and detailed information on any failures.
*   **Command (example):**
    ```bash
    ./target/release/submodule-collector --root-dir /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/ --output-file submodule_report_recursive_resilient.json
    ```
    *(Note: The `--root-dir` should point to the root of your meta-introspector project.)*

## 2. Develop Submodule Report Function (`report-analyzer-rs`)
A new reporting capability is required to analyze the generated JSON report. This function will extract key statistics and insights, and perform advanced string analysis.

*   **Objective:** Create a Rust-based mechanism (`report-analyzer-rs`) to read `submodule_report_recursive_resilient.json` and provide:
    *   Counts of successful vs. failed repositories.
    *   Identification of duplicate repository entries.
    *   Analysis of most frequently mentioned organizations (from repository URLs).
    *   Analysis of most frequently mentioned names or strings (e.g., submodule names, repository names).
    *   Analysis of top N-grams for various sizes (1, 2, 3, 5, 7, 11, 13, 17, 19).
    *   Identification of the Longest Common Prefix (LCP) among all paths and URLs for conceptual compression.
*   **Implementation Approach:** Implemented as a standalone Rust binary (`report-analyzer-rs`).

## 3. Document Changes (CRQ & README)
Formalize the recent development efforts and the plan for the reporting function.

*   **Action (CRQ):** Create `docs/crq/CRQ-002-submodule-report-function.md` outlining the task of developing the submodule report function, its objectives, and expected outcomes.
*   **Action (README):** Update the project's `README.md` to reflect the enhanced `submodule-collector` capabilities and the new `report-analyzer-rs` features.

## 4. Profiling and Development Environment Enhancements
Enhance the development environment with profiling tools and additional Emacs packages.

*   **4.1. Integrate Rust Profiling**
    *   **Objective:** Introduce a Rust profiling tool to analyze performance bottlenecks in the codebase.
    *   **Action:** Configure `iai-callgrind` (or direct `valgrind` usage) for benchmarking and profiling.
    *   **Status:** `iai-callgrind` added as dev-dependency. `flake.nix` updated with `valgrind`. Further testing required after re-entering Nix environment.
    *   **Report Generation:** Profiling reports will be generated in `callgrind.out.<pid>` format, viewable with `kcachegrind` or `callgrind_annotate`.

*   **4.2. Enhance Emacs Development Environment**
    *   **Objective:** Add common Emacs packages for Rust, Lean, OCaml, and LSP development to the Nix flake.
    *   **Action:** Added `emacsPackages.magit`, `emacsPackages.rustic`, `emacsPackages.cargo-mode`, `emacsPackages.rust-mode`, `emacsPackages.lsp-mode`, `emacsPackages.company`, `emacsPackages.flycheck`, `emacsPackages.lsp-ui`, `emacsPackages.dap-mode`, `emacsPackages.tuareg`, `emacsPackages.merlin-mode`, `emacsPackages.dune-mode`, and `emacsPackages.utop` to `flake.nix`.
    *   **Status:** Packages added to `flake.nix`. Requires re-entering `nix develop` for changes to take effect.

## 5. Next Steps: Advanced String Analysis and Compression
The next phase focuses on refining the string analysis and implementing a conceptual compression scheme.

*   **5.1. Generalized N-gram Analysis (Complete)**
    *   **Objective:** Extend `report-analyzer-rs` to generate and analyze N-grams for specified sizes (1, 2, 3, 5, 7, 11, 13, 17, 19).
    *   **Status:** Implemented and verified.

*   **5.2. Conceptual "Compression" and Emoji Ontology**
    *   **Objective:** Create a new compressed version of the JSON output, inspired by linked data principles (like Turtle/JSON-LD), using emoji keys for common strings/prefixes to represent an "ontology".
    *   **Sub-tasks:**
        *   **Identify Common Strings and Prefixes:** Develop a method to identify a comprehensive set of most frequent tokens and common prefixes/paths beyond just the LCP.
        *   **Develop Emoji Ontology:** Assign meaningful emojis to these identified common strings/concepts. This mapping will serve as a simple ontology.
        *   **Generate "Compressed" JSON Output:**
            *   Define a custom JSON output structure.
            *   Include a "header" section in the JSON that defines the emoji-based ontology (mapping emojis to their corresponding full strings/prefixes).
            *   Modify the main data section to use these emoji "keys" to shorten the original URLs and paths, effectively compressing the output.