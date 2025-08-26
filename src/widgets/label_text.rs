use std::{borrow::Borrow, cell::RefCell};

use macroquad::prelude::*;

use crate::Widget;

pub struct LabelText {
    pub text: String,
    pub font: RefCell<Font>,
    pub font_size: u16,
    pub color: Color,
}
impl LabelText {
    pub fn new(text: String, font: RefCell<Font>, font_size: u16, color: Color) -> Self {
        LabelText {
            text,
            font,
            font_size,
            color,
        }
    }
}
impl Widget for LabelText {
    fn draw(&self, x: f32, y: f32, _: f32, _: f32) {
        let offset_y = measure_text(
            &self.text,
            Some(self.font.borrow().borrow()),
            self.font_size,
            1.0,
        )
        .offset_y;
        draw_text(
            &self.text,
            x,
            y + offset_y,
            self.font_size as f32,
            self.color,
        );
    }

    fn get_dimensions(&self) -> (f32, f32) {
        let x = measure_text(
            &self.text,
            Some(self.font.borrow().borrow()),
            self.font_size,
            1.0,
        );
        (x.width, x.height)
    }
}
