use std::collections::HashMap;

use winit::keyboard::{KeyCode, PhysicalKey};
use winit::event::KeyEvent;

pub struct InputManager {
    keys: HashMap<KeyCode, bool>,
    just_pressed: HashMap<KeyCode, bool>,
    just_released: HashMap<KeyCode, bool>,
    mouse_position: (f64, f64),  // Absolute position
    mouse_delta: (f64, f64),  // Relative position since last update call
    mouse_wheel: f64,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            just_pressed: HashMap::new(),
            just_released: HashMap::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            mouse_wheel: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn process_key_event(&mut self, event: &KeyEvent) {
        if event.repeat {
            return;
        }

        let key = if let PhysicalKey::Code(key) = event.physical_key {
            key
        } else {
            return;
        };

        match event.state {
            winit::event::ElementState::Pressed => {
                self.keys.insert(key, true);
                self.just_pressed.insert(key, true);
            }
            winit::event::ElementState::Released => {
                self.keys.insert(key, false);
                self.just_released.insert(key, true);
            }
        };
    }

    pub fn process_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }

    pub fn process_mouse_delta(&mut self, dx: f64, dy: f64) {
        self.mouse_delta = (dx, dy);
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys.get(&key).copied().unwrap_or(false)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.get(&key).copied().unwrap_or(false)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released.get(&key).copied().unwrap_or(false)
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
