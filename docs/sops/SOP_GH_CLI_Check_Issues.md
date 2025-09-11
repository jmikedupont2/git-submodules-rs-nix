# SOP: GitHub CLI - Checking Issues

## Objective

To provide instructions for using the GitHub CLI (`gh cli`) to list, view, and manage issues for the current repository.

## Prerequisites

*   GitHub CLI (`gh`) installed and authenticated.

## Procedures

### 1. List Issues

To see a list of issues for the current repository:

```bash
gh issue list
```

This command will display a table with information such as the issue number, title, labels, and author. You can filter issues using various flags:

*   `--state {open|closed|all}`: Filter by state (default: `open`).
*   `--assignee <USER>`: Filter by assignee.
*   `--author <USER>`: Filter by author.
*   `--label <LABEL>`: Filter by label.
*   `--search <QUERY>`: Search issues by keywords.

**Example: List closed issues assigned to 'octocat'**
```bash
gh issue list --state closed --assignee octocat
```

### 2. View Details of a Specific Issue

To get detailed information about a specific issue, including its description, comments, and timeline:

```bash
gh issue view <ISSUE_NUMBER>
```

Replace `<ISSUE_NUMBER>` with the actual number of the issue.

### 3. Create a New Issue

To create a new issue:

```bash
gh issue create
```

This will open an interactive prompt to guide you through creating the issue, including title, body, labels, and assignees.

### 4. Close an Issue

To close an existing issue:

```bash
gh issue close <ISSUE_NUMBER>
```

### 5. Reopen an Issue

To reopen a closed issue:

```bash
gh issue reopen <ISSUE_NUMBER>
```

## Examples for Gemini

### Example 1: List all open issues

```python
print(default_api.run_shell_command(
    command="gh issue list",
    description="List all open issues for the current repository."
))
```

### Example 2: View details of issue number 123

```python
print(default_api.run_shell_command(
    command="gh issue view 123",
    description="View details of issue number 123."
))
```

## Notes

*   Ensure you are in the correct repository directory when running `gh cli` commands.
*   The `gh cli` commands are context-aware and will operate on the repository associated with the current working directory.
