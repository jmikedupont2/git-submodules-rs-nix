# SOP: Using CRQ as Commit Message

## Objective

To standardize the practice of using a Change Request Quality (CRQ) document directly as the commit message for relevant commits.

## Procedure

1.  **Identify Relevant CRQ:** Before committing changes, identify the CRQ document that most accurately and comprehensively describes the changes being committed.
2.  **Stage Changes:** Ensure all relevant changes (modified, added, deleted files) are staged for commit using `git add .` or `git add <file>...`.
3.  **Commit with CRQ:** Execute the git commit command using the `-F` flag, pointing directly to the CRQ file:
    ```bash
    git commit -F docs/crq/<CRQ_FILENAME>.md
    ```
    Replace `<CRQ_FILENAME>.md` with the actual filename of the chosen CRQ document (e.g., `CRQ-005-readme-updates.md`).
4.  **Verify Commit:** After the commit, verify its success and inspect the commit message using `git log -1`.

## Benefits

*   Ensures detailed and standardized commit messages.
*   Directly links code changes to their corresponding change requests.
*   Improves traceability and understanding of project history.
*   Reduces the effort of writing separate commit messages when a CRQ already exists.

## Notes

*   Only use CRQs that are complete and accurately reflect the committed changes.
*   If a CRQ does not fully cover the changes, consider updating the CRQ or creating a new one before committing.
