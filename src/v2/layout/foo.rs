use std::collections::HashMap;

use crate::v2::{layout::{LayoutResponse, Rect, Resolution, Splat, Widget}, Layout};



#[derive(Debug)]
pub enum Foo<Spaxel> {
    Base(Rect<Spaxel>),
    HBox,
}

impl<Target> Widget<i32, i32, usize, Target> for Foo<i32> {
    fn draw_under_children(
        &self,
        _left: i32,
        _top: i32,
        _area: Rect<i32>,
        _target: Target,
    ) {
        todo!()
    }

    fn draw_over_children(
        &self,
        _left: i32,
        _top: i32,
        _area: Rect<i32>,
        _target: Target,
    ) {
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
        callback: Option<super::Layout<i32, i32, usize>>,
        mut children_response: HashMap::<usize, super::Splat<i32, i32, usize>>,
        children: Vec<usize>,
        ) -> LayoutResponse<i32, i32, usize> {
            let mut callback = if let Some(callback) = callback {
                callback
            } else {
                Layout{ left: 0, top: 0, area: offered.clone(), children: HashMap::new() }
            };
            match self {
            Foo::Base(rect) => LayoutResponse::Layout(Splat{rect: rect.clone(), childrens_layouts:HashMap::new() }),
            Foo::HBox => {
                let children_to_layout: Vec<_> = children.into_iter().filter(
                    |y|
                    {
                        ! children_response.contains_key(y)
                    }
                )
                .collect();
                children_response.retain(|k,v| {
                    let Some(previous_layout) =  callback.children.get(k) else {
                        return true
                    };
                    v.rect != previous_layout.area || v.childrens_layouts != previous_layout.children
                });
                for (child_id, splat) in children_response {
                    callback.children.insert(child_id, super::Layout { left: 0, top: 0, area: splat.rect, children: splat.childrens_layouts });
                }
                let mut used = 0;
                let mut height = 0;
                for child_layout in callback.children.values() {
                    used += child_layout.area.width.max(0);
                    height = height.max(child_layout.area.height);
                }
                if children_to_layout.is_empty() {
                    //calc my layout
                    LayoutResponse::Layout(Splat { rect: Rect { width: used, height }, childrens_layouts: callback.children })
                } else {
                    let free = offered.width - used;
                    let share = free / children_to_layout.len() as i32;
                    LayoutResponse::RequestLayoutOfChildren { 
                        callback, 
                        children_to_layout: children_to_layout.into_iter()
                        .map(
                            |child_id|
                            {
                                (child_id, Rect{width: share, height})
                            }
                        )
                        .collect()
                    }
                }
            }
        }
    }   

}