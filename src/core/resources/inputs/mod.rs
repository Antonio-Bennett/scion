use serde::{Deserialize, Serialize};
use winit::event::ElementState;

use crate::core::resources::inputs::keycode::KeyCode;

pub mod inputs_controller;
pub mod keyboard;
pub mod keycode;
pub mod mouse;

#[derive(Debug, Serialize, Eq, PartialEq, Deserialize, Clone)]
pub enum InputState {
    Pressed,
    Released,
}

impl From<ElementState> for InputState {
    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => InputState::Pressed,
            ElementState::Released => InputState::Released,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardEvent {
    pub keycode: KeyCode,
    pub state: InputState,
}
