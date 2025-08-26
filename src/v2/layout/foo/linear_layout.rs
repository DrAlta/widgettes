use std::collections::HashMap;

use crate::v2::{
    foo::{Callback, ChildId, InterSpaxel, Spaxel},
    Layout, LayoutResponse, Rect, Splat,
};

#[derive(Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

pub fn linear_layout(
    axis: Axis,
    offered: Rect<Spaxel>,
    callback: Option<Callback>,
    mut children_response: HashMap<usize, Splat<InterSpaxel, Spaxel, ChildId>>,
    children: Vec<ChildId>,
) -> LayoutResponse<InterSpaxel, Spaxel, ChildId, Callback> {
    let mut callback = callback.unwrap_or(Layout {
        left: 0,
        top: 0,
        area: offered.clone(),
        children: HashMap::new(),
    });

    let children_to_layout: Vec<_> = children
        .iter()
        .filter(|id| !children_response.contains_key(id))
        .cloned()
        .collect();

    children_response.retain(|k, v| match callback.children.get(k) {
        Some(prev) => v.rect != prev.area || v.childrens_layouts != prev.children,
        None => true,
    });

    for (child_id, splat) in children_response {
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

    let mut used = 0;
    let mut cross = 0;

    for layout in callback.children.values() {
        match axis {
            Axis::Horizontal => {
                used += layout.area.width.max(0);
                cross = cross.max(layout.area.height);
            }
            Axis::Vertical => {
                used += layout.area.height.max(0);
                cross = cross.max(layout.area.width);
            }
        }
    }

    if children_to_layout.is_empty() {
        let rect = match axis {
            Axis::Horizontal => Rect {
                width: used,
                height: cross,
            },
            Axis::Vertical => Rect {
                width: cross,
                height: used,
            },
        };
        let mut turtle = 0;
        match axis {
            Axis::Horizontal => {
                for (_, c) in &mut callback.children {
                    c.left = turtle;
                    turtle += c.area.width;
                }
            }
            Axis::Vertical => {
                for (_, c) in &mut callback.children {
                    c.top = turtle;
                    turtle += c.area.height;
                }
            }
        }

        LayoutResponse::Layout(Splat {
            rect,
            childrens_layouts: callback.children,
        })
    } else {
        let free = match axis {
            Axis::Horizontal => offered.width - used,
            Axis::Vertical => offered.height - used,
        };
        let share = free / children_to_layout.len() as i32;

        let children_to_layout = children_to_layout
            .into_iter()
            .map(|id| {
                let rect = match axis {
                    Axis::Horizontal => Rect {
                        width: share,
                        height: cross,
                    },
                    Axis::Vertical => Rect {
                        width: cross,
                        height: share,
                    },
                };
                (id, rect)
            })
            .collect();

        LayoutResponse::RequestLayoutOfChildren {
            callback,
            children_to_layout,
        }
    }
}
