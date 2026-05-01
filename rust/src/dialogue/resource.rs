use godot::prelude::*;
use godot::classes::AudioStream;

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[godot(via = i32)]
pub enum DialogueType {
    #[default]
    Static = 0,
    MultiChoice = 1,
    RandStatic = 2,
    RandMultiChoice = 3,
}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct DialogueChoice {
    #[base] pub base: Base<Resource>,
    #[export] pub label: GString,
    #[export] pub next_node: Option<Gd<DialogueLine>>,
}

#[godot_api]
impl DialogueChoice {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct DialogueLine {
    #[base] pub base: Base<Resource>,
    #[export] pub text: GString,
    #[export] pub random_lines: Array<Gd<DialogueLine>>,
    #[export] pub audio: Option<Gd<AudioStream>>,
    #[export] pub line_type: DialogueType,
    #[export] pub next_line: Option<Gd<DialogueLine>>,
    #[export] pub choices: Array<Gd<DialogueChoice>>,
}

#[godot_api]
impl DialogueLine {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct DialogueResource {
    #[base] pub base: Base<Resource>,
}

#[godot_api]
impl DialogueResource {}
