use std::collections::HashMap;
use std::rc::Rc;

use crate::renderer::{ShaderProgram, Texture2D};

#[derive(Clone)]
pub struct Material {
    name: String,
    shader: Rc<ShaderProgram>,
    properties: HashMap<String, MaterialProperty>,
}

#[derive(Clone)]
pub enum MaterialProperty {
    Boolean(bool),
    Integer(i32),
    UInteger(u32),
    Float(f32),
    Color(f32, f32, f32),
    Texture(Rc<Texture2D>),
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
        Self {
            name: name.to_string(),
            shader,
            properties,
        }
    }

    pub fn clone_with_overrides(&self, new_name: &str, overrides: HashMap<String, MaterialProperty>) -> Self {
        let mut new_properties = self.properties.clone();
        new_properties.extend(overrides);
        Self {
            name: new_name.to_string(),
            shader: Rc::clone(&self.shader),
            properties: new_properties,
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
        self.properties.insert(name.to_string(), value);
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
        for (name, property) in &self.properties {
            match property {
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
                MaterialProperty::Color(r, g, b) => {
                    self.shader.set_uniform_3f(name, *r, *g, *b);
                }
                MaterialProperty::Texture(texture) => {
                    texture.bind_slot(texture_slot);
                    self.shader.set_uniform_1ui(name, texture_slot);
                    texture_slot += 1;
                }
            }
        }
    }
}
