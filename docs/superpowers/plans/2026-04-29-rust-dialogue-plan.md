# Rust Recursive Dialogue System Plan

**Goal:** Implement a high-performance, type-safe, recursive dialogue system in Rust for Godot.

**Architecture:**
- **DialogueLine (Resource):** A recursive Rust struct that stores text, audio, and either a single next line or an array of choices.
- **DialogueChoice (Resource):** Defines a button label and the next node to branch to.
- **DialogueManager (Component):** A Rust `Node` that acts as an isolated dialogue controller. It holds the current `DialogueLine` and exposes signals. The NPC just needs this component added as a child.

**Tech Stack:**
- Rust (`godot-rust` / `gdext`)
- Godot 4.x

---

### Task 1: Update Rust Data Structures
- Modify `rust/src/dialogue/resource.rs`.
- Define `DialogueLine` and `DialogueChoice` with recursive `Option<Gd<DialogueLine>>` fields.

### Task 2: Implement DialogueManager Logic
- Modify `rust/src/dialogue/manager.rs`.
- Add `start_dialogue`, `advance`, `select_choice` logic.
- Emit signals `dialogue_started`, `line_changed`, `dialogue_finished`.

### Task 3: Expose to Godot
- Ensure `manager` and `resources` are exposed to GDExtension.
- Update `lib.rs` exports.

### Task 4: Cleanup BaseNPC & DialogueInteractable
- Remove dialogue logic from `BaseNPC.gd` and `dialogue_interactable.gd`.
- Test attaching `DialogueManager` node in the editor.

---
