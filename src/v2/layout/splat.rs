use std::collections::HashMap;

use crate::v2::{Layout, Rect};

#[derive(Debug, Clone, PartialEq)]
pub struct Splat<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash> {
    pub rect:Rect<Spaxel>, 
    pub childrens_layouts: HashMap<ChildId, Layout<InterSpaxel, Spaxel, ChildId>>,
}