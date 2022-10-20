use crate::vectors::Vec2;
use std::collections::HashSet;
pub use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode as Key;

#[derive(Clone)]
pub enum InputEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    PointerMoved(Vec2<f64>),
    ScrollMoved(Vec2<f64>),
}

#[derive(Default)]
pub struct InputState {
    pub pressed_keys: HashSet<Key>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,
    pub cursor_pos: Vec2<f64>,
    pub scroll_pos: Vec2<f64>,
}
impl InputState {
    #[inline(always)]
    pub fn key_is_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    #[inline(always)]
    pub fn mouse_button_is_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }
}
