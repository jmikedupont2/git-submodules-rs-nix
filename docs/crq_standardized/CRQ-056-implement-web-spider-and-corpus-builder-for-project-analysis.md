**CRQ Title:** CRQ-056: Implement Web Spider and Corpus Builder for Project Analysis

**Problem/Goal:**
To develop a robust web spider and corpus builder capable of extracting URLs, downloading content, and hashing files from specified web sources. This tool will be used to build a comprehensive corpus for project analysis, enabling deeper insights into external dependencies, linked resources, and overall project context.

**Proposed Solution:**

1.  **Develop `web_spider_corpus_builder`:**
    *   Implement a Rust-based application to crawl specified URLs.
    *   Extract all hyperlinks from crawled pages.
    *   Download content from extracted URLs.
    *   Calculate hashes of downloaded files for integrity and deduplication.
    *   Store extracted URLs, domain reports, file type reports, and content hashes.

2.  **Integrate `url_extractor`:**
    *   Develop a dedicated Rust crate for efficient URL extraction from various text formats.

3.  **Integrate `hash_extractor_rust`:**
    *   Develop a dedicated Rust crate for generating various cryptographic hashes of file contents.

4.  **Integrate `url_hash_extractor`:**
    *   Develop a dedicated Rust crate that combines URL extraction and content hashing functionalities.

5.  **Update `Cargo.toml` and `Cargo.lock`:**
    *   Add new dependencies for web crawling, URL parsing, and hashing libraries.
    *   Ensure all dependencies are correctly managed.

6.  **Generate Reports:**
    *   `domain_report.json`: A report summarizing domains encountered.
    *   `extracted_urls.txt`: A list of all unique URLs extracted.
    *   `file_type_report.json`: A report on the types of files downloaded.
    *   `github_repo_counts.jsonld`: (Potentially) A report on GitHub repository links and their counts.
    *   `unique_urls_report.txt`: A refined list of unique URLs.
    *   `warnings.log`: Log file for any issues encountered during the crawling process.

**Justification/Impact:**
This CRQ will provide a foundational tool for expanding the scope of project analysis beyond local files. By building a comprehensive web corpus, we can:
*   Identify and analyze external dependencies and their impact.
*   Understand the broader context of linked resources.
*   Improve the accuracy and completeness of project introspection.
*   Enable future features like link rot detection and content change tracking.