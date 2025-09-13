use submodules::crq_state_recognizer::is_review_skipped_due_to_size_limit;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_review_skipped_due_to_size_limit_positive_max_files_limit() {
        let content = r#"
        <!-- This is an auto-generated comment: skip review by coderabbit.ai -->
        > [!IMPORTANT]
        > ## Review skipped
        > 
        > More than 25% of the files skipped due to max files limit.
        > 
        > Please upgrade to Pro plan to get higher limits.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), true);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_positive_exceeds_size_limit() {
        let content = r#"
        <!-- This is an auto-generated comment: skip review by coderabbit.ai -->
        > [!IMPORTANT]
        > ## Review skipped
        > 
        > The changes exceed size limit for review.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), true);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_positive_too_large() {
        let content = r#"
        <!-- This is an auto-generated comment: skip review by coderabbit.ai -->
        > [!IMPORTANT]
        > ## Review skipped
        > 
        > The issue is too large for an automated review.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), true);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_positive_low_quality_review() {
        let content = r#"
        Review skipped
        The review is being skipped to prevent a low-quality review.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), true);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_negative_normal_review() {
        let content = r#"
        <!-- This is an auto-generated comment: summarize by coderabbit.ai -->
        > [!NOTE]
        > ## Review Summary
        > 
        > This PR introduces a new feature.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), false);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_negative_rate_limit() {
        let content = r#"
        <!-- This is an auto-generated comment: skip review by coderabbit.ai -->
        > [!WARNING]
        > ## Rate Limit Exceeded
        > 
        > Please try again later.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), false);
    }

    #[test]
    fn test_is_review_skipped_due_to_size_limit_negative_no_keywords() {
        let content = r#"
        This is some random content without any relevant keywords.
        "#;
        assert_eq!(is_review_skipped_due_to_size_limit(content), false);
    }
}
