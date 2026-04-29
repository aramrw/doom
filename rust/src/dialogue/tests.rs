#[cfg(test)]
mod tests {
    use godot::prelude::*;
    use crate::dialogue::manager::DialogueManager;
    use crate::dialogue::resource::{DialogueResource, DialogueLine, DialogueChoice, DialogueType};

    #[test]
    fn test_static_navigation() {
        let line2 = Gd::from_init_fn(|base| DialogueLine {
            base,
            text: "Line 2".into(),
            audio: None,
            line_type: DialogueType::Static,
            next_line_index: -1,
            choices: Array::new(),
            action_id: "".into(),
        });

        let line1 = Gd::from_init_fn(|base| DialogueLine {
            base,
            text: "Line 1".into(),
            audio: None,
            line_type: DialogueType::Static,
            next_line_index: 1,
            choices: Array::new(),
            action_id: "".into(),
        });

        let resource = Gd::from_init_fn(|base| DialogueResource {
            base,
            lines: array![&line1.to_variant(), &line2.to_variant()],
            start_index: 0,
        });

        let mut manager = Gd::from_init_fn(|base| DialogueManager {
            base,
            resource: None,
            current_line_index: -1,
        });

        manager.bind_mut().start_dialogue(resource);
        assert_eq!(manager.bind().current_line_index, 0);

        manager.bind_mut().advance();
        assert_eq!(manager.bind().current_line_index, 1);
    }

    #[test]
    fn test_choice_navigation() {
        let target_line = Gd::from_init_fn(|base| DialogueLine {
            base,
            text: "Target".into(),
            audio: None,
            line_type: DialogueType::Static,
            next_line_index: -1,
            choices: Array::new(),
            action_id: "".into(),
        });

        let choice = Gd::from_init_fn(|base| DialogueChoice {
            base,
            text: "Pick me".into(),
            next_line_index: 1,
            action_id: "".into(),
        });

        let start_line = Gd::from_init_fn(|base| DialogueLine {
            base,
            text: "Start".into(),
            audio: None,
            line_type: DialogueType::Choice,
            next_line_index: -1,
            choices: array![&choice.to_variant()],
            action_id: "".into(),
        });

        let resource = Gd::from_init_fn(|base| DialogueResource {
            base,
            lines: array![&start_line.to_variant(), &target_line.to_variant()],
            start_index: 0,
        });

        let mut manager = Gd::from_init_fn(|base| DialogueManager {
            base,
            resource: None,
            current_line_index: -1,
        });

        manager.bind_mut().start_dialogue(resource);
        manager.bind_mut().select_choice(0);

        assert_eq!(manager.bind().current_line_index, 1);
    }
}
