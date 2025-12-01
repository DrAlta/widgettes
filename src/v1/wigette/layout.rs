use crate::v1::{LayoutRet, Wigette, WigetteType};

impl Wigette {
    pub fn update_childrens_pos(&mut self, padding: i64) {
        let mut turtle = 0_i64;
        let x = self.x + (padding / 2);
        let y = self.y + (padding / 2);
        let half_p_width = self.get_width() as i64 / 2;
        let half_p_height = self.get_height() as i64 / 2;
        match &mut self.wigette_type {
            WigetteType::HBox { children, .. } => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height() as i64;
                    let c_width = child.get_width() as i64;
                    let my_padding = index as i64 * padding;
                    child.set_pos(
                        x + turtle + (index as i64 * padding),
                        (y + half_p_height) - ((padding + c_height) / 2),
                    );
                    turtle += c_width + my_padding;
                }
            }
            WigetteType::VBox { children, .. } => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height() as i64;
                    let c_width = child.get_width() as i64;
                    let my_padding = index as i64 * padding;
                    child.set_pos(
                        (x + half_p_width) - ((padding + c_width) / 2),
                        y + turtle + my_padding,
                    );
                    turtle += c_height + my_padding;
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
