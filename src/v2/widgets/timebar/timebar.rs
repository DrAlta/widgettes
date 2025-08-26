use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct ApptInfo {
    pub start_time: StartTime,
    pub end_time: EndTime,
}

pub const TIME_BAR_WIDTH: f32 = 10.0;
pub const NUM_COLUMNS: usize = 5; // Max number of time bar columns

type EndTime = i64;
type StartTime = i64;

// this has been commited by Chad
pub fn draw_time_bars(offset: Vec2, mut appointments: Vec<ApptInfo>) {
    // Defines a function that takes a position offset and a vector of appointments.
    // The offset helps position the drawn bars correctly in the coordinate system.

    let thickness = 1.0;
    // Defines the thickness of the bars in the visualization.

    appointments.sort_by_key(|appt| appt.end_time);
    // Sorts appointments by their end times to process them in chronological order.
    // This ensures that overlapping appointments are handled properly in column assignment.

    let mut bars: [Vec<(StartTime, EndTime)>; 5] = core::array::from_fn(|_| Vec::new());
    // Initializes an array of 5 vectors, each storing (start_time, end_time) pairs for different columns.
    // This helps distribute appointments across multiple columns to prevent overlap.

    let mut end_times = vec![0; NUM_COLUMNS];
    // Tracks the latest end time in each column, helping to determine where new appointments should go.

    for appt in appointments {
        // Iterates through sorted appointments and assigns them to columns.

        let mut best_column_maybe = None;
        // Placeholder for the best column to place the appointment.

        let mut min_gap = i64::MAX;
        // Tracks the smallest gap between an appointment’s start time and the latest end time in a column.

        for i in 0..NUM_COLUMNS {
            // Loops through available columns to find the best placement.

            let gap = appt.start_time - end_times[i];
            // Calculates the time gap between the current appointment and the last scheduled appointment in this column.

            if gap >= 0 && gap < min_gap {
                // If the gap is valid (appointment does not overlap) and is the smallest found so far:
                min_gap = gap;
                best_column_maybe = Some(i);
            }
        }

        let Some(best_column) = best_column_maybe else {
            // If no suitable column is found, print an error and skip this appointment.
            println!("failed to find column");
            continue;
        };

        end_times[best_column] = appt.end_time;
        // Updates the latest end time for the chosen column.

        bars[best_column].push((appt.start_time, appt.end_time));
        // Stores the appointment times in the assigned column.
    }

    for (col_idx, col) in bars.iter().enumerate() {
        // Loops through each column and draws the corresponding time bars.

        for bar in col {
            // Iterates over appointments in this column.

            let x = col_idx as f32 * TIME_BAR_WIDTH;
            // Calculates the horizontal position based on column index.

            let y1 = bar.0 as f32;
            let y2 = bar.1 as f32;
            // Retrieves the start and end times for the vertical positioning.

            draw_line(
                offset.x + x,
                offset.y + y1,
                offset.x + x,
                offset.y + y2,
                thickness,
                WHITE,
            ); // Vertical time bar from start to end.

            draw_line(
                offset.x + x,
                offset.y + y1,
                offset.x + x + TIME_BAR_WIDTH - (1.0 + thickness),
                offset.y + y1,
                thickness,
                WHITE,
            );
            // Draws the **top horizontal line** of the appointment bar.

            draw_line(
                offset.x + x,
                offset.y + y2,
                offset.x + x + TIME_BAR_WIDTH - (1.0 + thickness),
                offset.y + y2,
                thickness,
                WHITE,
            );
            // Draws the **bottom horizontal line** of the appointment bar.
        }
    }
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
