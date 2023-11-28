use godot::prelude::*;

mod ship;
mod world;

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
