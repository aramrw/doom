# Dialogue System Issue Investigation

## Observations
1. `any_dialogue.gd` (BaseDialogue) and `dialogue_interactable.gd` (DialogueInteractable) both create and add a `DialogueManager` child node in `_ready`.
2. `any_dialogue.gd` handles NPCs, while `dialogue_interactable.gd` is for generic interactables.
3. Both add their manager to the group "DialogueManager".
4. Multiple managers in the "DialogueManager" group might cause confusion for the UI which calls `get_tree().get_first_node_in_group("DialogueManager")`.
5. The dialogue flow in Rust's `DialogueManager` (`rust/src/dialogue/manager.rs`) handles static lines and choices.

## Suspected Issue
- UI finding the *wrong* `DialogueManager` instance if multiple interactables or NPCs exist.
- NPC state machine might need better handling during talking.

## Questions for User
- What exactly is the "fix" you're looking for? Are you encountering a specific bug (e.g., UI not opening, dialogue not advancing, wrong manager)?
- Is there a specific dialogue behavior you are trying to implement?
