# Dialogue System Design & Implementation Plan (Rust)

## Goal
Design a modular, Souls-like dialogue system in Rust for Godot 4.

## Design Approach
*   **State:** Dialogue trees/graphs stored as Godot Resources.
*   **Logic:** A `DialogueManager` in Rust acting as an autoload (or component) to process current node state, handle choices, and execute side-effects.
*   **UI Communication:** Signal-based. Rust emits signals (e.g., "DialogueStarted", "NodeChanged"). The UI binds these to its control nodes.

## Implementation Steps

1.  **Define Dialogue Data Structure (Rust):** Define `DialogueNode`, `DialogueChoice`, `DialogueAction` structs.
2.  **Resource Mapping:** Create Godot-compatible wrappers or Resource types to allow designing trees inside the editor.
3.  **Core Dialogue Processor (Rust):** Implement `DialogueProcessor` to track state, handle choice selection, and trigger actions.
4.  **UI Bridge:** Implement the Godot-Rust interface to expose the current dialogue state to GDScript UI nodes.
5.  **Souls-like Integration:** Add logic for conditional dialogue based on inventory/quest state.
