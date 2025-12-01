use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{CornerRadiiBuilder, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::{Text, renderer::TextRenderer},
};
use crate::v1::Widget; // Assuming your Widget trait is compatible

// The font is now a generic type parameter
pub struct Button<'a, C> {
    pub text: String,
    pub font: MonoFont<'a>,
    pub text_color: C,
    pub bg_color: C,
    pub outline_color: C,
}

impl<'a, C: PixelColor> Button<'a, C> {
    pub fn new<S: Into<String>>(
        text: S,
        font: MonoFont<'a>,
        text_color: C,
        bg_color: C,
        outline_color: C,
    ) -> Self {
        Self {
            text: text.into(),
            font,
            text_color,
            bg_color,
            outline_color,
        }
    }
}

// Widget trait implementation now requires a DrawTarget
impl<'a, C: PixelColor> Widget<C> for Button<'a, C> {
    // Assuming the target can draw Rgb565 pixels
    fn draw<T: DrawTarget<Color = C>>(&self, x: i32, y: i32, w: u32, h: u32, target: &mut T) -> Result<(), <T as DrawTarget>::Error> {
        
        let position = Point::new(x, y);
        let size = Size::new(w, h);
        
        // 1. Draw the main button body using RoundedRectangle for a clean look
        let button_style = PrimitiveStyleBuilder::<C>::new()
            .fill_color(self.bg_color)
            .stroke_color(self.outline_color)
            .stroke_width(1)
            .build();

        // Approximate 3px radius corners to match your custom drawing style
        let corner_radius = CornerRadiiBuilder::new()
            .all(Size::new(3, 3))
            .build(); 

        RoundedRectangle::new(
            Rectangle::new(position, size),
            corner_radius,
        )
        .into_styled(button_style)
        .draw(target)?;

        // 2. Draw the text centered within the button (simplified centering)
        let text_style = MonoTextStyle::new(&self.font, self.text_color);
        
        // Measure text size to help position it
        let text_metrics = text_style.measure_string(&self.text, Point::zero(), embedded_graphics::text::Baseline::Middle);
        
        // Simple horizontal centering (vertical centering is harder without full layout engine)
        let text_x = x + (w as i32 / 2) - (text_metrics.bounding_box.size.width as i32 / 2);
        let text_y = y + (h as i32 / 2); // Baseline::Middle aligns Y to the vertical center line

        Text::new(
            &self.text,
            Point::new(text_x, text_y),
            text_style,
        ).draw(target)?;
            
        Ok(())
    }

    fn get_dimensions(&self) -> (u32, u32) {
        // Dimensions calculation remains similar but uses embedded_graphics metrics
        let text_style = MonoTextStyle::new(&self.font, self.text_color);
        let metrics = text_style.measure_string(&self.text, Point::zero(), embedded_graphics::text::Baseline::Top);
        
        // Add padding for the button boundary (4.0 was your original padding)
        (metrics.bounding_box.size.width + 4, metrics.bounding_box.size.height + 4)
    }
}
