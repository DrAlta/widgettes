mod layout;
pub use layout::{Rect, Resolution, Splat, Layout, LayoutResponse, Widget};
mod graphics;
pub use graphics::Graphics;
pub mod util;


pub use layout::foo;// foo is just testing it should be removed and replaced with the finisged Widget enum
pub mod widgets;