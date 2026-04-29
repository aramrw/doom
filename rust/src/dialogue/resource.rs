use godot::prelude::*;
use godot::classes::AudioStream;

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct DialogueChoice {
    #[base]
    pub base: Base<Resource>,

    #[export]
    pub text: GString,

    #[export]
    pub next_node: Option<Gd<DialogueNode>>,

    #[export]
    pub action_id: GString,
}

#[godot_api]
impl DialogueChoice {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct DialogueNode {
    #[base]
    pub base: Base<Resource>,

    #[export]
    pub dialogue_text: GString,

    #[export]
    pub choices: Array<Variant>,

    #[export]
    pub audio: Option<Gd<AudioStream>>,
}

#[godot_api]
impl DialogueNode {}
