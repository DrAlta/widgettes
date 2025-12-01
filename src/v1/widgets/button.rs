use macroquad::prelude::*;

use crate::v1::Widget;

pub struct Button<'a> {
    pub text: String,
    pub font: &'a Font,
    pub font_size: u16,
    pub text_color: Color,
    pub bg_color: Color,
    //pub outline_color: Color,
}
impl<'a> Button<'a> {
    pub fn new<S: Into<String>>(
        text: S,
        font: &'a Font,
        font_size: u16,
        text_color: Color,
        bg_color: Color,
    ) -> Self {
        Self {
            text: text.into(),
            font,
            font_size,
            text_color,
            bg_color,
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32) {
        let outline_color = BLACK;
        let y2 = y + h;
        let x2 = x + w;

        draw_rectangle(x + 3.0, y + 1.0, w - 5.0, 1.0, self.bg_color);
        draw_rectangle(x + 2.0, y + 2.0, w - 3.0, 1.0, self.bg_color);

        draw_rectangle(x + 1.0, y + 3.0, w - 1.0, h - 5.0, self.bg_color);

        draw_rectangle(x + 2.0, y2 - 2.0, w - 3.0, 1.0, self.bg_color);
        draw_rectangle(x + 3.0, y2 - 1.0, w - 5.0, 1.0, self.bg_color);

        //top
        draw_rectangle(x + 1.0, y + 1.0, 2.0, 1.0, outline_color);
        draw_rectangle(x + 3.0, y, w - 5.0, 1.0, outline_color);
        draw_rectangle(x2 - 2.0, y + 1.0, 2.0, 1.0, outline_color);

        // left
        draw_rectangle(x + 1.0, y + 2.0, 1.0, 1.0, outline_color);
        draw_rectangle(x, y + 3.0, 1.0, h - 5.0, outline_color);
        draw_rectangle(x + 1.0, y2 - 2.0, 1.0, 2.0, outline_color);

        // bottom
        draw_rectangle(x + 1.0, y2 - 1.0, 2.0, 1.0, outline_color);
        draw_rectangle(x + 3.0, y2, w - 5.0, 1.0, outline_color);
        draw_rectangle(x + w - 2.0, y2 - 1.0, 2.0, 1.0, outline_color);

        // right
        draw_rectangle(x2 - 1.0, y + 2.0, 1.0, 1.0, outline_color);
        draw_rectangle(x2, y + 3.0, 1.0, h - 5.0, outline_color);
        draw_rectangle(x2 - 1.0, y2 - 2.0, 1.0, 2.0, outline_color);

        //println!("font:{:?}", );
        let offset_y =
            measure_text(&self.text, Some(self.font), self.font_size, 1.0).offset_y + 2.0;

        draw_text_ex(
            &self.text,
            x + 2.0,
            offset_y,
            TextParams {
                font_size: self.font_size,
                font: Some(self.font),
                color: self.text_color,
                ..Default::default()
            },
        );
    }

    fn get_dimensions(&self) -> (f32, f32) {
        let x = measure_text(&self.text, Some(self.font), self.font_size, 1.0);

        println!("{x:?}");

        (x.width + 4.0, x.height + 4.0)
    }
}
/*

...****
.**11111
.*222222
*333333
*

*/
