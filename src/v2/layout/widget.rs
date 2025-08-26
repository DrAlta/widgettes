use std::collections::HashMap;

use crate::v2::layout::{LayoutResponse, Rect, Resolution, Splat};

pub trait Widget<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash, Callback, Target> {
    fn ideal_size(&self, field: Rect<Spaxel>) -> Rect<Spaxel>;
    fn min_size(&self) -> Rect<Spaxel>;
    fn layout(
        &self,
        offered: Rect<Spaxel>,
        callback: Option<Callback>,
        children_responses: HashMap<ChildId, Splat<InterSpaxel, Spaxel, ChildId>>,
        children: Vec<ChildId>,
    ) -> LayoutResponse<InterSpaxel, Spaxel, ChildId, Callback>;
    fn draw_under_children(
        &self,
        left: InterSpaxel,
        top: InterSpaxel,
        area: Rect<Spaxel>,
        target: Target,
    );
    fn draw_over_children(
        &self,
        left: InterSpaxel,
        top: InterSpaxel,
        area: Rect<Spaxel>,
        target: Target,
    );
    fn handle_click(
        &mut self,
        click_x: Spaxel,
        click_y: Spaxel,
        your_left: InterSpaxel,
        your_top: InterSpaxel,
        your_area: Rect<Spaxel>,
    ) -> Resolution;
}
