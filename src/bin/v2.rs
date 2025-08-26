use std::collections::HashMap;
use macroquad::texture::Image;
use widgettes::v2::{foo::Foo, LayoutResponse, Rect, Splat, Widget};

pub fn do_layout<InterSpaxel, Spaxel, ChildId, Target, W>(
    world: &HashMap<ChildId, W>,
    world_children: &HashMap<ChildId, Vec<ChildId>>,
    start: &ChildId,
    offered: Rect<Spaxel>,
) -> Splat<InterSpaxel, Spaxel, ChildId>
where
    ChildId: Eq + std::hash::Hash + Clone,
    W: Widget<InterSpaxel, Spaxel, ChildId, Target>,
    Spaxel: Clone,
    InterSpaxel: Clone,
{

    let children = if let Some(children) = world_children.get(&start) {
        children.clone()
    } else {
        Vec::new()
    };

    // Initial layout call with empty children response
    let mut response = world.get(start).unwrap().layout(
        offered.clone(),
        None,
        HashMap::new(),
        children.clone(),
    );

    loop {
        match response {
            LayoutResponse::Layout (splat) => return splat,
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


fn main(){
    let world = HashMap::from([
        (1_usize, Foo::HBox),
        (2, Foo::Base(Rect { width: 5, height: 10 })),
        (3, Foo::Base(Rect { width: 10, height: 5 })),
    ]);
    let world_children = HashMap::from([(1, vec![2,3])]);
    let a = do_layout::<_,_,_,Image,_>(&world, &world_children, &1, Rect { width: 50, height: 50 });
    println!("{:#?}", a);
}