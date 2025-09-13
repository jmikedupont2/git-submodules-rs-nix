# SOP: Using `git-config-parser`

## Objective

To provide instructions for using the `git-config-parser` binary to extract and parse information from `.git/config` and `.gitmodules` files.

## Usage

The `git-config-parser` binary is located at `target/debug/git-config-parser` (or `target/release/git-config-parser` if built in release mode).

### Basic Syntax

```bash
./target/debug/git-config-parser --git-config <PATH_TO_GIT_CONFIG> --git-modules <PATH_TO_GITMODULES>
```

### Parameters

*   `--git-config <PATH_TO_GIT_CONFIG>`: (Optional) Specifies the absolute path to the `.git/config` file to parse. If omitted, no Git config will be parsed.
*   `--git-modules <PATH_TO_GITMODULES>`: (Optional) Specifies the absolute path to the `.gitmodules` file to parse. If omitted, no Git modules will be parsed.

### Output

The tool prints a JSON object to standard output containing the parsed data. The JSON will have keys `git_config` and `git_modules` if the respective files were provided and parsed successfully.

## Examples for Gemini

### Example 1: Parse a `.git/config` file

To parse the `.git/config` file of the current repository:

```python
print(default_api.run_shell_command(
    command="./target/debug/git-config-parser --git-config /path/to/your/repo/.git/config",
    description="Parse the .git/config file and output as JSON."
))
```

### Example 2: Parse a `.gitmodules` file

To parse the `.gitmodules` file of the current repository:

```python
print(default_api.run_shell_command(
    command="./target/debug/git-config-parser --git-modules /path/to/your/repo/.gitmodules",
    description="Parse the .gitmodules file and output as JSON."
))
```

### Example 3: Parse both `.git/config` and `.gitmodules`

```python
print(default_api.run_shell_command(
    command="./target/debug/git-config-parser --git-config /path/to/your/repo/.git/config --git-modules /path/to/your/repo/.gitmodules",
    description="Parse both .git/config and .gitmodules files and output as JSON."
))
```

## Notes

*   Ensure the paths to `.git/config` and `.gitmodules` are absolute.
*   The tool expects valid Git configuration file formats. Invalid formats may lead to parsing errors.
