use embedded_graphics::{ Drawable,
    mono_font::{MonoFont, MonoTextStyle}, prelude::{DrawTarget, PixelColor, Point}, text::{Baseline, Text, renderer::TextRenderer}
};
// Assuming your existing Widget trait might need adjustment to accept a DrawTarget
use crate::v1::Widget; 

// The generic type F allows using any monospaced font from embedded-graphics
pub struct LabelText<'a, C> {
    pub text: String,
    pub font: MonoFont<'a>,
    pub color: C, // Use Rgb565, Rgb888, BinaryColor, etc.
}

impl<'a, C: PixelColor> LabelText<'a, C> {
    // Note: font_size is determined by F, so we remove it from the constructor's parameters
    pub fn new(text: String, font: MonoFont<'a>, color: C) -> Self {
        LabelText {
            text,
            font,
            color,
        }
    }
}

// The Widget trait's draw method needs to accept a mutable DrawTarget
impl<'a, C: PixelColor> Widget<C> for LabelText<'a, C> {
    // We assume the draw target uses Rgb565 color; adjust as needed
    fn draw<T: DrawTarget<Color = C>>(&self, x: i32, y: i32, _: u32, _: u32, target: &mut T ) -> Result<(), <T as DrawTarget>::Error> {
        
        // embedded-graphics uses Point for positioning (i32, i32)
        let position = Point::new(x, y);
        
        // Create a character style and apply it to the text
        let character_style = MonoTextStyle::new(&self.font, self.color);

        // embedded-graphics aligns text differently. By default, the Y coordinate 
        // is the top of the text bounding box. You may need to adjust Y based on baseline if desired.
        Text::new(&self.text, position, character_style)
            .draw(target)?; // The draw call might return a Result<(), Error>
            
        Ok(())
    }

    fn get_dimensions(&self) -> (u32, u32) {
        // Use the bounding box calculation for dimensions. This is an estimate.
        let character_style = MonoTextStyle::new(&self.font, self.color);
        let metrics = character_style.measure_string(&self.text, Point::zero(), Baseline::Top);
        
        (metrics.bounding_box.size.width, metrics.bounding_box.size.height)
    }
}
