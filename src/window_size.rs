use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized, WindowScaleFactorChanged};

// Track window size an automatically adjust UI scale
#[derive(Default)]
pub struct WindowSizePlugin;

impl Plugin for WindowSizePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowSize>();

        app.add_systems(Update, handle_window_resized);
        app.add_systems(PreUpdate, touch_text_2d_on_window_size_changed.run_if(|ws: Res<WindowSize>|ws.is_changed()));
    }
}

pub trait Breakpoints: Default + Send + Sync + 'static {
    /// The scale to multiply the height and width by.
    /// The object scale will be the reciprocal of this.
    /// e.g. if the size scale is 0.5, objects will appear twice as big
    fn size_scale(raw_window_width: f32, raw_window_height: f32) -> f32;
}

#[derive(Debug, PartialEq, Resource)]
pub struct WindowSize {
    pub logical_width: f32,
    pub logical_height: f32,
    pub scale_factor: f32,
}

impl WindowSize {
    pub fn to_window_resolution(&self) -> bevy::window::WindowResolution {
        let mut res = bevy::window::WindowResolution::default();
        res.set_scale_factor(self.scale_factor);
        res.set(self.logical_width, self.logical_height);
        res
    }

    pub fn clamp_to_resize_constraints(&mut self, constraints: &WindowResizeConstraints) {
        self.logical_width = self
            .logical_width
            .clamp(constraints.min_width, constraints.max_width);
        self.logical_height = self
            .logical_height
            .clamp(constraints.min_height, constraints.max_height);
    }
}

impl<'w> From<&'w Window> for WindowSize {
    fn from(value: &'w Window) -> Self {
        Self {
            logical_height: value.height(),
            logical_width: value.width(),
            scale_factor: value.scale_factor(),
        }
    }
}

impl FromWorld for WindowSize {
    fn from_world(world: &mut World) -> Self {
        let mut query = world.query_filtered::<&Window, With<PrimaryWindow>>();
        let window = query.single(world);

        window.into()
    }
}

pub fn handle_window_resized(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_scale_factor_events: EventReader<WindowScaleFactorChanged>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>,
) {
    for _ in window_resized_events.read() {
        let window = window_query.single();
        if window_size.set_if_neq(window.into()) {
            info!("1 Window Resized: {window_size:?}")
        }
    }

    for _ in window_scale_factor_events.read() {
        let window = window_query.single();
        if window_size.set_if_neq(window.into()) {
            info!("1 Scale factor changed: {window_size:?}")
        }
    }
}


fn touch_text_2d_on_window_size_changed(ws: Res<WindowSize>, mut query: Query<&mut bevy::text::Text2dBounds>){
    if ws.is_changed(){
        for mut x in query.iter_mut(){
            x.set_changed();
        }
    }
}