# SOP: GitHub CLI - Checking Workflow Status

## Objective

To provide instructions for using the GitHub CLI (`gh cli`) to check the status of GitHub Actions workflows for the current repository.

## Prerequisites

*   GitHub CLI (`gh`) installed and authenticated. You can authenticate by running `gh auth login`.

## Procedures

### 1. List Recent Workflow Runs

To see a list of recent workflow runs for the current repository:

```bash
gh run list
```

This command will display a table with information such as the workflow name, run ID, status (e.g., `completed`, `in_progress`, `queued`), and the branch.

### 2. View Details of a Specific Workflow Run

To get detailed information about a specific workflow run, including its jobs and their statuses, use the `gh run view` command with the run ID.

First, get the run ID from `gh run list`.

```bash
gh run view <RUN_ID>
```

Replace `<RUN_ID>` with the actual ID of the workflow run you want to inspect.

### 3. View Logs for a Specific Job (within a Workflow Run)

To view the logs for a specific job within a workflow run, you can use the `--log` flag with `gh run view` and specify the job name.

```bash
gh run view <RUN_ID> --log <JOB_NAME>
```

Replace `<RUN_ID>` with the workflow run ID and `<JOB_NAME>` with the name of the job (e.g., `build`, `test`).

### 4. Re-run a Workflow

To re-run a specific workflow run:

```bash
gh run rerun <RUN_ID>
```

### 5. Watch a Workflow Run

To continuously watch the progress of a workflow run:

```bash
gh run watch <RUN_ID>
```

## Examples for Gemini

### Example 1: List all workflow runs

```python
print(default_api.run_shell_command(
    command="gh run list",
    description="List all GitHub Actions workflow runs for the current repository."
))
```

### Example 2: View details of the latest workflow run

```python
print(default_api.run_shell_command(
    command="gh run view --json databaseId --jq '.[0].databaseId' | xargs gh run view",
    description="View details of the latest GitHub Actions workflow run."
))
```

## Notes

*   Ensure you are in the correct repository directory when running `gh cli` commands.
*   The `gh cli` commands are context-aware and will operate on the repository associated with the current working directory.
