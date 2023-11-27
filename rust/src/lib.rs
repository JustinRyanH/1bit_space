use godot::prelude::*;

mod ship;

struct OneBitSpace;

#[gdextension]
unsafe impl ExtensionLibrary for OneBitSpace {}
