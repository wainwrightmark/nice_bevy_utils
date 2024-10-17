use glam::Vec2;

use super::insets::Insets;
use crate::layout::prelude::*;
use crate::window_size::WindowSize;

pub trait ScalableWindowSize {
    const IDEAL_RATIO: f32;
    const IDEAL_WIDTH: f32;

    fn window_size(&self) -> &WindowSize;

    fn insets(&self) -> Insets;

    /// Ideally this should be overridden and cached
    fn layout_sizing(&self) -> LayoutSizing {
        LayoutSizing::from_window_size(
            self.window_size(),
            self.insets(),
            Self::IDEAL_RATIO,
            Self::IDEAL_WIDTH,
        )
    }

    fn scale(&self) -> f32 {
        let window_size = self.window_size();
        (window_size.logical_width / 4.0).min(window_size.logical_height / 8.0)
    }

    fn font_size<T: LayoutPositioningWithFont>(
        &self,
        entity: &T,
        context: &T::FontContext<'_>,
    ) -> f32 {
        self.layout_sizing().font_size(entity, context)
    }

    fn get_rect<T: LayoutPositioning>(
        &self,
        entity: &T,
        context: &T::Context<'_>,
    ) -> LayoutRectangle {
        let window_size = self.window_size();
        let layout_sizing = self.layout_sizing();
        let mut rect = layout_sizing.get_rect(entity, context);

        rect.top_left = Vec2 {
            x: (window_size.logical_width as f32 * -0.5) + rect.top_left.x,
            y: (window_size.logical_height as f32 * 0.5) - rect.top_left.y,
        };

        rect.extents.y *= -1.0;

        rect
    }
}
