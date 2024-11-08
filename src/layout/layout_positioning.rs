use crate::layout::prelude::*;
use glam::Vec2;
use std::fmt::Debug;

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

pub trait HasFontSize {
    type FontContext<'a>;
    fn font_size(&self, context: &Self::FontContext<'_>) -> f32;
}

pub trait HasOrigin: LayoutPositioning {
    fn origin(&self, context: &Self::Context<'_>, sizing: &LayoutSizing) -> Origin;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Origin {
    //todo maybe use bevy anchor
    TopLeft,
    TopCenter,
    TopRight,

    CenterLeft,
    Center,
    CenterRight,

    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[cfg(test)]
mod tests {
    use crate::layout::layout_positioning::*;
    extern crate self as nice_bevy_utils;

    #[test]
    pub fn test_has_font_size() {
        const FONT_SIZE: f32 = 42.0;

        #[derive(nice_bevy_utils_macro::HasFontSize)]
        #[font_size(42.0)]
        pub struct FooLiteral;

        #[derive(nice_bevy_utils_macro::HasFontSize)]
        #[font_size(FONT_SIZE)]
        pub struct FooConst;

        #[derive(nice_bevy_utils_macro::HasFontSize)]
        #[font_size(20.0 + 22.0)]
        pub struct FooSum;

        assert_eq!(FooLiteral.font_size(&()), 42.0);
        assert_eq!(FooConst.font_size(&()), 42.0);
        assert_eq!(FooSum.font_size(&()), 42.0);
    }

    #[test]
    pub fn test_layout_positioning() {
        #[derive(
            Debug,
            PartialEq,
            nice_bevy_utils_macro::HasOrigin,
            nice_bevy_utils_macro::LayoutPositioning,
        )]
        #[origin(Origin::TopCenter)]
        #[width(1.0)]
        #[height(2.0)]
        #[left(3.0)]
        #[top(4.0)]

        pub struct Foo;

        let rect = Foo.rect(
            &(),
            &LayoutSizing::from_page_size(Vec2::ZERO, 1.0, 256.0, Insets::new(0.0)),
        );

        assert_eq!(rect, LayoutRectangle{ extents: Vec2 { x: 1.0, y: 2.0 }, top_left: Vec2{x: 3.0, y:4.0}});
    }

    #[test]
    pub fn test_has_origin() {
        #[derive(
            Debug,
            PartialEq,
            nice_bevy_utils_macro::HasOrigin,
            nice_bevy_utils_macro::LayoutPositioning,
        )]
        #[origin(Origin::TopCenter)]
        #[width(0.0)]
        #[height(0.0)]
        #[top(0.0)]
        #[left(0.0)]
        pub struct Foo;

        assert_eq!(
            Foo.origin(
                &(),
                &LayoutSizing::from_page_size(Vec2::ZERO, 0.0, 0.0, Insets::new(0.0))
            ),
            Origin::TopCenter
        )
    }
}
