use godot::prelude::*;

mod world;
mod ship;
mod components;
mod resources;
mod systems;
mod managers;

pub mod prelude {
    pub(crate) use crate::resources::*;
    pub(crate) use crate::ship::*;
}

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
