use godot::prelude::*;
use godot::classes::Area3D;
use crate::dialogue::resource::{DialogueLine, DialogueType};

#[derive(GodotClass)]
#[class(base=Area3D, init)]
pub struct DialogueManager {
    pub base: Base<Area3D>,
    #[export] pub root_node: Option<Gd<DialogueLine>>,
    pub current_line: Option<Gd<DialogueLine>>,
}

#[godot_api]
impl DialogueManager {
    #[signal] fn dialogue_started(line: Gd<DialogueLine>);
    #[signal] fn line_changed(line: Gd<DialogueLine>);
    #[signal] fn dialogue_finished();

    #[func]
    pub fn interact(&mut self) {
        if let Some(root) = self.root_node.clone() {
            self.current_line = Some(root.clone());
            self.base_mut().emit_signal("dialogue_started", &[root.to_variant()]);
        }
    }

    #[func]
    pub fn advance(&mut self) {
        let Some(line) = self.current_line.clone() else { return };
        let line_bind = line.bind();
        
        if line_bind.line_type == DialogueType::Static {
            if let Some(next) = line_bind.next_line.clone() {
                self.current_line = Some(next.clone());
                self.base_mut().emit_signal("line_changed", &[next.to_variant()]);
            } else {
                self.finish();
            }
        }
    }

    #[func]
    pub fn select_choice(&mut self, index: i32) {
        let Some(line) = self.current_line.clone() else { return };
        let line_bind = line.bind();
        
        if line_bind.line_type == DialogueType::MultiChoice {
            if let Some(choice_gd) = line_bind.choices.get(index as usize) {
                if let Some(next) = choice_gd.bind().next_node.clone() {
                    self.current_line = Some(next.clone());
                    self.base_mut().emit_signal("line_changed", &[next.to_variant()]);
                } else {
                    self.finish();
                }
            }
        }
    }

    #[func]
    pub fn finish(&mut self) {
        self.current_line = None;
        self.base_mut().emit_signal("dialogue_finished", &[]);
    }
}
