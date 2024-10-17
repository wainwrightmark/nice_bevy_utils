pub mod insets;
pub mod layout_positioning;
pub mod layout_sizing;
pub mod menu_layout_entity;
pub mod rect;
pub mod spacing;
pub mod scalable_window_size;

pub mod prelude {

    pub use crate::layout::insets::*;
    pub use crate::layout::layout_positioning::*;
    pub use crate::layout::layout_sizing::*;
    pub use crate::layout::menu_layout_entity::*;
    pub use crate::layout::rect::*;
    pub use crate::layout::spacing::*;
    pub use crate::layout::scalable_window_size::*;
}

#[cfg(test)]
mod tests {}
