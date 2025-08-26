use crate::v2::Layout;

type ChildId = usize;
type InterSpaxel = i32;
type Spaxel = i32;
type Callback = Layout<InterSpaxel, Spaxel, ChildId>;

//mod flex_wrap_layout;
mod foo;
pub use foo::Foo;
mod linear_layout;
pub use linear_layout::{linear_layout, Axis};
