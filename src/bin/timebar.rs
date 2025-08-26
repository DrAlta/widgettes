use macroquad::prelude::*;

use widgettes::v2::widgets::timebar::timebar::*;

#[macroquad::main("Time Bar Scheduler")]
async fn main() {
    /*
    println!(
        "{:?}",
        find_overlaps((10, 20), &[
        Vec::from([(0, 9)]),
        Vec::from([(21, 30)]),
        Vec::from([(0, 10)]),
        Vec::from([(20, 30)]),
        Vec::from([(0, 30)]),
        ]));
    */
    let appointments = vec![
        ApptInfo {
            start_time: 10,
            end_time: 20,
        },
        ApptInfo {
            start_time: 0,
            end_time: 12,
        },
        ApptInfo {
            start_time: 15,
            end_time: 30,
        },
    ];

    loop {
        clear_background(BLACK);
        let x = mouse_position();
        draw_time_bars(vec2(x.0, x.1), appointments.clone());

        next_frame().await;
    }
}
