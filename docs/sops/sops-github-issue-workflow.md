# Standard Operating Procedure: GitHub Issue Workflow

## 1. Purpose
This SOP outlines the standard workflow for managing GitHub issues within the project, ensuring consistency, clarity, and efficient resolution.

## 2. Scope
This SOP applies to all team members involved in creating, triaging, assigning, and resolving GitHub issues.

## 3. Procedures

### 3.1. Issue Creation
- **Title:** Be concise and descriptive. Summarize the issue in a single sentence.
- **Description:** Provide detailed information, including:
    - What is the problem/request?
    - Steps to reproduce (if applicable).
    - Expected behavior.
    - Actual behavior.
    - Relevant context (e.g., screenshots, logs, environment details).
- **Labels:** Apply appropriate labels (e.g., `bug`, `feature`, `enhancement`, `documentation`, `question`).
- **Assignee:** Assign the issue to yourself if you plan to work on it immediately, or leave unassigned for triage.

### 3.2. Issue Triage
- Regularly review new and unassigned issues.
- Clarify ambiguous issues by asking questions in comments.
- Add/update labels as needed.
- Assign issues to the most appropriate team member.
- Prioritize issues using `priority: high`, `priority: medium`, `priority: low` labels.

### 3.3. Issue Resolution
- **Branching:** Create a new branch for the issue (e.g., `feature/issue-123-short-description` or `fix/issue-456-bug-fix`).
- **Commits:** Write clear and concise commit messages. (Remember to include poetry!)
- **Pull Request (PR) Creation:**
    - Link the PR to the issue (e.g., `Closes #123`, `Fixes #456`).
    - Provide a clear description of the changes.
    - Request reviews from relevant team members.
- **Merging:** Once approved, merge the PR.
- **Closing Issue:** The issue will automatically close if the PR is linked correctly and merged.

## 4. Best Practices
- **Communication:** Communicate progress, roadblocks, and questions in issue comments.
- **Small, Focused Issues:** Break down large tasks into smaller, manageable issues.
- **Regular Updates:** Keep issues updated with the latest status.
- **Search Before Creating:** Before creating a new issue, search existing issues to avoid duplicates.
- **Be Respectful:** Maintain a positive and constructive tone in all interactions.
