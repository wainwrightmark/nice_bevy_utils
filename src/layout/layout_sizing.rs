use crate::layout::prelude::*;
use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutSizing {
    pub size_ratio: f32,
    pub left_pad: f32,
    pub bottom_pad: f32,
    pub insets: Insets
}

impl Default for LayoutSizing {
    fn default() -> Self {
        Self {
            size_ratio: 1.0,
            left_pad: 0.0,
            bottom_pad: 0.0,
            insets: Insets::default()
        }
    }
}

impl LayoutSizing {
    pub fn from_page_size(page_size: Vec2, ideal_ratio: f32, ideal_width: f32, insets: Insets) -> Self {
        let ratio = page_size.x / page_size.y;

        let used_y: f32;
        let used_x: f32;

        if ratio >= ideal_ratio {
            // There is additional x, so just left pad everything
            used_y = page_size.y;
            used_x = page_size.y * ideal_ratio;
        } else {
            // There is additional y, so don't use the bottom area
            used_x = page_size.x;
            used_y = page_size.x / ideal_ratio;
        }

        let left_pad = ((page_size.x - used_x) / 2.).round();
        let bottom_pad = page_size.y - used_y;
        let size_ratio = used_x / ideal_width;

        Self {
            size_ratio,
            left_pad,
            bottom_pad,
            insets
        }
    }

    pub fn get_size<T: LayoutPositioning>(&self, entity: &T, context: &T::Context<'_>) -> Vec2 {
        let v2: Vec2 = entity.size(context, self);
        v2 * self.size_ratio
    }

    pub fn get_location<T: LayoutPositioning>(
        &self,
        entity: &T,
        context: &T::Context<'_>,
    ) -> glam::Vec2 {
        let Vec2 { x, y } = entity.location(context, self);

        Vec2 {
            x: self.left_pad + self.size_ratio * x,
            y: self.size_ratio * y,
        }
    }

    pub fn get_rect<T: LayoutPositioning>(
        &self,
        entity: &T,
        context: &T::Context<'_>,
    ) -> LayoutRectangle {
        LayoutRectangle {
            top_left: self.get_location(entity, context),
            extents: self.get_size(entity, context),
        }
    }

    pub fn font_size<T: LayoutPositioningWithFont>(
        &self,
        entity: &T,
        context: &T::FontContext<'_>,
    ) -> f32 {
        const FONT_INTERVAL: f32 = 1.0;
        let base_size = entity.font_size(context);

        (self.size_ratio * base_size / FONT_INTERVAL).floor() * FONT_INTERVAL
    }
}
