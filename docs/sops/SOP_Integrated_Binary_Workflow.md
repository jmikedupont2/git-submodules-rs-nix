# SOP: Integrated Binary Workflow

## Objective

To outline a conceptual workflow for integrating the functionalities of `submodule-collector`, `git-config-parser`, and `project_file_lattice_builder` to achieve a more comprehensive analysis and classification of project files, incorporating Git repository and submodule metadata.

## Workflow Steps

This workflow requires scripting to orchestrate the execution and data flow between the binaries. Direct integration into `project_file_lattice_builder` would require modifications to its input mechanism.

### Step 1: Collect Comprehensive Submodule Data

Use `submodule-collector` to generate a JSON report containing details of all Git repositories and submodules within a specified root directory. This report will provide the foundational data, including paths to each repository.

**Gemini Action:**
```python
print(default_api.run_shell_command(
    command="./target/debug/submodule-collector --root-dir /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules --output-file /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/integrated_submodule_report.json",
    description="Generate a comprehensive submodule report."
))
```

### Step 2: Enrich Data with Git Configuration Details

Parse the `integrated_submodule_report.json`. For each Git repository identified in the report, use `git-config-parser` to extract its `.git/config` and `.gitmodules` information. This data can then be merged with the existing repository information.

**Gemini Action (Conceptual - requires parsing JSON and looping):**
```python
# Pseudocode for illustration
# submodule_report = read_json_file("integrated_submodule_report.json")
# for repo in submodule_report["repositories"]:
#     repo_path = repo["path"]
#     git_config_path = f"{repo_path}/.git/config"
#     git_modules_path = f"{repo_path}/.gitmodules"
#     git_config_data = run_git_config_parser(git_config_path, git_modules_path)
#     # Merge git_config_data into repo object
# write_json_file("enriched_submodule_report.json", submodule_report)
```

### Step 3: Build an Enriched File Lattice

Utilize the `project_file_lattice_builder` to classify project files. For a truly integrated approach, `project_file_lattice_builder` would need to be modified to accept the `enriched_submodule_report.json` as an input. It could then use the Git-specific metadata (e.g., remote URLs, submodule status, custom Git configs) to inform its predicate extraction and classification logic, creating a more context-aware lattice.

**Gemini Action (Conceptual - requires modification to project_file_lattice_builder or a sophisticated wrapper script):**
```python
# Pseudocode for illustration
# run_project_file_lattice_builder_with_enriched_data("enriched_submodule_report.json")
```

## Benefits of Integration

*   **Deeper Insights:** Gain a more profound understanding of the project's file structure by incorporating Git-specific metadata into the classification process.
*   **Enhanced Traceability:** Improve the ability to trace files back to their originating repositories, submodules, and specific Git configurations.
*   **Automated Analysis:** Automate the collection and analysis of complex project structures, reducing manual effort.
*   **Customizable Classification:** Enable more sophisticated and context-aware file classification based on Git properties.

## Notes

*   The integration of Step 2 and Step 3 requires custom scripting or modifications to the existing binaries to handle data passing and processing.
*   This SOP provides a conceptual framework; the exact implementation details will depend on the desired level of integration and the specific analysis goals.
