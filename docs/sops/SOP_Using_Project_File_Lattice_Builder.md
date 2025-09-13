# SOP: Using `project_file_lattice_builder`

## Objective

To provide instructions for using the `project_file_lattice_builder` binary to conceptually classify project files into a lattice structure.

## Usage

The `project_file_lattice_builder` binary is located at `target/debug/project_file_lattice_builder` (or `target/release/project_file_lattice_builder` if built in release mode).

### Basic Syntax

```bash
./target/debug/project_file_lattice_builder
```

### Description

This tool scans the current directory recursively, identifies files, extracts conceptual predicates (based on file path, extension, and predefined keywords), and classifies them into a simplified lattice hierarchy. It prints the classification details for each file to standard output.

### Parameters

This tool currently does not accept any command-line parameters. It operates on the current working directory.

### Output

The tool prints detailed information about each classified file to standard output, including:

*   File name
*   Extracted predicates
*   Conceptual classification path within the lattice

It also provides a summary of the total number of files classified.

## Examples for Gemini

### Example 1: Run the lattice builder in the current project root

```python
print(default_api.run_shell_command(
    command="./target/debug/project_file_lattice_builder",
    description="Run the project file lattice builder to classify files in the current directory."
))
```

## Notes

*   The classification is conceptual and based on predefined predicates and simplified logic within the tool.
*   The output can be extensive for large projects.
