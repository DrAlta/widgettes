use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
// Assuming you have your other structs defined somewhere
// use crate::{ApptInfo, StartTime, EndTime}; 

#[derive(Debug, Clone)]
pub struct ApptInfo {
    pub start_time: StartTime,
    pub end_time: EndTime,
}

pub const TIME_BAR_WIDTH: i32 = 10; // Changed to i32 for consistency with Point
pub const NUM_COLUMNS: usize = 5; 

type EndTime = i64;
type StartTime = i64;

// The function signature now accepts a DrawTarget and returns a Result
pub fn draw_time_bars<T: DrawTarget<Color = Rgb565>>(
    offset: Point, // Changed from Vec2 to Point
    mut appointments: Vec<ApptInfo>,
    target: &mut T // Added DrawTarget
) -> Result<(), <T as DrawTarget>::Error> {

    let thickness = 1u32; // Stroke width is u32

    appointments.sort_by_key(|appt| appt.end_time);
    
    let mut bars: [Vec<(StartTime, EndTime)>; 5] = core::array::from_fn(|_| Vec::new());
    
    let mut end_times = vec![0; NUM_COLUMNS];
    
    for appt in appointments {
        let mut best_column_maybe = None;
        let mut min_gap = i64::MAX;

        for i in 0..NUM_COLUMNS {
            let gap = appt.start_time - end_times[i];
            if gap >= 0 && gap < min_gap {
                min_gap = gap;
                best_column_maybe = Some(i);
            }
        }

        let Some(best_column) = best_column_maybe else {
            println!("failed to find column");
            continue;
        };

        end_times[best_column] = appt.end_time;
        bars[best_column].push((appt.start_time, appt.end_time));
    }

    // Define the style once, using embedded_graphics colors and thickness
    let line_style = PrimitiveStyle::with_stroke(Rgb565::WHITE, thickness);

    for (col_idx, col) in bars.iter().enumerate() {
        for bar in col {
            let x = col_idx as i32 * TIME_BAR_WIDTH; // Use i32
            let y1 = bar.0 as i32;
            let y2 = bar.1 as i32;
            
            let start_point = offset + Point::new(x, y1);
            let end_point = offset + Point::new(x, y2);

            // 1. Vertical time bar (from start to end)
            Line::new(start_point, end_point)
                .into_styled(line_style)
                .draw(target)?;

            // 2. Top horizontal line
            Line::new(
                start_point,
                start_point + Point::new(TIME_BAR_WIDTH - (1 + thickness as i32), 0)
            )
            .into_styled(line_style)
            .draw(target)?;

            // 3. Bottom horizontal line
            Line::new(
                end_point,
                end_point + Point::new(TIME_BAR_WIDTH - (1 + thickness as i32), 0)
            )
            .into_styled(line_style)
            .draw(target)?;
        }
    }
    
    Ok(()) // Must return Ok(()) now
}

/* hand coded
pub fn draw_time_bars(offset: Vec2, mut appointments: Vec<ApptInfo>) {
    let thickness = 1.0;
    appointments.sort_by_key(|appt| appt.end_time); // Sort by end time

    let mut bars: [Vec<(i64, i64)>; 5] = core::array::from_fn(|_| Vec::new());

    let mut end_times = vec![0; NUM_COLUMNS]; // Track end times per column

    for appt in appointments {
        // Find the column where the gap from the last appointment is smallest
        let mut best_column_maybe = None;
        let mut min_gap = i64::MAX;

        for i in 0..NUM_COLUMNS {
            let gap = appt.start_time - end_times[i];
            if gap >= 0 && gap < min_gap {
                min_gap = gap;
                best_column_maybe = Some(i);
            }
        }

        let Some(best_column) = best_column_maybe else {
            println!("failed to find column");
            continue;
        };
        // Update the column's end time
        end_times[best_column] = appt.end_time;

        bars[best_column].push((appt.start_time, appt.end_time));
    }
    for (col_idx, col) in bars.iter().enumerate() {
        for bar in col {
            let x = col_idx as f32 * TIME_BAR_WIDTH;
            let y1 = bar.0 as f32;
            let y2 = bar.1 as f32;

            draw_line(offset.x + x, offset.y + y1,
                offset.x + x, offset.y + y2,
                thickness, WHITE); // Vertical bar

                println!("start:{x}:{y1}: {y2}");

            // **Draw horizontal top line**
            draw_line(
                offset.x + x, offset.y + y1,
                offset.x + x + TIME_BAR_WIDTH - (1.0 + thickness), offset.y + y1,
                thickness, WHITE);

            // **Draw horizontal bottom line**
            draw_line(offset.x + x, offset.y + y2,
                offset.x + x + TIME_BAR_WIDTH - (1.0 + thickness), offset.y + y2,
                thickness, WHITE);
        }
    }
}

pub fn find_overlaps(point: (i64, i64), bars: &[Vec<(i64, i64)>; NUM_COLUMNS]) -> [bool; NUM_COLUMNS] {
    let mut ret = [false; NUM_COLUMNS];

    for (bar_idx, bar) in bars.iter().enumerate() {
        for (start, end) in bar{
            ret[bar_idx] = &point.0 <= end && &point.1 >= start;
        }
    }
    ret
}
*/
