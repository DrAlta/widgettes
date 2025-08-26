use std::collections::HashMap;

use crate::v2::layout::{Layout, LayoutResponse, Rect, Resolution, Splat};

pub trait Widget<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash, Target> {
    fn layout(
        &self,
        offered: Rect<Spaxel>,
        callback: Option<Layout<InterSpaxel, Spaxel, ChildId>>,
        children_responses: HashMap::<ChildId, Splat<InterSpaxel, Spaxel, ChildId>>,
        children: Vec<ChildId>,
    ) -> LayoutResponse<InterSpaxel, Spaxel, ChildId>;
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
