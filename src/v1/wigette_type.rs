use crate::v1::{widgets::label_text::LabelText, Wigette};

#[allow(dead_code)]
pub enum WigetteType {
    Box,
    HBox {
        children: Vec<Wigette>,
        distended_width: u32,
        distended_height: u32,
    },
    VBox {
        children: Vec<Wigette>,
        distended_width: u32,
        distended_height: u32,
    },
    Label(LabelText),
}
