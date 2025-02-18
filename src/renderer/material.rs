use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::renderer::{ShaderProgram, Texture2D};

#[derive(Clone)]
pub struct Material {
    name: String,
    shader: Rc<ShaderProgram>,
    properties: PropertiesMap,
    texture_to_slot: RefCell<HashMap<Rc<Texture2D>, u32>>,
    texture_slots: RefCell<[bool; 16]>, // Mark which slots are in use
}

#[derive(Clone, PartialEq, Debug)]
pub enum MaterialProperty {
    Boolean(bool),
    Integer(i32),
    UInteger(u32),
    Float(f32),
    Vec3([f32; 3]),
    Color(f32, f32, f32),
    Texture(Rc<Texture2D>),
}

impl Material {
    pub fn new(name: &str, shader: Rc<ShaderProgram>) -> Self {
        Self {
            name: name.to_string(),
            shader,
            properties: PropertiesMap::new(),
            texture_to_slot: RefCell::new(HashMap::new()),
            texture_slots: RefCell::new([false; 16]),
        }
    }

    pub fn new_with_properties(
        name: &str,
        shader: Rc<ShaderProgram>,
        properties: PropertiesMap,
    ) -> Self {
        Self {
            name: name.to_string(),
            shader,
            properties,
            texture_to_slot: RefCell::new(HashMap::new()),
            texture_slots: RefCell::new([false; 16]),
        }
    }

    pub fn clone_with_overrides(
        &self,
        new_name: &str,
        overrides: HashMap<String, MaterialProperty>,
    ) -> Self {
        let mut properties = self.properties.clone();
        properties.map.extend(overrides);
        Self {
            name: new_name.to_string(),
            shader: Rc::clone(&self.shader),
            properties,
            texture_to_slot: self.texture_to_slot.clone(),
            texture_slots: self.texture_slots.clone(),
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

    pub fn properties(&self) -> &PropertiesMap {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut PropertiesMap {
        &mut self.properties
    }

    pub fn use_material(&self, overrides: &PropertiesMap) {
        self.shader.use_program();

        for (name, value) in &self.properties.map {
            let value = match overrides.map.get(name) {
                Some(value) => value,
                None => value,
            };

            match value {
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
                    let slot = self.texture_to_slot.borrow().get(texture).copied();
                    let texture_slot = match slot {
                        Some(slot) => slot,
                        None => {
                            // Updates for all textures
                            self.update_texture_slots();
                            self.texture_to_slot.borrow()[texture] // panic if not found
                        }
                    };
                    self.shader.set_uniform_1i(name, texture_slot as i32);
                }
            }
        }

        // Bind textures
        for (texture, slot) in &*self.texture_to_slot.borrow() {
            texture.bind_slot(*slot);
        }
    }

    fn update_texture_slots(&self) {
        let used_textures: HashSet<_> = self
            .properties
            .map
            .values()
            .filter_map(|value| {
                if let MaterialProperty::Texture(texture) = value {
                    Some(Rc::clone(texture))
                } else {
                    None
                }
            })
            .collect();
        let bound_textures: HashSet<_> = self
            .texture_to_slot
            .borrow()
            .keys()
            .map(Rc::clone)
            .collect();

        // Remove unused textures from the map
        let unused_textures: HashSet<_> = bound_textures.difference(&used_textures).collect();
        for texture in unused_textures {
            let slot = self.texture_to_slot.borrow()[texture];
            self.texture_slots.borrow_mut()[slot as usize] = false;
            self.texture_to_slot.borrow_mut().remove(texture);
        }

        // Add new textures to the map
        let unbound_textures: HashSet<_> = used_textures.difference(&bound_textures).collect();
        for texture in unbound_textures {
            let slot = self
                .texture_slots
                .borrow()
                .iter()
                .position(|&x| !x)
                .unwrap();
            self.texture_slots.borrow_mut()[slot] = true;
            self.texture_to_slot
                .borrow_mut()
                .insert(Rc::clone(texture), slot as u32);
        }
    }
}

// Holds a set of properties for a material
#[derive(Clone, Default)]
pub struct PropertiesMap {
    map: HashMap<String, MaterialProperty>,
}

impl PropertiesMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: MaterialProperty) {
        self.map.insert(name.to_string(), value);
    }

    pub fn set_boolean(&mut self, name: &str, value: bool) {
        self.set(name, MaterialProperty::Boolean(value));
    }

    pub fn set_integer(&mut self, name: &str, value: i32) {
        self.set(name, MaterialProperty::Integer(value));
    }

    pub fn set_uinteger(&mut self, name: &str, value: u32) {
        self.set(name, MaterialProperty::UInteger(value));
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        self.set(name, MaterialProperty::Float(value));
    }

    pub fn set_vec3(&mut self, name: &str, value: [f32; 3]) {
        self.set(name, MaterialProperty::Vec3(value));
    }

    pub fn set_color(&mut self, name: &str, r: f32, g: f32, b: f32) {
        self.set(name, MaterialProperty::Color(r, g, b));
    }

    pub fn set_texture(&mut self, name: &str, texture: Rc<Texture2D>) {
        self.set(name, MaterialProperty::Texture(texture));
    }

    pub fn delete(&mut self, name: &str) {
        self.map.remove(name);
    }
}

impl From<HashMap<String, MaterialProperty>> for PropertiesMap {
    fn from(map: HashMap<String, MaterialProperty>) -> Self {
        Self { map }
    }
}

impl<const N: usize> From<[(String, MaterialProperty); N]> for PropertiesMap {
    fn from(array: [(String, MaterialProperty); N]) -> Self {
        Self {
            map: array.into()
        }
    }
}
