# CRQ-009-Read_Project_Tool.md

## Change Request: Git Project Reader Library and Integration

### Objective

To develop a new, reusable Rust library (`git-project-reader`) capable of extracting comprehensive information from Git repositories, specifically focusing on listing tracked files and obtaining the `git status` of a given repository. This library will then be integrated into existing or new tools (e.g., `submodule-collector`) to enrich their reports with detailed project-level data.

### Description

This CRQ addresses the need for a modular component that can provide insights into the contents and state of Git repositories. The `git-project-reader` library will offer functionalities to:

1.  **List Tracked Files:** Given a path to a Git repository, return a list of all files tracked by Git within that repository's working directory.
2.  **Obtain Git Status:** For a given Git repository, execute `git status --porcelain` and return its output, providing a machine-readable summary of the repository's current state (e.g., modified, untracked, staged files).

The `submodule-collector` will be a primary consumer of this library, allowing its generated reports to include detailed file lists and status information for each identified submodule and repository.

### Expected Outcome

*   A new, well-documented Rust library (`git-project-reader`) with clear APIs for file listing and Git status retrieval.
*   Enhanced `submodule-collector` reports that include file lists and `git status` output for each repository/submodule.
*   Improved modularity and reusability of Git-related functionalities across the project.
*   More comprehensive data for subsequent analysis and lattice building.

### Justification/Benefit

*   **Modularity:** Separates Git interaction logic into a dedicated library, promoting cleaner code and easier maintenance.
*   **Reusability:** The `git-project-reader` can be used by other tools requiring Git repository introspection.
*   **Enriched Data:** Provides more granular and actionable data in reports, enabling deeper analysis of project states.
*   **Accuracy:** Directly leverages Git commands for accurate file and status information.

### Dependencies

*   Rust toolchain.
*   `git2` crate (for Git interactions, if chosen over shelling out).
*   Integration into `submodule-collector` (subsequent CRQ or task) - *ready for integration*.
).
