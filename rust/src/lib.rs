use godot::prelude::*;

mod ship;
mod world;
mod movement_attributes;

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
