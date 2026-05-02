Task: Harden the ZScript/DECORATE parser to be strict and fail-fast.

Files: rust/src/realm667/nom_parser.rs

Instructions:
1. Remove all "robust skip" logic that silently skips tokens when parsing fails.
2. Ensure that every parsing function (e.g., parse_actor, parse_states_block, parse_property_or_flag) propagates errors properly using Result/IResult.
3. If parsing an actor's content fails, the parser must return an Err, not continue with partial data.
4. Run tests in rust/ to verify the parser still handles valid inputs correctly.
