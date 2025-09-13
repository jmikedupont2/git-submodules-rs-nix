# SOP: Using `submodule-collector`

## Objective

To provide instructions for using the `submodule-collector` binary to scan Git repositories and their submodules, and generate a comprehensive JSON report.

## Usage

The `submodule-collector` binary is located at `target/debug/submodule-collector` (or `target/release/submodule-collector` if built in release mode).

### Basic Syntax

```bash
./target/debug/submodule-collector --root-dir <PATH_TO_ROOT_DIRECTORY> --output-file <PATH_TO_OUTPUT_JSON>
```

### Parameters

*   `--root-dir <PATH_TO_ROOT_DIRECTORY>`: (Required) Specifies the absolute path to the root directory where the tool should start scanning for Git repositories.
*   `--output-file <PATH_TO_OUTPUT_JSON>`: (Required) Specifies the absolute path where the generated JSON report will be saved.

### Output

The tool generates a JSON file at the specified `--output-file` path. This JSON report contains:

*   `repositories`: A map of successfully processed repositories, keyed by their remote URL, including their submodules (recursively).
*   `failed_repositories`: A list of any repositories or submodules that could not be processed due to errors, along with error messages.

## Examples for Gemini

### Example 1: Scan the current project root and save report

```python
print(default_api.run_shell_command(
    command="./target/debug/submodule-collector --root-dir /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules --output-file /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/submodule_report.json",
    description="Scan the current project directory for Git repositories and submodules, and save the report to submodule_report.json."
))
```

### Example 2: Scan a specific directory and save report

```python
print(default_api.run_shell_command(
    command="./target/debug/submodule-collector --root-dir /path/to/another/repo --output-file /path/to/another/repo/submodule_report.json",
    description="Scan a different repository directory for Git repositories and submodules, and save the report."
))
```

## Notes

*   Ensure that the `--root-dir` and `--output-file` paths are absolute.
*   The tool is designed to be resilient; it will log errors for failed repositories/submodules and continue processing others.
