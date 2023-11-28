use godot::prelude::*;

mod ship;
mod world;
mod movement_attributes;

pub mod prelude {
    pub(crate) use crate::movement_attributes::*;
}

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
