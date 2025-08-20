pub(crate) mod grepv;

#[cfg(test)]
mod tests {
    use crate::utils_test::*;

    #[test]
    fn test_no_lint_grepv() {
        expect_no_lint("grep('i', x)", "grepv");
        expect_no_lint("grep(pattern = 'i', x)", "grepv");
        expect_no_lint("grep('i', x, TRUE, TRUE)", "grepv");
    }

    #[test]
    fn test_lint_grepv() {
        use insta::assert_snapshot;

        let expected_message = "Use `grepv(...)`";
        has_lint_with_version(
            "grep('i', x, value = TRUE)",
            expected_message,
            "grepv",
            "4.5",
        );
        has_lint_with_version(
            "grep('i', x, TRUE, TRUE, TRUE)",
            expected_message,
            "grepv",
            "4.5",
        );
        has_lint_with_version(
            "grep('i', x, TRUE, TRUE, TRUE, value = TRUE)",
            expected_message,
            "grepv",
            "4.5",
        );
        assert_snapshot!(
            "fix_output",
            get_fixed_text_with_version(
                vec![
                    "grep('i', x, value = TRUE)",
                    "grep('i', x, TRUE, TRUE, TRUE)",
                    "grep('i', x, TRUE, TRUE, TRUE, value = TRUE)",
                    // Keep the name of other args
                    "grep(pattern = 'i', x, value = TRUE)",
                    // Wrong code but no panic
                    "grep(value = TRUE)",
                ],
                "grepv",
                "4.5"
            )
        );
    }
}
