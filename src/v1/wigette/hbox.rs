use crate::v1::{LayoutRet, Wigette};
impl Wigette {
    pub fn h_size(
        my_min_width: u32,
        my_min_height: u32,
        children: &mut Vec<Wigette>,
        padding: u32,
    ) -> LayoutRet {
        let my_min_width = my_min_width as i64;
        let my_min_height = my_min_height as i64;
        let padding = padding as i64;

        let mut needed_width = padding * children.len() as i64;
        let mut expanding_children = Vec::<usize>::new();
        let mut expanding_x_count = 0_i64;

        let mut height = my_min_height;
        for (index, child) in children.iter_mut().enumerate() {
            child.update_size();
            needed_width += child.get_min_width() as i64;
            height = height.max(child.get_min_height() as i64 + (padding * 2));
            if child.get_expand_width() {
                expanding_x_count += 1;
            }
            if child.get_expand_width() || child.get_expand_height() {
                expanding_children.push(index);
            }
        }
        /*
        if needed_width >= my_min_width {
            return LayoutRet {
                height: height as u32,
                width: needed_width as u32,
            }
        }
        */
        // need to check is needed is larger than min then skip expanding chidren
        let foo_x = {
            if expanding_x_count != 0 {
                let diff_width = my_min_width as i64 - needed_width as i64;
                let mut foo = diff_width / expanding_x_count;
                // round foo up
                if (diff_width % expanding_x_count) != 0 {
                    needed_width += expanding_x_count;
                    foo += 1;
                }
                // set the actual dim on this widget
                foo.max(0)
            } else {
                0
            }
        };
        let width = needed_width.max(my_min_width);

        for index in expanding_children {
            let child = children.get_mut(index).unwrap();

            let childs_min_width = child.get_min_width();

            let childs_min_y = child.get_min_height();
            let desired_width = if child.expand_width {
                childs_min_width + (foo_x as u32)
            } else {
                childs_min_width
            };
            let desired_heigth = if child.expand_height {
                (height - (padding * 2)) as u32
            } else {
                childs_min_y
            };
            child.set_size(desired_width, desired_heigth);
        }
        LayoutRet {
            height: height as u32,
            width: width as u32,
            distended_width: needed_width as u32,
            distended_height: height as u32,
        }
    }
}
