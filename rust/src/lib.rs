use godot::prelude::*;

mod world;
mod movement_attributes;
mod ship;
mod components;

pub mod prelude {
    pub(crate) use crate::movement_attributes::*;
    pub(crate) use crate::ship::*;
}

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
