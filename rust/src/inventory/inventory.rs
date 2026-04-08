use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct Inventory {
    base: Base<Node>,
}

#[godot_api]
impl INode for Inventory {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}
