# SOP: GitHub CLI - Checking Pull Requests

## Objective

To provide instructions for using the GitHub CLI (`gh cli`) to list, view, and manage pull requests (PRs) for the current repository.

## Prerequisites

*   GitHub CLI (`gh`) installed and authenticated.

## Procedures

### 1. List Pull Requests

To see a list of pull requests for the current repository:

```bash
gh pr list
```

This command will display a table with information such as the PR number, title, branch, and author. You can filter PRs using various flags:

*   `--state {open|closed|all}`: Filter by state (default: `open`).
*   `--assignee <USER>`: Filter by assignee.
*   `--author <USER>`: Filter by author.
*   `--label <LABEL>`: Filter by label.
*   `--search <QUERY>`: Search PRs by keywords.

**Example: List merged PRs by 'monalisa'**
```bash
gh pr list --state merged --author monalisa
```

### 2. View Details of a Specific Pull Request

To get detailed information about a specific pull request, including its description, commits, and files changed:

```bash
gh pr view <PR_NUMBER>
```

Replace `<PR_NUMBER>` with the actual number of the pull request.

### 3. Create a New Pull Request

To create a new pull request from your current branch:

```bash
gh pr create
```

This will open an interactive prompt to guide you through creating the PR, including title, body, base branch, and reviewers.

### 4. Checkout a Pull Request

To checkout the branch of a specific pull request locally:

```bash
gh pr checkout <PR_NUMBER>
```

### 5. Merge a Pull Request

To merge an open pull request:

```bash
gh pr merge <PR_NUMBER>
```

### 6. Close a Pull Request

To close an open pull request:

```bash
gh pr close <PR_NUMBER>
```

## Examples for Gemini

### Example 1: List all open pull requests

```python
print(default_api.run_shell_command(
    command="gh pr list",
    description="List all open pull requests for the current repository."
))
```

### Example 2: View details of pull request number 456

```python
print(default_api.run_shell_command(
    command="gh pr view 456",
    description="View details of pull request number 456."
))
```

## Notes

*   Ensure you are in the correct repository directory when running `gh cli` commands.
*   The `gh cli` commands are context-aware and will operate on the repository associated with the current working directory.
