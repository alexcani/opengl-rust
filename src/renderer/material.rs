use std::collections::hash_map::Entry;
use std::rc::Rc;
use std::{cell::Cell, collections::HashMap};

use crate::renderer::{ShaderProgram, Texture2D};

#[derive(Clone)]
pub struct Material {
    name: String,
    shader: Rc<ShaderProgram>,
    properties: HashMap<String, PropertyState>,
}

#[derive(Clone, PartialEq)]
pub enum MaterialProperty {
    Boolean(bool),
    Integer(i32),
    UInteger(u32),
    Float(f32),
    Vec3([f32; 3]),
    Color(f32, f32, f32),
    Texture(Rc<Texture2D>),
}

#[derive(Clone)]
struct PropertyState {
    property: MaterialProperty,
    dirty: Cell<bool>,
}

impl Material {
    pub fn new(name: &str, shader: Rc<ShaderProgram>) -> Self {
        Self {
            name: name.to_string(),
            shader,
            properties: HashMap::new(),
        }
    }

    pub fn new_with_properties(
        name: &str,
        shader: Rc<ShaderProgram>,
        properties: HashMap<String, MaterialProperty>,
    ) -> Self {
        let states = properties
            .into_iter()
            .map(|(name, property)| {
                (
                    name,
                    PropertyState {
                        property,
                        dirty: Cell::new(true),
                    },
                )
            })
            .collect();
        Self {
            name: name.to_string(),
            shader,
            properties: states,
        }
    }

    pub fn clone_with_overrides(
        &self,
        new_name: &str,
        overrides: HashMap<String, MaterialProperty>,
    ) -> Self {
        let new_states: HashMap<_, _> = overrides
            .into_iter()
            .map(|(name, property)| {
                (
                    name,
                    PropertyState {
                        property,
                        dirty: Cell::new(true),
                    },
                )
            })
            .collect();

        let mut old_states = self.properties.clone();
        old_states.extend(new_states);
        Self {
            name: new_name.to_string(),
            shader: Rc::clone(&self.shader),
            properties: old_states,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shader(&self) -> Rc<ShaderProgram> {
        Rc::clone(&self.shader)
    }

    pub fn set_property(&mut self, name: &str, value: MaterialProperty) {
        let key = name.to_string();
        match self.properties.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(PropertyState {
                    property: value,
                    dirty: Cell::new(true),
                });
            }
            Entry::Occupied(mut entry) => {
                let state = entry.get_mut();
                if state.property != value {
                    state.property = value;
                    state.dirty = Cell::new(true);
                }
            }
        }
    }

    pub fn set_boolean(&mut self, name: &str, value: bool) {
        self.set_property(name, MaterialProperty::Boolean(value));
    }

    pub fn set_integer(&mut self, name: &str, value: i32) {
        self.set_property(name, MaterialProperty::Integer(value));
    }

    pub fn set_uinteger(&mut self, name: &str, value: u32) {
        self.set_property(name, MaterialProperty::UInteger(value));
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        self.set_property(name, MaterialProperty::Float(value));
    }

    pub fn set_vec3(&mut self, name: &str, value: [f32; 3]) {
        self.set_property(name, MaterialProperty::Vec3(value));
    }

    pub fn set_color(&mut self, name: &str, r: f32, g: f32, b: f32) {
        self.set_property(name, MaterialProperty::Color(r, g, b));
    }

    pub fn set_texture(&mut self, name: &str, texture: Rc<Texture2D>) {
        self.set_property(name, MaterialProperty::Texture(texture));
    }

    pub fn delete_property(&mut self, name: &str) {
        self.properties.remove(name);
    }

    pub fn use_material(&self) {
        self.shader.use_program();

        let mut texture_slot = 0;
        for (name, state) in &self.properties {
            if !state.dirty.get() {
                continue;
            }

            match &state.property {
                MaterialProperty::Boolean(value) => {
                    self.shader.set_uniform_1i(name, *value as i32);
                }
                MaterialProperty::Integer(value) => {
                    self.shader.set_uniform_1i(name, *value);
                }
                MaterialProperty::UInteger(value) => {
                    self.shader.set_uniform_1ui(name, *value);
                }
                MaterialProperty::Float(value) => {
                    self.shader.set_uniform_1f(name, *value);
                }
                MaterialProperty::Vec3(value) => {
                    self.shader.set_uniform_3fv(name, value);
                }
                MaterialProperty::Color(r, g, b) => {
                    self.shader.set_uniform_3f(name, *r, *g, *b);
                }
                MaterialProperty::Texture(texture) => {
                    texture.bind_slot(texture_slot);
                    self.shader.set_uniform_1i(name, texture_slot as i32);
                    texture_slot += 1;
                }
            }
            state.dirty.set(false);
        }
    }
}
