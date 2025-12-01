use embedded_graphics::prelude::PixelColor;

use crate::v1::{LayoutRet, Wigette, WigetteType};

impl<'a, C: PixelColor> Wigette<'a, C> {
    pub fn update_childrens_pos(&mut self, padding: u32) {
        let mut turtle = 0_i32;
        let x = self.x + (padding as i32 / 2);
        let y = self.y + (padding as i32 / 2);
        let half_p_width = (self.get_width() / 2 ) as i32 ;
        let half_p_height = (self.get_height() / 2 ) as i32 ;
        match &mut self.wigette_type {
            WigetteType::HBox { children, .. } => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height();
                    let c_width = child.get_width() ;
                    let my_padding = index as u32 * padding;
                    child.set_pos(
                        x + turtle + (index as i32 * padding as i32 ),
                        (y + half_p_height) - ((padding + c_height) / 2) as i32 ,
                    );
                    turtle += (c_width + my_padding) as i32 ;
                }
            }
            WigetteType::VBox { children, .. } => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height() as u32;
                    let c_width = child.get_width() as u32;
                    let my_padding = index as u32 * padding;
                    child.set_pos(
                        (x + half_p_width) - ((padding + c_width) / 2) as i32,
                        y + turtle + my_padding as i32,
                    );
                    turtle += (c_height + my_padding) as i32;
                }
            }
            WigetteType::Box => {}
            WigetteType::Label(_) => {}
        }
    }
    pub fn update_size(&mut self) {
        let my_min_width = self.get_min_width();
        let my_min_height = self.get_min_height();
        match &mut self.wigette_type {
            WigetteType::HBox { children, .. } => {
                let LayoutRet {
                    height: y,
                    width: x,
                    distended_width: _,
                    distended_height: _,
                } = Self::h_size(
                    my_min_width,
                    my_min_height,
                    children,
                    2, //&Self::get_width,
                       //&Self::get_min_width,
                       //&Self::set_size_width,
                       //&Self::get_expand_width
                );
                self.set_size(x, y);
            }
            WigetteType::VBox { children, .. } => {
                let LayoutRet {
                    height: y,
                    width: x,
                    distended_width: _,
                    distended_height: _,
                } = Self::v_size(
                    my_min_width as usize,
                    my_min_height as usize,
                    children,
                    2, //&Self::get_height,
                       //&Self::get_min_y,
                       //&Self::set_size_y,
                       //&Self::get_expand_y
                );
                self.set_size(x, y);
            }
            _ => {}
        }
    }
} // public functions
