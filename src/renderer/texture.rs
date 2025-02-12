use std::io::{Error, ErrorKind};

use gl::types::*;

use image::ImageReader;
use image::metadata::Orientation;

#[derive(PartialEq, Eq)]
pub struct Texture2D {
    id: GLuint,
}

#[allow(dead_code)]
impl Texture2D {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        Self { id }
    }

    pub fn new_from_file(file_path: &str) -> Result<Self, String> {
        let texture = Self::new();
        texture.load_file_impl(file_path).map_err(|e| e.to_string())?;
        Ok(texture)
    }

    pub fn load_file(&self, file_path: &str) -> Result<(), String> {
        self.load_file_impl(file_path).map_err(|e| e.to_string())
    }

    pub fn bind_slot(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn load_file_impl(&self, file_path: &str) -> Result<(), Error> {
        let loader = ImageReader::open(file_path)?;
        let mut image = loader.decode().map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        image.apply_orientation(Orientation::FlipVertical);
        let image = image.into_rgb8();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                image.width() as GLint,
                image.height() as GLint,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(())
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
