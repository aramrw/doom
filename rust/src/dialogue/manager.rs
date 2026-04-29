use godot::prelude::*;
use crate::dialogue::resource::DialogueNode;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct DialogueManager {
    pub base: Base<Node>,
    pub current_node: Option<Gd<DialogueNode>>,
}

#[godot_api]
impl DialogueManager {
    #[signal]
    fn dialogue_started(node: Gd<DialogueNode>);
    #[signal]
    fn node_changed(node: Gd<DialogueNode>);
    #[signal]
    fn dialogue_finished();
    #[signal]
    fn action_triggered(action_id: GString);

    #[func]
    pub fn start_dialogue(&mut self, start_node: Gd<DialogueNode>) {
        self.current_node = Some(start_node.clone());
        self.base_mut().emit_signal("dialogue_started", &[start_node.to_variant()]);
    }

    #[func]
    pub fn select_choice(&mut self, index: i32) {
        let Some(node) = self.current_node.clone() else { return };

        let choices = node.bind().choices.clone();

        if index >= 0 && index < choices.len() as i32 {
            if let Some(choice_variant) = choices.get(index as usize) {
                if let Ok(choice_gd) = choice_variant.try_to::<Gd<crate::dialogue::resource::DialogueChoice>>() {
                    let choice = choice_gd.bind();

                    // Emit action if present
                    if !choice.action_id.is_empty() {
                        self.base_mut().emit_signal("action_triggered", &[choice.action_id.to_variant()]);
                    }

                    if let Some(next) = choice.next_node.clone() {
                        self.current_node = Some(next.clone());
                        self.base_mut().emit_signal("node_changed", &[next.to_variant()]);
                    } else {
                        self.current_node = None;
                        self.base_mut().emit_signal("dialogue_finished", &[]);
                    }
                }
            }
        }
    }
}
