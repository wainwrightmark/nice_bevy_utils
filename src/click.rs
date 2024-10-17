use bevy::{ecs::system::SystemState, prelude::*, sprite::Anchor, window::PrimaryWindow};

use std::sync::Arc;

use crate::window_size::WindowSize;

pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>();

        app.init_resource::<PressedEntity>();

        app.add_systems(
            Update,
            handle_click_events.run_if(|x: EventReader<ClickEvent>| !x.is_empty()),
        );
        app.add_systems(Update, handle_mouse_clicks.before(handle_click_events));
        app.add_systems(Update, handle_touches.before(handle_click_events));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputDevice {
    Mouse,
    Touch,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClickPhase {
    Start,
    Move,
    End,
}

impl ClickPhase {
    pub fn is_start(&self) -> bool {
        *self == ClickPhase::Start
    }

    pub fn is_move(&self) -> bool {
        *self == ClickPhase::Move
    }

    pub fn is_end(&self) -> bool {
        *self == ClickPhase::End
    }
}

pub trait ClickAction: Sync + Send + core::fmt::Debug + 'static {
    fn on_click(
        &self,
        click_stage: ClickPhase,
        input_device: InputDevice,
        distance_ratio: Vec2,
        world: &mut World,
    );
}

#[derive(Debug, Event, Clone)]
struct ClickEvent {
    on_click: Arc<dyn ClickAction>,
    input_device: InputDevice,
    click_phase: ClickPhase,

    /// distance from the centre of the element with each component in 0.0..1.0
    scaled_distance: Vec2,
}

fn handle_click_events(world: &mut World, events: &mut SystemState<EventReader<ClickEvent>>) {
    while let Some(ev) = events.get(world).read().next() {
        let ev = ev.clone();
        ev.on_click
            .as_ref()
            .on_click(ev.click_phase, ev.input_device, ev.scaled_distance, world);
    }
}

#[derive(Debug, Component, Clone)]
pub struct ClickableComponent {
    pub on_click: Arc<dyn ClickAction>,
    pub extents_abs: Vec2,
    pub anchor: Anchor,
    pub enabled: bool,
    //todo different shapes
}

impl ClickableComponent {
    /// If the click is inside the component, gets the distance from the centre scaled to the size of the component
    pub fn get_click_distance(&self, position: Vec2, transform: &GlobalTransform) -> Option<Vec2> {
        let (scale, _rotation, translation) = transform.to_scale_rotation_translation();

        if scale.min_element() <= 0.0 {
            //todo do scale properly
            return None;
        }
        let anchor = self.anchor.as_vec();
        let centre = translation.truncate() //todo precalculate and put on the component
            + Vec2 {
                x: self.extents_abs.x * -anchor.x,
                y: self.extents_abs.y * -anchor.y,
            };

        let double_distance = (position - centre).abs() * 2.0;

        if double_distance.x <= self.extents_abs.x && double_distance.y <= self.extents_abs.y {
            Some(double_distance / self.extents_abs)
        } else {
            None
        }
    }
}
#[derive(Debug, Resource, Default, PartialEq)]
pub enum PressedEntity {
    #[default]
    None,

    Pressed {
        start_entity: Option<Entity>,
        current_entity: Option<Entity>,
        start_elapsed: core::time::Duration,
        is_mouse: bool,
    },
}

impl PressedEntity {
    pub fn is_mouse(&self) -> bool {
        match self {
            PressedEntity::None => false,
            PressedEntity::Pressed { is_mouse, .. } => *is_mouse,
        }
    }
}

fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Query<(Entity, &GlobalTransform, &ClickableComponent)>,

    mut events: EventWriter<ClickEvent>,
    size: Res<WindowSize>,

    mut pressed_entity: ResMut<PressedEntity>,
    time: Res<Time>,
) {
    let position: Vec2;
    let click_phase: ClickPhase;

    if mouse_input.just_released(MouseButton::Left) {
        let Some(position1) = get_cursor_position(q_windows, &size) else {
            *pressed_entity = PressedEntity::None;
            return;
        };
        position = position1;
        click_phase = ClickPhase::End;
    } else if mouse_input.just_pressed(MouseButton::Left) {
        let Some(position1) = get_cursor_position(q_windows, &size) else {
            *pressed_entity = PressedEntity::None;
            return;
        };
        position = position1;
        click_phase = ClickPhase::Start;
    } else if mouse_input.pressed(MouseButton::Left) {
        let Some(position1) = get_cursor_position(q_windows, &size) else {
            *pressed_entity = PressedEntity::None;
            return;
        };
        position = position1;
        click_phase = ClickPhase::Move;
    } else {
        if pressed_entity.is_mouse() {
            *pressed_entity = PressedEntity::None;
        }

        return;
    };

    if let Some((entity, _transform, component, scaled_distance)) = buttons
        .iter()
        .filter(|x| x.2.enabled)
        .flat_map(|(entity, transform, component)| {
            let Some(scaled_distance) = component.get_click_distance(position, &transform) else {
                return None;
            };
            Some((entity, transform, component, scaled_distance))
        })
        .max_by(|(_, g1, _, _), (_, g2, _, _)| g1.translation().z.total_cmp(&g2.translation().z))
    {
        //info!("Event Found");
        events.send(ClickEvent {
            on_click: component.on_click.clone(),
            input_device: InputDevice::Mouse,
            click_phase,
            scaled_distance,
        });

        match click_phase {
            ClickPhase::Start => {
                *pressed_entity = PressedEntity::Pressed {
                    start_entity: Some(entity),
                    current_entity: Some(entity),
                    start_elapsed: time.elapsed(),
                    is_mouse: true,
                }
            }
            ClickPhase::Move => {
                let new_pressed = match pressed_entity.as_ref() {
                    PressedEntity::None => PressedEntity::Pressed {
                        start_entity: Some(entity),
                        current_entity: Some(entity),
                        start_elapsed: time.elapsed(),
                        is_mouse: true,
                    },
                    PressedEntity::Pressed {
                        start_entity,
                        start_elapsed,
                        current_entity: _current_entity,
                        is_mouse: _,
                    } => PressedEntity::Pressed {
                        start_entity: start_entity.clone(),
                        current_entity: Some(entity),
                        start_elapsed: start_elapsed.clone(),
                        is_mouse: true,
                    },
                };

                pressed_entity.set_if_neq(new_pressed);
            }
            ClickPhase::End => {
                pressed_entity.set_if_neq(PressedEntity::None);
            }
        }
    } else if click_phase.is_end() || click_phase.is_start() {
        if click_phase.is_start() {
            *pressed_entity = PressedEntity::Pressed {
                start_entity: None,
                current_entity: None,
                start_elapsed: time.elapsed_wrapped(),
                is_mouse: true,
            };
        } else {
            *pressed_entity = PressedEntity::None;
        }
    }
}
fn handle_touches(
    mut touch_events: EventReader<TouchInput>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    size: Res<WindowSize>,
    mut events: EventWriter<ClickEvent>,
    buttons: Query<(Entity, &GlobalTransform, &ClickableComponent)>,
    mut pressed_entity: ResMut<PressedEntity>,
    time: Res<Time>,
) {
    let mut position: Vec2;
    let mut click_phase: ClickPhase;

    //let touch_events_len = touch_events.len();

    for ev in touch_events.read() {
        match ev.phase {
            bevy::input::touch::TouchPhase::Started => {
                let Some(position1) = get_touch_position(ev.position, &q_camera, &size) else {
                    *pressed_entity = PressedEntity::None;
                    continue;
                };
                position = position1;
                click_phase = ClickPhase::Start;
            }
            bevy::input::touch::TouchPhase::Moved => {
                let Some(position1) = get_touch_position(ev.position, &q_camera, &size) else {
                    *pressed_entity = PressedEntity::None;
                    continue;
                };
                position = position1;
                click_phase = ClickPhase::Move;
            }
            bevy::input::touch::TouchPhase::Ended => {
                let Some(position1) = get_touch_position(ev.position, &q_camera, &size) else {
                    *pressed_entity = PressedEntity::None;
                    continue;
                };
                position = position1;
                click_phase = ClickPhase::End;
            }
            bevy::input::touch::TouchPhase::Canceled => {
                let Some(position1) = get_touch_position(ev.position, &q_camera, &size) else {
                    *pressed_entity = PressedEntity::None;
                    continue;
                };
                position = position1;
                click_phase = ClickPhase::End;
            }
        }

        if let Some((entity, _, component, scaled_distance)) = buttons
            .iter()
            .filter(|x| x.2.enabled)
            .flat_map(|(entity, transform, component)| {
                let Some(scaled_distance) = component.get_click_distance(position, &transform)
                else {
                    return None;
                };
                Some((entity, transform, component, scaled_distance))
            })
            .max_by(|(_, g1, _, _), (_, g2, _, _)| {
                g1.translation().z.total_cmp(&g2.translation().z)
            })
        {
            //info!("Event Found");
            events.send(ClickEvent {
                on_click: component.on_click.clone(),
                input_device: InputDevice::Touch,
                click_phase,
                scaled_distance,
            });

            match click_phase {
                ClickPhase::Start => {
                    //info!("Click start touch with entity");
                    *pressed_entity = PressedEntity::Pressed {
                        start_entity: Some(entity),
                        current_entity: Some(entity),
                        start_elapsed: time.elapsed(),
                        is_mouse: false,
                    }
                }
                ClickPhase::Move => {
                    let new_pressed = match pressed_entity.as_ref() {
                        PressedEntity::None => PressedEntity::Pressed {
                            start_entity: Some(entity),
                            current_entity: Some(entity),
                            start_elapsed: time.elapsed(),
                            is_mouse: false,
                        },
                        PressedEntity::Pressed {
                            start_entity,
                            start_elapsed,
                            current_entity: _current_entity,
                            is_mouse: _,
                        } => PressedEntity::Pressed {
                            start_entity: start_entity.clone(),
                            current_entity: Some(entity),
                            start_elapsed: start_elapsed.clone(),
                            is_mouse: false,
                        },
                    };

                    pressed_entity.set_if_neq(new_pressed);
                }
                ClickPhase::End => {
                    //info!("Click end touch with entity");
                    *pressed_entity = PressedEntity::None
                }
            }
        } else if click_phase.is_start() {
            //info!("Click start touch");
            *pressed_entity = PressedEntity::Pressed {
                start_entity: None,
                current_entity: None,
                start_elapsed: time.elapsed(),
                is_mouse: false,
            };
        } else if click_phase.is_end() {
            //info!("Click end touch");
            *pressed_entity = PressedEntity::None;
        }
    }

    //info!("After {} touch events, pressed entity is `{:?}`", touch_events_len, pressed_entity);
}

fn get_touch_position(
    position: Vec2,
    q_camera: &Query<(&Camera, &GlobalTransform)>,
    _size: &WindowSize,
    //tolerance: f32,
) -> Option<Vec2> {
    let p = convert_screen_to_world_position(position, q_camera)?;

    // let p = Vec2 {
    //     x: p.x + (size.0.scaled_width * 0.5),
    //     y: (size.0.scaled_height * 0.5) - p.y,
    // };
    Some(p)
}

fn convert_screen_to_world_position(
    screen_pos: Vec2,
    q_camera: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    camera.viewport_to_world_2d(camera_transform, screen_pos)
}

fn get_cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    window_size: &WindowSize,
) -> Option<Vec2> {
    let window = q_windows.iter().next()?;
    let p = window.cursor_position()?;

    let p = Vec2 {
        x: p.x - (window_size.logical_width * 0.5),
        y: (window_size.logical_height * 0.5) - p.y,
    };
    Some(p)
}
