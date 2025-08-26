use macroquad::prelude::*;
use std::collections::HashMap;
use widgettes::v2::{foo::Foo, Layout, LayoutResponse, Rect, Splat, Widget};

pub fn do_layout<InterSpaxel, Spaxel, ChildId, Target, W, Callback>(
    world: &HashMap<ChildId, W>,
    world_children: &HashMap<ChildId, Vec<ChildId>>,
    start: &ChildId,
    offered: Rect<Spaxel>,
) -> Splat<InterSpaxel, Spaxel, ChildId>
where
    ChildId: Eq + std::hash::Hash + Clone,
    W: Widget<InterSpaxel, Spaxel, ChildId, Callback, Target>,
    Spaxel: Clone,
    InterSpaxel: Clone,
{
    let children = if let Some(children) = world_children.get(&start) {
        children.clone()
    } else {
        Vec::new()
    };

    // Initial layout call with empty children response
    let mut response =
        world
            .get(start)
            .unwrap()
            .layout(offered.clone(), None, HashMap::new(), children.clone());

    loop {
        match response {
            LayoutResponse::Layout(splat) => return splat,
            LayoutResponse::RequestLayoutOfChildren {
                callback,
                children_to_layout,
            } => {
                let mut children_responses = HashMap::new();

                for (child_id, child_offered) in children_to_layout {
                    let child_splat = do_layout(world, world_children, &child_id, child_offered);

                    children_responses.insert(child_id.clone(), child_splat);
                }

                // Re-call layout with resolved children
                response = world.get(start).unwrap().layout(
                    offered.clone(),
                    Some(callback),
                    children_responses,
                    children.clone(),
                );
            }
        }
    }
}

#[macroquad::main("Time Bar Scheduler")]
async fn main() {
    let camera = Camera2D {
        //offset: vec2(-1.0, 1.0),
        //zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),// 1x
        zoom: vec2(4.0 / screen_width(), 4.0 / screen_height()), // 2x
        ..Default::default()
    };
    let world = HashMap::from([
        (1_usize, Foo::VBox),
        (
            2,
            Foo::Base(Rect {
                width: 5,
                height: 10,
            }),
        ),
        (
            3,
            Foo::Base(Rect {
                width: 10,
                height: 5,
            }),
        ),
    ]);
    let world_children = HashMap::from([(1, vec![2, 3])]);
    let a = do_layout::<_, _, _, Image, _, _>(
        &world,
        &world_children,
        &1,
        Rect {
            width: 50,
            height: 50,
        },
    );
    println!("{:#?}", a);
    let colors = [RED, BLUE, GREEN, PINK, YELLOW, ORANGE, PURPLE];
    loop {
        clear_background(DARKGRAY);

        set_camera(&camera);
        draw_rectangle(0.0, 0.0, a.rect.width as f32, a.rect.height as f32, WHITE);
        let mut idx = 0;
        for (_, l) in &a.childrens_layouts {
            foo(0.0, 0.0, l, &colors, &mut idx);
        }
        next_frame().await;
    }
}

fn foo<ChildId: Eq + std::hash::Hash>(
    x: f32,
    y: f32,
    layout: &Layout<i32, i32, ChildId>,
    colors: &[Color],
    idx: &mut usize,
) {
    let (x, y) = (x + layout.left as f32, y + layout.top as f32);
    draw_box(
        x,
        y,
        layout.area.width as f32,
        layout.area.height as f32,
        colors[*idx],
    );
    *idx += 1;
    for (_, l) in &layout.children {
        foo(x, y, l, colors, idx);
        *idx += 1;
    }
}

fn draw_box(x: f32, y: f32, width: f32, height: f32, color: Color) {
    draw_rectangle_lines(x, y, width, height, 1.0, color);
    draw_line(x, y, x + width, y + height, 1.0, color);
    draw_line(x, y + height, x + width, y, 1.0, color);
}
