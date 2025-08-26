// with the layout system needs to keep track of which chilren have responed to 
// the layout_requests and collect them all before retrying the to layout the parent
//  is the parent assumes that it is geiven all the responses it requested last time it was 
// asked to layout it self.

// widgets should put exes pading on ther right and bottom

// containers should put extra panding around theri children on the up and left
// that ther then extra in the child and around it in the parent balnce each other out


pub mod foo;
mod layout_response;
pub use layout_response::LayoutResponse;
mod rect;
pub use rect::Rect;
mod layout;
pub use layout::Layout;
mod resolution;
pub use resolution::Resolution;
mod splat;
pub use splat::Splat;
mod widget;
pub use widget::Widget;