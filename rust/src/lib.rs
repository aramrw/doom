use godot::prelude::*;

pub mod realm667;
pub use realm667::Realm667Importer;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
