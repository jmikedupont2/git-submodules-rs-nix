**CRQ: Enhance CRQ Parser for CoderabbitAI Skipped Reviews**

**Problem/Goal:**
To enhance the `crq_parser` within the `crq_table_generator` project to automatically identify CRQs where CoderabbitAI has skipped an automated review, and classify them as requiring a manual review request to CoderabbitAI.

**Proposed Solution:**

1.  **New `NextStep` Variant:**
    *   Add `RequestCoderabbitAIReview` to the `NextStep` enum in `crq_parser.rs`.

2.  **`crq_parser.rs` Modifications:**

    *   **New Function: `check_coderabbitai_comms(crq_id: &str) -> bool`**
        *   This function will check communication logs for a given `crq_id`.
        *   It will construct the path to CoderabbitAI responses (e.g., `analysis_data/comms/git/coderabbitai/{crq_id}/responses/`).
        *   It will read file content and search for keywords/patterns indicating a skipped review (e.g., "Review skipped", "Auto reviews are disabled").
        *   It will return `true` if a skipped review message is found, `false` otherwise.

    *   **Modify `determine_next_step(crq_content: &str, crq_id: &str) -> NextStep`**
        *   The function will accept `crq_id` as an additional argument.
        *   The classification logic will be updated:
            1.  **CoderabbitAI Review (Size-based):** (Existing rule) If content size < 2500 bytes, classify as `CoderabbitAIReview`.
            2.  **Request CoderabbitAI Review (Comms-based):** (NEW RULE) If `check_coderabbitai_comms(crq_id)` returns `true`, classify as `RequestCoderabbitAIReview`. This rule takes precedence over general "Respond To / Our Turn".
            3.  **Respond To / Our Turn:** (Existing rule) If neither of above, and CRQ contains relevant keywords, classify as `RespondTo`.
            4.  **Develop/Refactor/Document/Unknown:** (Existing rules) Fallback if no other specific classification applies.

3.  **`main.rs` Modifications:**

    *   **Extract `crq_id`:** Extract CRQ ID from filename and pass to `determine_next_step`.
    *   **Update `match` statement:** Add a new case for `NextStep::RequestCoderabbitAIReview` for display.

**Justification/Impact:**
This enhancement will make the `crq_parser` more intelligent and context-aware by incorporating external communication context into its classification decisions. It will automate the identification of CRQs that require manual intervention to trigger a CoderabbitAI review, improving workflow efficiency and ensuring that skipped reviews are addressed proactively.

**Progress Update (2025-09-11):**

The core implementation of the proposed solution has been completed and integrated into the `crq_table_generator`. This includes:

*   Refined `NextStep` enum with new states: `ReviewProvided`, `ReviewSkipped (No Meaningful Response)`, `ReviewNeeded from CoderabbitAI`, and `Respond To / Our Turn`.
*   Implementation of `check_coderabbitai_comms` to analyze communication logs for skipped reviews and meaningful responses.
*   Updated `determine_next_step` with a prioritized state machine logic, incorporating n-gram analysis and communication log data.
*   The `crq_table_generator` now produces a detailed report reflecting these new classifications.

This phase of the CRQ is considered complete. Further work will be tracked under new CRQs, such as `CRQ-046-crq-classification-via-comms-analysis-and-state-machine.md` for more advanced comms analysis.


## CRQ Status Report (Appended 2025-09-11)

**1. Agent Rate Limit Management and Interaction Receipts (CRQ-046)**

*   **Problem/Goal:** To manage interactions with external agents (like CoderabbitAI) by respecting their API rate limits and to record these interactions within CRQ documents.
*   **Progress:**
    *   The `crq_table_generator` has been enhanced to include new classification states: `IssueTooLarge` and `OverQuota`. These states are now prioritized in the classification logic, allowing for immediate identification of CRQs affected by size limits or rate limits.
    *   The `check_coderabbitai_comms` function within the `crq_table_generator` has been updated to detect keywords related to "issue too large" and "over quota" messages in CoderabbitAI's communication logs.
    *   The `process_all_crq_branches.sh` script, which pulls communication data from GitHub, has been fixed to correctly mirror all CoderabbitAI responses, ensuring accurate data for analysis.
    *   The `crq_table_generator` can now generate `gh pr comment` commands for CRQs classified as `ReviewSkipped`, facilitating manual review requests.
