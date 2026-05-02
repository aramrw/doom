Task: Review changes to nom_parser.rs for strict parsing.

Instructions:
1. Verify that "robust skip" (the `Err(_) => { ... }` block that skipped tokens) has been removed from `parse_actor`.
2. Confirm that `parse_actor` and its helper functions properly propagate errors (`Err`) instead of trying to recover silently.
3. Ensure the parser still passes existing tests in `parser_tests.rs`.
