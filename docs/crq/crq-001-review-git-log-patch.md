**Change Request (CRQ)**

**Title:** Automate SOP Generation from Full Git History

**Description:**
This change requests the development of a process and tooling to automatically review the entire Git history of the project (`git log --all --patch`) and extract all significant operations. For each identified operation, a corresponding Standard Operating Procedure (SOP) document will be generated, detailing the context, steps, and best practices. All generated SOPs will be organized and stored in the `docs/sops/` directory.

**Justification/Business Value:**
*   **Comprehensive Documentation**: Ensures that all historical operations, fixes, and feature implementations are formally documented, preventing knowledge loss.
*   **Improved Onboarding**: New team members can quickly understand the project's evolution and common operational procedures by reviewing auto-generated SOPs.
*   **Enhanced Reproducibility**: By documenting every significant operation, the project's setup and maintenance become more reproducible.
*   **Knowledge Management**: Centralizes operational knowledge derived directly from the codebase's history.

**Scope:**
*   **Included**:
    *   Review of the complete Git history (`git log --all --patch`).
    *   Identification and extraction of distinct operational procedures.
    *   Automated generation of SOP documents for each identified operation.
    *   Organization of generated SOPs in `docs/sops/`.
*   **Excluded**:
    *   Manual review and refinement of *every* auto-generated SOP (initial focus is on automation).
    *   Deep domain-specific analysis beyond what can be inferred from commit messages and code changes.

**Impact:**
*   **Positive**: Significantly improves project documentation and knowledge transfer.
*   **Negative**: Requires significant development effort to create the necessary parsing and SOP generation tooling. Potential for initial noise or less refined SOPs due to automated generation.

**Dependencies:**
*   Access to the full Git repository history.
*   Development of custom scripts/tools capable of parsing `git log --patch` output.
*   Defined templates or structures for SOP documents.

**Effort/Timeline:**
*   **Estimated Effort**: High. This task requires significant development of custom tooling for Git history parsing and intelligent SOP generation. It is not a trivial task that can be completed with existing general-purpose tools.
*   **Estimated Timeline**: To be determined after a detailed design and feasibility study of the required tooling.

**Verification:**
*   Confirmation that `docs/sops/` contains a comprehensive set of SOP documents.
*   Spot-checking of generated SOPs for accuracy and relevance to the corresponding Git history.

## Review of Last GitHub Action Failure

As part of the ongoing project review, the last GitHub Action failure (Run ID: 17621809753) was investigated. The failure was identified as a `sha256` hash mismatch in the `flake.nix` file, specifically for the `git-submodules-rs-nix` repository. This issue was subsequently resolved by commit `fdecbc5fc889277e2a8e2dfc8a1806e90b4ef00b` ("fix: Update flake.nix sha256 for repo fetch").