use embedded_graphics::{
    pixelcolor::Rgb565, // Using Rgb565 as the standard color type
    prelude::*,
};
// Assuming your other types are defined and compatible
use crate::v1::{Widget, Wigette, WigetteType};

impl<'a> Wigette<'a, Rgb565> {
    // We must redefine the colors using the embedded_graphics color type (Rgb565)
    pub const COLORS: [Rgb565; 8] = [
        Rgb565::RED,
        Rgb565::MAGENTA, // Pink replacement, Rgb565 doesn't have a direct 'Pink'
        Rgb565::BLUE,
        Rgb565::GREEN,
        Rgb565::MAGENTA, // Purple replacement
        Rgb565::YELLOW,
        Rgb565::CSS_ORANGE,
        Rgb565::MAGENTA, // Magenta
    ];

    // The draw function now takes a mutable DrawTarget as a parameter
    pub fn draw<T: DrawTarget<Color = Rgb565>>(&self, lvl: usize, target: &mut T) -> Result<(), <T as DrawTarget>::Error> {
        
        #[cfg(feature = "gui_debug")]
        {
            // Use Rectangle primitive instead of draw_rectangle function
            let fill_color = Self::COLORS[lvl % Self::COLORS.len()]; // Use modulo to prevent index out of bounds
            let style = PrimitiveStyleBuilder::new()
                .fill(fill_color)
                .build();
            
            let rect = Rectangle::new(
                Point::new(self.x as i32, self.y as i32),
                Size::new(self.get_width() as u32, self.get_height() as u32),
            );

            rect.into_styled(style).draw(target)?; // Draw the rectangle onto the target
        }

        match &self.wigette_type {
            WigetteType::HBox { children, .. } | WigetteType::VBox { children, .. } => {
                for child in children {
                    // Pass the target down to the children recursively
                    child.draw(lvl + 1, target)?; 
                }
            }
            WigetteType::Label(label) => {
                // Call the label's draw method, passing the target
                label.draw(self.x, self.y, 0, 0, target)?;
            }
            _ => {}
        }

        // Return Ok(()) on success, as embedded-graphics draw calls return Results
        Ok(())
    }
}