*   **Next Steps:** The core logic for detecting rate limit and size issues is in place. The next phase for this CRQ would involve implementing the actual rate limit management (e.g., delaying command execution based on reset times) and the mechanism for adding interaction receipts directly into CRQ files.

**2. CRQ Classification via Comms Analysis and State Machine (Proposed as CRQ-056)**

*   **Problem/Goal:** To achieve a more deterministic and nuanced classification of CRQs by analyzing their communication history with CoderabbitAI, moving towards a state machine model.
*   **Progress:**
    *   The `crq_table_generator` now employs a more granular state machine logic for CRQ classification. This includes states suchs `ReviewProvided`, `ReviewSkipped (No Meaningful Response)`, `ReviewNeeded from CoderabbitAI`, and `Respond To / Our Turn`.
    *   The `crq_word_analyzer` tool has been significantly enhanced to support advanced text analysis:
        *   It can now perform n-gram analysis (for lengths 2, 3, 5, 7, 11, 13, 17, 19) to identify more contextual patterns in text.
        *   It includes a new mode for classifying individual CoderabbitAI responses into types like `SkippedMessage`, `RateLimitMessage`, `ReviewSummary`, and `QuestionMessage`.
        *   It can perform sequential response analysis, identifying common words/n-grams at different positions within a communication sequence.
*   **Next Steps:** Further refinement of the classification rules based on the insights gained from n-gram analysis and individual response type classification. This will lead to a more robust and accurate state machine for CRQ triage.

This report summarizes the significant progress made in automating and refining our CRQ management and classification processes.

## Related Work: Wikipedia and Wikidata Extraction

During the analysis of the codebase, significant components related to Wikipedia and Wikidata extraction were identified, primarily within the `wikipedia_extractor` Rust crate. This crate is designed to facilitate the extraction of information from Wikipedia articles and Wikidata entities, which is crucial for the broader project goal of analyzing and understanding external knowledge sources.

**Key Files and Components:**

*   **`wikipedia_extractor/Cargo.toml`**: Defines the crate and its dependencies, including `wikipedia` and `wikidata` crates for API interactions, `reqwest` for HTTP, `tokio` for async operations, `scraper` for HTML parsing, and `serde` for data serialization/deserialization.
*   **`wikipedia_extractor/src/lib.rs`**: The main library file, exposing the following modules and functions:
    *   `data_structures`: Defines data structures like `WikipediaArticle`, `WikidataFact`, and `WikidataEntity` for representing extracted information.
    *   `wikipedia_parser`: Contains logic for parsing Wikipedia articles, including `extract_article_data`.
    *   `wikidata_client`: Handles interactions with the Wikidata API, including `fetch_wikidata_entity`.
    *   `cache`: Provides caching mechanisms with functions like `save_article_to_cache`, `load_article_from_cache`, `save_entity_to_cache`, and `load_entity_from_cache`.
*   **`wikipedia_extractor/src/wikipedia_parser.rs`**: Implements the core logic for fetching and parsing Wikipedia articles.
*   **`wikipedia_extractor/src/wikidata_client.rs`**: Implements the core logic for fetching Wikidata entities.
*   **`wikipedia_extractor/src/cache.rs`**: Manages the caching of Wikipedia articles and Wikidata entities, using `wikipedia_extractor/cache/wikipedia` and `wikipedia_extractor/cache/wikidata` directories.
*   **`wikipedia_extractor/tests/integration_test.rs`**: Contains integration tests demonstrating the functionality of fetching and processing Wikipedia articles and Wikidata entities.

This `wikipedia_extractor` crate directly supports the project's objectives of:
*   Extracting links from the repository (by processing content that might contain Wikipedia/Wikidata links).
*   Finding Wikipedia articles and Wikidata entities.
*   Cross-referencing extracted links with Wikipedia content.

The presence and active development of this crate indicate a strong focus on integrating external knowledge bases into the meta-introspector's analytical capabilities.