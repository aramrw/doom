# Enemy Parser Improvement Implementation Plan

**Goal:** Improve the GZDoom-to-Godot enemy parsing logic by adding unit tests to the Rust parser to ensure accurate sprite sheet generation, Tscn creation, and enemy configuration.

**Architecture:**
- Leverage the existing `rust/src/realm667` parser and resource generator.
- Add comprehensive test cases in `rust/src/realm667/parser_tests.rs` (or a dedicated test file) covering complex enemy definitions.
- Refactor the parsing logic in `rust/src/realm667/nom_parser.rs` if needed to support features currently missing or bugged for enemies.
- Ensure enemy resource generation produces a functional TSCN mirroring the successful `cultist` pattern.

**Tech Stack:**
- Rust (Cargo)
- Godot 4.x
- Nom (Parser combinator library)

---

### Task 1: Add unit tests for enemy parsing
- Modify: `rust/src/realm667/parser_tests.rs`
- Add tests for:
  - Complex enemy state definitions with multiple sub-actions
  - Parsing of all enemy properties (Health, Speed, PainChance, MeleeRange, Radius, Height, etc.)
  - Parsing of flags (NOGRAVITY, etc.)

### Task 2: Verify `nom_parser` implementation
- Modify: `rust/src/realm667/nom_parser.rs`
- Ensure the parser correctly extracts all required fields from the enemy actor definitions.
- Run tests: `cd rust && cargo test`

### Task 3: Improve Enemy Resource Generation
- Modify: `rust/src/realm667/resource_gen.rs`
- Refactor `generate_enemy_resources` to better match the successful `cultist` pattern:
  - Correct scaling of Doom units/tics.
  - Better handling of animation frames and transitions.
  - Proper binding of sounds and projectiles.

### Task 4: Integration testing
- Run the importer on a test GZDoom enemy (e.g., a simple one like 'PlagueRat.pk3') and check the output directory for correctness.
- Validate:
  - `spriteframes.tres` is generated correctly.
  - `*.tscn` contains valid node references and scripts.
