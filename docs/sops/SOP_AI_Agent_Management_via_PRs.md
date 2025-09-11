# SOP: AI Agent Management via Pull Request Comments

## Objective

To establish a standardized procedure for interacting with and managing other AI agents within the development workflow by leveraging GitHub Pull Request (PR) comments and `@` mentions.

## Concept

This SOP introduces a meta-management system where specific AI agents can be invoked, instructed, or queried directly within the context of a Pull Request by mentioning their designated GitHub username (e.g., `@agent-name`) in a PR comment, followed by a command or query.

## Procedures

### 1. Identifying the Target AI Agent

Each AI agent integrated into the workflow will have a unique GitHub username. To interact with a specific agent, use its username preceded by an `@` symbol in a PR comment.

**Example:** `@code-reviewer-ai`

### 2. Issuing Commands or Queries

After mentioning the agent, follow the mention with a clear, concise command or query. The specific commands and expected syntax will be detailed in individual SOPs for each agent type.

**General Syntax:**

```
@<agent-username> <command> [arguments]
```

**Example:**

```
@code-reviewer-ai review this PR for security vulnerabilities.
```

### 3. Context of Interaction

AI agents will interpret commands within the context of the PR where the comment is made. This means they will typically operate on the code changes, files, or metadata associated with that specific Pull Request.

### 4. Receiving Agent Responses

Agent responses will typically be posted as new comments within the same Pull Request thread. The format and verbosity of responses will vary by agent and will be detailed in their respective SOPs.

### 5. Error Handling and Feedback

If an agent encounters an error or cannot fulfill a request, it should provide clear feedback in a PR comment, indicating the issue and, if possible, suggesting corrective actions.

## Benefits

*   **Centralized Management:** Manage AI agents directly within the familiar GitHub PR workflow.
*   **Contextual Interaction:** Agents operate on relevant PR data, reducing ambiguity.
*   **Auditability:** All agent interactions and responses are recorded in the PR history.
*   **Scalability:** Easily integrate new AI agents by defining their interaction protocols.

## Notes

*   Specific commands, capabilities, and response formats for each AI agent will be detailed in separate, dedicated SOPs (e.g., `SOP_Code_Reviewer_AI.md`, `SOP_Test_Generator_AI.md`).
*   Ensure the AI agent has the necessary permissions to access the repository and perform the requested actions.
