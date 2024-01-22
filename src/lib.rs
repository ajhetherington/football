use std::ops::{Deref, DerefMut};

use macroquad::color::Color;
#[macro_export]
macro_rules! get_quad_gl {
    () => {
        unsafe { get_internal_gl().quad_gl }
    };
}

// todo: impl serialize, fine for now
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct MacroColour(pub macroquad::color::Color);

impl Deref for MacroColour {
    type Target = macroquad::color::Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MacroColour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl From<macroquad::color::Color> for MacroColour {
    fn from(col: Color) -> Self {
        Self(col)
    }
}

impl Into<macroquad::color::Color> for MacroColour {
    fn into(self) -> macroquad::color::Color {
        self.0
    }
}