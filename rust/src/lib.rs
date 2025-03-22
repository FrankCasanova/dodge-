use godot::prelude::*;

mod player;
mod mob;
mod game;
mod hud;

struct RustScriptExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustScriptExtension {}