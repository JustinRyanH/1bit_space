use godot::prelude::*;

mod world;
mod movement_attributes;
mod ship;

pub mod prelude {
    pub(crate) use crate::movement_attributes::*;
}

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
