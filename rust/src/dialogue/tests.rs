#[cfg(test)]
mod tests {
    use godot::prelude::*;
    use crate::dialogue::manager::DialogueManager;
    use crate::dialogue::resource::{DialogueNode, DialogueChoice};

    #[test]
    fn test_choice_navigation() {
        // Create a basic dialogue structure: StartNode -> Choice -> TargetNode
        let target_node = Gd::from_init_fn(|base| DialogueNode {
            base,
            dialogue_text: "Target node".into(),
            choices: Array::new(),
            audio: None,
        });

        let choice = Gd::from_init_fn(|base| DialogueChoice {
            base,
            text: "Next".into(),
            next_node: Some(target_node.clone()),
            action_id: "".into(),
        });

        let mut choices = Array::new();
        choices.push(&choice.to_variant());

        let start_node = Gd::from_init_fn(|base| DialogueNode {
            base,
            dialogue_text: "Start node".into(),
            choices,
            audio: None,
        });

        let mut manager = Gd::from_init_fn(|base| DialogueManager {
            base,
            current_node: Some(start_node.clone()),
        });

        // Trigger selection
        manager.bind_mut().select_choice(0);

        // Expectation: current_node is now target_node
        let current = manager.bind().current_node.clone().unwrap();
        assert_eq!(current.bind().dialogue_text.to_string(), "Target node");
    }
}
