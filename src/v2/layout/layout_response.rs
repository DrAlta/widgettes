use std::collections::HashMap;

use crate::v2::layout::{Layout, Rect, Splat};

pub enum LayoutResponse<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash> {
    Layout(Splat<InterSpaxel, Spaxel, ChildId>),
    RequestLayoutOfChildren {
        callback: Layout<InterSpaxel, Spaxel, ChildId>,
        children_to_layout: HashMap<ChildId, Rect<Spaxel>>,
    },
}
