use std::fmt::Debug;
use glam::Vec2;
use crate::layout::prelude::*;

pub trait LayoutPositioning: Sized + PartialEq + Debug {
    type Context<'a>;

    // fn pick(point: Vec2, context: &Self::Context<'_>, sizing: &LayoutSizing) -> Option<Self> {
    //     Self::iter_all(context).find(|x| x.rect(context, sizing).contains(point))
    // }

    fn rect(&self, context: &Self::Context<'_>, sizing: &LayoutSizing) -> LayoutRectangle {
        LayoutRectangle {
            top_left: self.location(context, sizing),
            extents: self.size(context, sizing),
        }
    }

    /// The size on an ideal sized canvas
    fn size(&self, context: &Self::Context<'_>, sizing: &LayoutSizing) -> Vec2;

    fn location(&self, context: &Self::Context<'_>, sizing: &LayoutSizing) -> Vec2;
}

pub trait LayoutPositionIterAll: LayoutPositioning {
    fn iter_all(context: &Self::Context<'_>) -> impl Iterator<Item = Self>;
}

pub trait LayoutPositioningWithFont {
    type FontContext<'a>;
    fn font_size(&self, context: &Self::FontContext<'_>) -> f32;
}


pub trait LayoutPositioningWithOrigin: LayoutPositioning {
    fn origin(&self, context: &Self::Context<'_>, sizing: &LayoutSizing) -> Origin;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Origin { //todo maybe use bevy anchor

    TopLeft,
    TopCenter,
    TopRight,

    CenterLeft,
    Center,
    CenterRight,

    BottomLeft,
    BottomCenter,
    BottomRight
}