use crate::v1::{LayoutRet, Wigette};
impl Wigette {
    pub fn v_size(
        my_min_width: usize,
        my_min_height: usize,
        children: &mut Vec<Wigette>,
        padding: i64,
    ) -> LayoutRet {
        let mut needed_height = padding as usize * children.len();
        let mut expanding_children = Vec::<usize>::new();
        let mut expanding_y_count = 0_usize;

        let mut width = my_min_width;
        for (index, child) in children.iter_mut().enumerate() {
            child.update_size();
            needed_height += child.get_height() as usize + (padding * 2) as usize;
            width = width.max(child.get_width() as usize);
            if child.get_expand_height() {
                expanding_y_count += 1;
            }
            if child.get_expand_width() || child.get_expand_height() {
                expanding_children.push(index);
            }
        }

        let (height, foo_y) = {
            if expanding_y_count != 0 {
                let diff_y = my_min_height - needed_height;
                let mut foo = diff_y / expanding_y_count;
                // round foo up
                if (diff_y % expanding_y_count) != 0 {
                    needed_height += expanding_y_count;
                    foo += 1;
                }
                //set the actual dim on this widget
                (needed_height.max(my_min_height), foo)
            } else {
                (my_min_height, 0)
            }
        };

        for index in expanding_children {
            let child = children.get_mut(index).unwrap();

            let childs_min_width = child.get_min_width();

            let childs_min_height = child.get_min_height();
            let desired_width = if child.get_expand_width() {
                (width - (padding * 2) as usize) as u32
            } else {
                childs_min_width
            };
            let desired_height = if child.get_expand_width() {
                childs_min_height + (foo_y as u32)
            } else {
                childs_min_height
            };
            child.set_size(desired_width, desired_height);
        }
        LayoutRet {
            height: height as u32,
            width: width as u32,
            distended_width: width as u32,
            distended_height: needed_height as u32,
        }
    }
}
