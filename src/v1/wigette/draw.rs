use macroquad::prelude::*;

use crate::v1::{Widget, Wigette, WigetteType};

impl Wigette {
    pub const COLORS: [Color; 8] = [RED, PINK, BLUE, GREEN, PURPLE, YELLOW, ORANGE, MAGENTA];
    pub fn draw(&self, lvl: usize) {
        #[cfg(feature = "gui_debug")]
        draw_rectangle(
            self.x as f32,
            self.y as f32,
            self.get_width() as f32,
            self.get_height() as f32,
            Self::COLORS[lvl],
        );

        match &self.wigette_type {
            WigetteType::HBox {
                children,
                distended_width: _,
                distended_height: _,
            } => {
                for child in children {
                    child.draw(lvl + 1);
                }
            }
            WigetteType::VBox {
                children,
                distended_width: _,
                distended_height: _,
            } => {
                for child in children {
                    child.draw(lvl + 1);
                }
            }
            WigetteType::Label(label) => {
                label.draw(self.x as f32, self.y as f32, 0.0, 0.0);
            }
            _ => {}
        }
    }
}
