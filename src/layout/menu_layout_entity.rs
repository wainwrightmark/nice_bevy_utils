use std::fmt::Debug;

use crate::layout::prelude::*;
use glam::Vec2;

pub trait MenuButtonPositioning: Debug + PartialEq + Sized {
    type MenuButtonsContext;
    fn index(&self) -> usize;
    fn count(context: &Self::MenuButtonsContext) -> usize;

    fn iter_all(context: &Self::MenuButtonsContext) -> impl Iterator<Item = Self>;

    const CONSTANTS: MenuConstants;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuConstants {
    pub font_size: f32,
    pub button_width: f32,
    pub button_height: f32,
    pub button_spacing: f32,
    pub ideal_width: f32,
    pub ideal_height: f32,
    pub top_bar_height: f32,
    pub virtual_children: usize,
}

impl<T: MenuButtonPositioning> LayoutPositioningWithFont for T {
    type FontContext<'a> = ();

    fn font_size(&self, _context: &Self::FontContext<'_>) -> f32 {
        <T as MenuButtonPositioning>::CONSTANTS.font_size
    }
}

impl<T: MenuButtonPositioning> LayoutPositioning for T {
    type Context<'a> = T::MenuButtonsContext;

    fn size(&self, _context: &Self::Context<'_>, _sizing: &LayoutSizing) -> Vec2 {
        Vec2 {
            x: Self::CONSTANTS.button_width,
            y: Self::CONSTANTS.button_height,
        }
    }

    fn location(&self, context: &Self::Context<'_>, _sizing: &LayoutSizing) -> Vec2 {
        Vec2 {
            x: (Self::CONSTANTS.ideal_width - Self::CONSTANTS.button_width) / 2.,
            y: Self::CONSTANTS.top_bar_height
                + Spacing::Centre.apply(
                    Self::CONSTANTS.ideal_height - Self::CONSTANTS.top_bar_height,
                    (Self::CONSTANTS.button_height + Self::CONSTANTS.button_spacing) * 0.5,
                    Self::CONSTANTS.virtual_children * 2,
                    ((self.index() * 2) + Self::CONSTANTS.virtual_children
                        - (Self::count(&context) + 1)) as f32,
                ),
        }
    }
}

impl<T: MenuButtonPositioning> LayoutPositionIterAll for T {
    fn iter_all(context: &Self::Context<'_>) -> impl Iterator<Item = Self> {
        Self::iter_all(&context)
    }
}
