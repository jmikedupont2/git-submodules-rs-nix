# CRQ-45-crq-012-naersk-integration.md

## Change Request: crq 012 naersk integration
**Change Request (CRQ)**

**Title:** Integrate Naersk Submodule and Enhance Nix/Rust Build System for Context Introspection Tools

**Description:**
This CRQ details the integration of the `naersk` submodule to streamline Rust project builds within the Nix environment. This integration is crucial for enabling reproducible builds of our internal Rust tools, specifically the newly introduced `git-config-parser` and `git-history-parser`. These tools are foundational components of the "Project Context Introspector" (CRQ-003), allowing for the parsing of Git configurations and history into structured JSON data. The changes include updates to `.gitmodules`, `flake.nix`, `flake.lock`, `Cargo.toml`, `rust-toolchain.toml`, and the addition of `generate-lock.nix`, `shell.nix`, `src/bin/git-config-parser.rs`, and `tools/git-history-parser.sh`. This work directly supports the automation of SOPs (CRQ-002) and the broader vision of a self-aware project.

**Justification/Business Value:**
*   **Improved Reproducibility**: `naersk` ensures consistent Rust builds across environments.
*   **Enhanced Context Introspection**: Enables the development and use of tools for parsing Git configurations and history.
*   **Foundation for Automation**: Supports CRQ-002 by providing a robust build system for Rust-based automation tools.
*   **Streamlined Development**: Simplifies the build process for Rust projects within Nix.

**Scope:**
*   **Included**:
    *   Integration of `naersk` submodule.
    *   Updates to Nix configuration (`flake.nix`, `flake.lock`, `generate-lock.nix`, `shell.nix`).
    *   Updates to Rust project configuration (`Cargo.toml`, `rust-toolchain.toml`).
    *   Addition of `src/bin/git-config-parser.rs` and `tools/git-history-parser.sh`.
*   **Excluded**:
    *   Full implementation of CRQ-003 (Context Introspector) or CRQ-002 (Automate SOPs).

**Impact:**
*   **Positive**: More reliable Rust builds, foundational tools for context introspection, progress towards project automation.
*   **Negative**: Initial setup and configuration effort.

**Dependencies:**
*   Existing Nix environment.
*   Rust toolchain.

**Effort/Timeline:**
*   **Estimated Effort**: Medium.
*   **Estimated Timeline**: Completed with current commit.

**Verification (QA Plan):**

The QA plan will verify the successful `naersk` integration and the functionality of the new context introspection tools.

1.  **Verify `naersk` Integration and Rust Build:**
    *   Run `nix build .#submodules-project` to ensure the main Rust project builds successfully using `naersk`.
    *   Run `nix build .#git-config-parser` to ensure the `git-config-parser` tool builds successfully.
    *   Run `nix build .#cargo-lock-generator` to ensure the lock file generator works.
    *   Run `nix develop` and then `cargo test` within the development shell to ensure Rust tests pass.

2.  **Test `git-config-parser` Functionality:**
    *   **Test `.gitmodules` parsing:**
        *   Run the `git-config-parser` tool with the current `.gitmodules` file:
            ```bash
            ./result/bin/git-config-parser --git-modules .gitmodules
            ```
        *   Verify the output is valid JSON and accurately reflects the submodules defined in `.gitmodules`.
    *   **Test `.git/config` parsing:**
        *   Create a dummy `.git/config` file (or use an existing one if available and safe).
        *   Run the `git-config-parser` tool with the dummy `.git/config` file:
            ```bash
            ./result/bin/git-config-parser --git-config /path/to/dummy/.git/config
            ```
        *   Verify the output is valid JSON and accurately reflects the configuration.
    *   **Test combined parsing:**
        *   Run the tool with both arguments:
            ```bash
            ./result/bin/git-config-parser --git-config /path/to/dummy/.git/config --git-modules .gitmodules
            ```
        *   Verify the output contains both parsed configurations in valid JSON.

3.  **Test `git-history-parser.sh` Functionality:**
    *   Run the `git-history-parser.sh` script:
        ```bash
        ./tools/git-history-parser.sh > git_history.json
        ```
    *   Verify `git_history.json` is created, is valid JSON, and contains a reasonable representation of the Git history.

**QA Status: All steps successfully executed and verified.**
