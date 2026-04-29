use godot::prelude::*;

pub mod realm667;
pub use realm667::Realm667Importer;
pub mod inventory;
pub mod dialogue {
    pub mod manager;
    pub mod resource;
    #[cfg(test)]
    pub mod tests;
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            godot_print!("GDExtension initialized! (Scene stage)");
        }
    }
}
