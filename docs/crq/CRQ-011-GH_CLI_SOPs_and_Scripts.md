# CRQ-011-GH_CLI_SOPs_and_Scripts.md

## Change Request: GitHub CLI SOPs and Wrapper Scripts

### Objective

To enhance project documentation and operational efficiency by adding Standard Operating Procedures (SOPs) for GitHub CLI interactions and providing convenient shell scripts to wrap common `gh cli` operations.

### Description

This CRQ encompasses the following additions:

1.  **GitHub CLI SOPs:** New SOP documents have been created to guide users on interacting with GitHub via the `gh cli` for:
    *   Checking workflow status (`SOP_GH_CLI_Check_Workflows.md`)
    *   Checking issues (`SOP_GH_CLI_Check_Issues.md`)
    *   Checking pull requests (`SOP_GH_CLI_Check_PRs.md`)

2.  **AI Agent Management SOP:** A foundational SOP (`SOP_AI_Agent_Management_via_PRs.md`) has been introduced to define the process of managing other AI agents through GitHub Pull Request comments using `@` mentions.

3.  **GitHub CLI Wrapper Scripts:** A collection of shell scripts has been added to `tools/gh_scripts/` to simplify common `gh cli` operations. These scripts include:
    *   Scripts for listing, viewing, and rerunning workflow runs.
    *   Scripts for listing, viewing, and creating issues.
    *   Scripts for listing, viewing, creating, and checking out pull requests.
    *   A script (`gh_extract_actors.sh`) to extract unique users (actors) from issues and comments, providing valuable insights into project interactions.

These additions aim to streamline development workflows, improve documentation accessibility, and facilitate interaction with automated agents. The shell scripts have been created and their shebangs adjusted for compatibility.

### Expected Outcome

*   Comprehensive SOPs for GitHub CLI usage are available in `docs/sops/`.
*   Convenient shell scripts for common `gh cli` tasks are available in `tools/gh_scripts/`.
*   The project has a defined process for AI agent management via PR comments.
*   Improved developer experience and operational consistency.

### Justification/Benefit

*   **Increased Efficiency:** Automates repetitive `gh cli` commands and provides quick access to common operations.
*   **Enhanced Documentation:** Offers clear, step-by-step guides for GitHub interactions.
*   **Facilitates AI Integration:** Lays the groundwork for seamless interaction with AI agents within the development pipeline.
*   **Improved Collaboration:** Provides tools for better understanding and managing project interactions.

### Dependencies

*   GitHub CLI (`gh`) installed and authenticated.
pt.
*   Implement a Git pre-commit hook with `shellcheck`.
