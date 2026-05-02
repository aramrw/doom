# Atomic Resource Generation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Modify `rust/src/realm667/resource_gen.rs` to generate resources atomically in a temporary directory before moving them to the destination.

**Architecture:** 
1. `generate_*` functions will create a `temp_gen` directory inside `actor_root`.
2. Generate all assets into `temp_gen`.
3. If successful, move all contents from `temp_gen` to `actor_root`.
4. Use a RAII-like pattern or explicit cleanup on error to ensure `temp_gen` is deleted if an error occurs.
5. `std::fs::rename` is atomic on most platforms.

**Tech Stack:** Rust (std::fs, std::path)

---

### Task 1: Create atomic helper function

**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Implement `with_atomic_gen` helper**
We need a helper that takes a closure that generates assets into a temporary directory, and handles move/cleanup.

```rust
fn with_atomic_gen<F>(actor_root: &Path, f: F) -> std::io::Result<()>
where F: FnOnce(&Path) -> std::io::Result<()>
{
    let temp_dir = actor_root.join("temp_gen");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    let result = f(&temp_dir);

    if result.is_ok() {
        // Move files
        for entry in fs::read_dir(&temp_dir)? {
            let entry = entry?;
            let dest = actor_root.join(entry.file_name());
            fs::rename(entry.path(), dest)?;
        }
    }

    fs::remove_dir_all(&temp_dir)?;
    result
}
```

### Task 2: Apply to `generate_weapon_resources`

**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Wrap generation logic in `with_atomic_gen`**
Refactor the body of `generate_weapon_resources` to use the helper.

### Task 3: Apply to `generate_enemy_resources`, `generate_projectile_resources`, etc.

**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Apply to all generation methods.**
- `generate_enemy_resources`
- `generate_projectile_resources`
- `generate_prop_resources`
- `generate_item_resources`

---
