use macroquad::prelude::*;

use widgettes::v1::widgets::button::Button;
use widgettes::v1::Widget;

#[macroquad::main("Widget test")]
pub async fn main() {
    let camera = Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()) * 1.0,
        target: Vec2::ZERO,
        ..Default::default()
    };
    set_camera(&camera);

    let font = load_ttf_font("./FreeSans.ttf").await.unwrap();

    let button = Button::new("test", &font, 40, BROWN, WHITE);
    let (w, h) = button.get_dimensions();
    println!("w:{w} h:{h}");

    loop {
        clear_background(DARKBLUE);

        //let mouse_pos: Vec2 = mouse_position().into();
        button.draw(0.0, 0.0, w.ceil(), h.ceil());
        next_frame().await;
    }
}

/*

...*****
.**....
.*
*.
*.
*.

*/
