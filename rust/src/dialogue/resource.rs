use godot::prelude::*;
use godot::classes::AudioStream;

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[godot(via = i32)]
pub enum RsDialogueType {
    #[default]
    Static = 0,
    MultiChoice = 1,
    RandStatic = 2,
    RandMultiChoice = 3,
}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct RsDialogueChoice {
    #[base] pub base: Base<Resource>,
    #[export] pub label: GString,
    #[export] pub next_node: Option<Gd<RsDialogueLine>>,
}

#[godot_api]
impl RsDialogueChoice {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct RsDialogueLine {
    #[base] pub base: Base<Resource>,
    #[export] pub text: GString,
    #[export] pub random_lines: Array<Gd<RsDialogueLine>>,
    #[export] pub audio: Option<Gd<AudioStream>>,
    #[export] pub line_type: RsDialogueType,
    #[export] pub next_line: Option<Gd<RsDialogueLine>>,
    #[export] pub choices: Array<Gd<RsDialogueChoice>>,
}

#[godot_api]
impl RsDialogueLine {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct RsDialogueResource {
    #[base] pub base: Base<Resource>,
}

#[godot_api]
impl RsDialogueResource {}
