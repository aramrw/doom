# Enemy Parser and Generator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement strict, deterministic GZDoom enemy parsing and robust Godot resource generation, enabling one-click import of enemies like the Cultist.

**Architecture:**
- **Strict Parser:** Modify `nom_parser.rs` to return `Result` with explicit errors for any unknown tokens or ambiguous constructs, ensuring the parser fails fast and noisily.
- **Fail-Fast Generator:** Update `resource_gen.rs` to perform atomic resource generation per actor, deleting temporary files on failure.
- **Cultist Integration Test:** Implement a formal integration test that imports `cultist.pk3` and validates generated TSCN/Resources.

**Tech Stack:**
- Rust (Cargo, Nom)
- Godot 4.x

---

### Task 1: Hardening `nom_parser`
- Modify: `rust/src/realm667/nom_parser.rs`
- Goal: Implement strict error handling and fail-fast for unknown tokens or constructs.

- [ ] **Step 1: Replace `Result` with explicit error types in parser**
  Change parser functions (like `parse_actor`, `parse_states_block`) to explicitly return `Err(String)` when parsing fails instead of returning partial results.

- [ ] **Step 2: Add failure conditions**
  Add explicit checks for unknown flags or property keys. If `parse_property_or_flag` hits an unknown item, return an error rather than skipping.

- [ ] **Step 3: Commit**

```bash
git add rust/src/realm667/nom_parser.rs
git commit -m "feat: implement strict error handling in parser"
```

### Task 2: Atomic Resource Generation in `ResourceGenerator`
- Modify: `rust/src/realm667/resource_gen.rs`
- Goal: Ensure partial imports are cleaned up on error.

- [ ] **Step 1: Implement directory-level rollback**
  Modify `generate_enemy_resources` to create files in a `temp_gen/` subfolder first. If the process succeeds, move the folder to the final destination; if it fails, remove `temp_gen/`.

- [ ] **Step 2: Commit**

```bash
git add rust/src/realm667/resource_gen.rs
git commit -m "feat: add atomic resource generation"
```

### Task 3: Integration Test: Cultist Import
- Create: `rust/src/realm667/cultist_integration_test.rs`
- Modify: `rust/src/realm667/mod.rs` (to register the test)

- [ ] **Step 1: Add integration test file**
  Create a test that invokes `import_pk3` on `cultist.pk3` and validates the resulting `cultist.tscn` exists and contains correct node structure.

- [ ] **Step 2: Register test**
  Add `pub mod cultist_integration_test;` to `rust/src/realm667/mod.rs`.

- [ ] **Step 3: Run and Validate**
  Run `cd rust && cargo test --test cultist_integration_test`.

- [ ] **Step 4: Commit**

```bash
git add rust/src/realm667/cultist_integration_test.rs rust/src/realm667/mod.rs
git commit -m "feat: add integration test for cultist pk3"
```
