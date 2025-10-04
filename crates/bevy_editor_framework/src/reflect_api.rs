use bevy_reflect::{reflect_trait, Reflect};

#[reflect_trait]
pub trait EditorCompatibility {
    fn is_plugin_compatible(&self) -> bool;
}
