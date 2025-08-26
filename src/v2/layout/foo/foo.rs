use std::collections::HashMap;

use crate::v2::{
    foo::{linear_layout, Axis, Callback, ChildId, InterSpaxel, Spaxel},
    Layout, LayoutResponse, Rect, Resolution, Splat, Widget,
};

#[derive(Debug)]
pub enum Foo<Spaxel> {
    Base(Rect<Spaxel>),
    HBox,
    VBox,
}

impl<Target> Widget<InterSpaxel, Spaxel, ChildId, Callback, Target> for Foo<i32> {
    fn draw_under_children(&self, _left: i32, _top: i32, _area: Rect<i32>, _target: Target) {
        todo!()
    }

    fn draw_over_children(&self, _left: i32, _top: i32, _area: Rect<i32>, _target: Target) {
        todo!()
    }

    fn handle_click(
        &mut self,
        _click_x: i32,
        _click_y: i32,
        _your_left: i32,
        _your_top: i32,
        _your_area: Rect<i32>,
    ) -> Resolution {
        todo!()
    }

    fn layout(
        &self,
        offered: Rect<i32>,
        callback: Option<Callback>,
        mut children_response: HashMap<usize, Splat<InterSpaxel, Spaxel, ChildId>>,
        children: Vec<usize>,
    ) -> LayoutResponse<InterSpaxel, Spaxel, ChildId, Callback> {
        let mut callback = if let Some(callback) = callback {
            callback
        } else {
            Layout {
                left: 0,
                top: 0,
                area: offered.clone(),
                children: HashMap::new(),
            }
        };
        match self {
            Foo::Base(rect) => LayoutResponse::Layout(Splat {
                rect: rect.clone(),
                childrens_layouts: HashMap::new(),
            }),
            Foo::HBox => linear_layout(
                Axis::Horizontal,
                offered,
                callback.into(),
                children_response,
                children,
            ),
            Foo::VBox => linear_layout(
                Axis::Vertical,
                offered,
                callback.into(),
                children_response,
                children,
            ),
        }
    }

    fn ideal_size(&self, _field: Rect<i32>) -> Rect<i32> {
        todo!()
    }

    fn min_size(&self) -> Rect<i32> {
        todo!()
    }
}
