// use embedded_graphics_core prelude where most necessary traits are defined
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, Rectangle, PrimitiveStyle},
};

// The function signature now accepts a mutable reference to a DrawTarget
// We don't use the specific macroquad::Image type anymore, but whatever display/canvas you are using
pub fn draw_table<T: DrawTarget<Color = Rgb565>>(
    target: &mut T, 
    rows: Vec<i8>,
    columns: Vec<i8>,
    cell_width: u32, // use u32 for dimensions
    cell_height: u32, // use u32 for dimensions
) -> Result<(), <T as DrawTarget>::Error> { // Return Result

    let column_count = columns.len() as u32; // use u32
    println!("column_count:{column_count}");

    // All dimension calculations should use u32 or i32 for consistency with embedded-graphics
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

    // Define colors using embedded_graphics Rgb565
    let colors = [Rgb565::RED, Rgb565::BLUE];
    let white = Rgb565::WHITE;
    
    // Coordinates use i32
    let mut turtle_x = 3i32; 
    let half_extra = extra as i32 / 2;
    let todo = (rows, alt_extra, half_extra);
    let mut turtle_y = 3i32; 
    let mut idx = 0;

    // Define a style for the white border lines
    let border_style = PrimitiveStyle::with_stroke(white, 1);

    // Draw the initial outer lines using Line primitives and the target
    Line::new(Point::new(1, 1), Point::new(1, turtle_y + cell_height as i32 + 1))
        .into_styled(border_style)
        .draw(target)?;
        
    Line::new(Point::new(2, 1), Point::new((1 * cell_width as i32) + extra as i32 + 2, 1))
        .into_styled(border_style)
        .draw(target)?;

    for count in columns {
        let this_cell_width = cell_width as i32 * count as i32;
        
        // Draw the colored cell using Rectangle primitive
        Rectangle::new(
            Point::new(turtle_x, turtle_y),
            Size::new(this_cell_width as u32, cell_height as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(colors[idx % 2]))
        .draw(target)?;

        turtle_x += this_cell_width;
        let x = turtle_x + 1;
        
        // Draw the vertical line separator
        Line::new(Point::new(x, turtle_y - 1), Point::new(x, turtle_y + cell_height as i32 + 1))
            .into_styled(border_style)
            .draw(target)?;
            
        turtle_x += 3;
        idx += 1;
    }
    
    Ok(()) // Return success
}
