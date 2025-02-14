use std::any::Any;

#[derive(Debug)]
pub struct Light {
    pub position: glam::Vec3,
    pub color: [f32; 3],
    pub intensity: f32,
    inner: Box<dyn LightTrait>,
}

// Trait for different types of lights
trait LightTrait: Any + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct PointLight {
    pub attenuation: [f32; 3], // constant, linear, quadratic
}

impl PointLight {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Light {
        Light::new_point_light()
    }
}

impl LightTrait for PointLight {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            attenuation: [1.0, 0.09, 0.032],
        }
    }
}

#[derive(Debug)]
pub struct SpotLight {
    pub direction: glam::Vec3,
    pub attenuation: [f32; 3], // constant, linear, quadratic
    pub inner_cutoff_rad: f32,
    pub outer_cutoff_rad: f32,
}

impl SpotLight {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Light {
        Light::new_spot_light()
    }
}

impl LightTrait for SpotLight {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for SpotLight {
    fn default() -> Self {
        Self {
            direction: glam::Vec3::new(0.0, 0.0, -1.0),
            attenuation: [1.0, 0.09, 0.032],
            inner_cutoff_rad: 12.5f32.to_radians(),
            outer_cutoff_rad: 17.5f32.to_radians(),
        }
    }
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: glam::Vec3,
}

impl DirectionalLight {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Light {
        Light::new_directional_light()
    }
}

impl LightTrait for DirectionalLight {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: glam::Vec3::new(0.0, 0.0, -1.0),
        }
    }
}

impl Light {
    pub fn is_point_light(&self) -> bool {
        self.inner.as_any().is::<PointLight>()
    }

    pub fn as_point_light(&self) -> Option<&PointLight> {
        self.inner.as_any().downcast_ref::<PointLight>()
    }

    pub fn as_point_light_mut(&mut self) -> Option<&mut PointLight> {
        self.inner.as_any_mut().downcast_mut::<PointLight>()
    }

    pub fn is_spot_light(&self) -> bool {
        self.inner.as_any().is::<SpotLight>()
    }

    pub fn as_spot_light(&self) -> Option<&SpotLight> {
        self.inner.as_any().downcast_ref::<SpotLight>()
    }

    pub fn as_spot_light_mut(&mut self) -> Option<&mut SpotLight> {
        self.inner.as_any_mut().downcast_mut::<SpotLight>()
    }

    pub fn is_directional_light(&self) -> bool {
        self.inner.as_any().is::<DirectionalLight>()
    }

    pub fn as_directional_light(&self) -> Option<&DirectionalLight> {
        self.inner.as_any().downcast_ref::<DirectionalLight>()
    }

    pub fn as_directional_light_mut(&mut self) -> Option<&mut DirectionalLight> {
        self.inner.as_any_mut().downcast_mut::<DirectionalLight>()
    }

    pub fn new_point_light() -> Self {
        Self::default()
    }

    pub fn new_spot_light() -> Self {
        Self {
            inner: Box::new(SpotLight::default()),
            ..Default::default()
        }
    }

    pub fn new_directional_light() -> Self {
        Self {
            inner: Box::new(DirectionalLight::default()),
            ..Default::default()
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            position: glam::Vec3::new(0.0, 0.0, 0.0),
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            inner: Box::new(PointLight::default()),
        }
    }
}
