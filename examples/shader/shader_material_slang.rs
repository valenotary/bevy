//! An example that demonstrates Slang support for custom Bevy materials.
//! This requires the following:
//! - `slangc` to findable on PATH (usually, just need to instake Vulkan SDK on system)
//! - `shader_slang` feature enabled
//! - a slang source file can define multiple entry points -- since the [Material] will need to 
//!   be able to select between them, just pass in the same file into each stage your shader has an 
//!   entry point for.

//! TODO: implement!