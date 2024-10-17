use bevy::prelude::{Deref, Resource};
use crate::layout::insets::*;

#[derive(Debug, Clone, Default, PartialEq, Resource, Deref)]
pub struct InsetsResource(pub Insets);
