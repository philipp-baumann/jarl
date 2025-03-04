pub(crate) mod redundant_equals;

#[cfg(test)]
mod tests {
    use crate::utils_test::*;

    #[test]
    fn test_lint_redundant_equals() {
        use insta::assert_snapshot;
        let expected_message = "Using == on a logical vector is";

        assert!(expect_lint(
            "a == TRUE",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "TRUE == a",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "a == FALSE",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "FALSE == a",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "a != TRUE",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "TRUE != a",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "a != FALSE",
            expected_message,
            "redundant_equals"
        ));
        assert!(expect_lint(
            "FALSE != a",
            expected_message,
            "redundant_equals"
        ));

        assert_snapshot!(
            "fix_output",
            get_fixed_text(
                vec![
                    "a == TRUE",
                    "TRUE == a",
                    "a == FALSE",
                    "FALSE == a",
                    "a != TRUE",
                    "TRUE != a",
                    "a != FALSE",
                    "FALSE != a",
                    "foo(a(b = 1)) == TRUE"
                ],
                "redundant_equals"
            )
        );
    }

    #[test]
    fn test_no_lint_redundant_equals() {
        assert!(no_lint("x == 1", "redundant_equals"));
        assert!(no_lint("x == 'TRUE'", "redundant_equals"));
        assert!(no_lint("x == 'FALSE'", "redundant_equals"));
        assert!(no_lint("x > 1", "redundant_equals"));
    }
}
