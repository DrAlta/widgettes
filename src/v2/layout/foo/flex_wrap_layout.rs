type InterSpaxel = i32;
type Spaxel = i32; 

use std::collections::HashMap;

use crate::v2::{Layout, LayoutResponse, Rect, Splat};


#[derive(Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}


pub fn flex_wrap_layout<ChildId: Clone +  Eq + std::hash::Hash>(
    direction: FlexDirection,
    offered: Rect<Spaxel>,
    callback: Option<Layout<Spaxel, Spaxel, ChildId>>,
    mut children_response: HashMap<ChildId, Splat<Spaxel, Spaxel, ChildId>>,
    children: Vec<ChildId>,
) -> LayoutResponse<Spaxel, Spaxel, ChildId> {
    let mut callback = callback.unwrap_or(Layout {
        left: 0,
        top: 0,
        area: offered.clone(),
        children: HashMap::new(),
    });

    // Layout children that haven't been processed yet
    let children_to_layout: Vec<_> = children
        .iter()
        .filter(|&id| !children_response.contains_key(id))
        .cloned()
        .collect();

    children_response.retain(|k, v| match callback.children.get(k) {
        Some(prev) => v.rect != prev.area || v.childrens_layouts != prev.children,
        None => true,
    });


    // Insert already laid-out children
    for (child_id, splat) in children_response.drain() {
        callback.children.insert(
            child_id,
            Layout {
                left: 0,
                top: 0,
                area: splat.rect,
                children: splat.childrens_layouts,
            },
        );
    }

    // Perform wrapping layout
    let mut x = 0;
    let mut y = 0;
    let mut line_thickness = 0;
    let mut max_width = 0;
    let mut max_height = 0;

    for (child_id, layout) in &mut callback.children {
        let size = layout.area.clone();

        match direction {
            FlexDirection::Row => {
                if x + size.width > offered.width {
                    x = 0;
                    y += line_thickness;
                    line_thickness = 0;
                }
                layout.left = x;
                layout.top = y;
                x += size.width;
                line_thickness = line_thickness.max(size.height);
                max_width = max_width.max(x);
                max_height = y + line_thickness;
            }
            FlexDirection::Column => {
                if y + size.height > offered.height {
                    y = 0;
                    x += line_thickness;
                    line_thickness = 0;
                }
                layout.left = x;
                layout.top = y;
                y += size.height;
                line_thickness = line_thickness.max(size.width);
                max_height = max_height.max(y);
                max_width = x + line_thickness;
            }
        }
    }

    LayoutResponse::Layout(Splat {
        rect: Rect {
            width: max_width,
            height: max_height,
        },
        childrens_layouts: callback.children,
    })
}
