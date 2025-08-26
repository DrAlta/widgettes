// need to get it choosing between 3 pixel boarders and 4(alt) boarders
// I thikn about abstrating the spacing code so I can re use for the x and c spacing

use crate::v2::Graphics;
use macroquad::prelude::{Image, BLUE, RED, WHITE};
pub fn draw_table(
    image: &mut Image,
    rows: Vec<i8>,
    columns: Vec<i8>,
    cell_width: usize,
    cell_height: usize,
) {
    let column_count = columns.len();
    println!("column_count:{column_count}");

    let (extra, cells_needed) = {
        let needed_x_pixels = (cell_width * column_count) + ((column_count - 1) * 3);
        let cells_needed_1 = needed_x_pixels % cell_width;
        let rem = needed_x_pixels % cell_width;
        (
            cell_width - rem,
            if rem == 0 {
                cells_needed_1
            } else {
                cells_needed_1 + 1
            },
        )
    };

    let (alt_extra, alt_cells_needed) = {
        let alt_needed_x_pixels = (cell_width * column_count) + ((column_count - 1) * 4);
        let alt_cells_needed_1 = alt_needed_x_pixels % cell_width;
        let rem = alt_needed_x_pixels % cell_width;
        (
            cell_width - rem,
            if rem == 0 {
                alt_cells_needed_1
            } else {
                alt_cells_needed_1 + 1
            },
        )
    };
    println!("cells_needed:{cells_needed}, extra:{extra}");
    if alt_cells_needed < cells_needed {
        //use alt cells
    } else {
        //use cells
    }

    let colors = [RED, BLUE];
    let mut turtle_x = 3;
    let half_extra = extra as i32 / 2;
    let todo = (rows, alt_extra, half_extra);
    let mut turtle_y = 3; // + (extra as i32 - half_extra);
    let mut idx = 0;
    image.draw_line(1, 1, 1, turtle_y + cell_height as i32 + 1, WHITE);
    image.draw_line(2, 1, (1 * cell_width) as i32 + extra as i32 + 2, 1, WHITE);
    for count in columns {
        let this_cell_width = cell_width as i32 * count as i32;
        image.draw_rect(
            turtle_x,
            turtle_y,
            this_cell_width,
            cell_height as i32,
            colors[idx % 2],
        );
        turtle_x += this_cell_width;
        let x = turtle_x + 1;
        image.draw_line(x, turtle_y - 1, x, turtle_y + cell_height as i32 + 1, WHITE);
        turtle_x += 3;
        idx += 1;
    }
    //    image
}
