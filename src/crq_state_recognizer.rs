pub fn is_review_skipped_due_to_size_limit(coderabbitai_response_content: &str) -> bool {
    let lower_content = coderabbitai_response_content.to_lowercase();

    // Keywords and phrases indicating a skipped review due to size
    let size_limit_keywords = [
        "review skipped",
        "max files limit",
        "exceeds size limit",
        "too large",
        "low-quality review", // Often associated with skipped reviews due to size
    ];

    // Check if any of the keywords are present in the content
    size_limit_keywords.iter().any(|&keyword| lower_content.contains(keyword))
}
