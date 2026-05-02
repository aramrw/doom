use crate::dialogue::resource::{RsDialogueLine, RsDialogueType};
use godot::classes::Area3D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area3D, init)]
pub struct RsDialogueManager {
    pub base: Base<Area3D>,
    #[export]
    pub root_node: Option<Gd<RsDialogueLine>>,
    pub current_line: Option<Gd<RsDialogueLine>>,
}

#[godot_api]
impl RsDialogueManager {
    #[signal]
    fn dialogue_started(line: Gd<RsDialogueLine>);
    #[signal]
    fn line_changed(line: Gd<RsDialogueLine>);
    #[signal]
    fn dialogue_finished();

    #[func]
    pub fn interact(&mut self) {
        if let Some(root) = self.root_node.clone() {
            self.current_line = Some(root.clone());
            self.base_mut()
                .emit_signal("dialogue_started", &[root.to_variant()]);
        }
    }

    #[func]
    pub fn advance(&mut self) {
        let Some(line) = self.current_line.clone() else {
            return;
        };
        let line_bind = line.bind();

        if line_bind.line_type == RsDialogueType::Static || line_bind.line_type == RsDialogueType::RandStatic {
            if let Some(next) = line_bind.next_line.clone() {
                self.current_line = Some(next.clone());
                self.base_mut()
                    .emit_signal("line_changed", &[next.to_variant()]);
            } else {
                self.finish();
            }
        }
    }

    #[func]
    pub fn select_choice(&mut self, index: i32) {
        let Some(line) = self.current_line.clone() else {
            return;
        };
        let line_bind = line.bind();

        if line_bind.line_type == RsDialogueType::MultiChoice || line_bind.line_type == RsDialogueType::RandMultiChoice {
            if let Some(choice_gd) = line_bind.choices.get(index as usize) {
                if let Some(next) = choice_gd.bind().next_node.clone() {
                    self.current_line = Some(next.clone());
                    self.base_mut()
                        .emit_signal("line_changed", &[next.to_variant()]);
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
