use std::collections::HashMap;

use crate::v2::layout::{Rect, Splat};

pub enum LayoutResponse<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash, Callback> {
    Layout(Splat<InterSpaxel, Spaxel, ChildId>),
    RequestLayoutOfChildren {
        callback: Callback,
        children_to_layout: HashMap<ChildId, Rect<Spaxel>>,
    },
}
