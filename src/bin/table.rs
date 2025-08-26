use macroquad::prelude::*;
use widgettes::v2::widgets::table::draw_table;

#[macroquad::main("Swag Table")]
async fn main() {
    let camera = Camera2D {
        offset: vec2(-1.0, 1.0),
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    };
    let width = 160;
    let height = 120;
    let mut image = Image {
        bytes: vec![[0, 0, 0, u8::MAX]; width as usize * height as usize]
            .into_iter()
            .flatten()
            .collect(),
        width,
        height,
    };

    draw_table(&mut image, vec![1, 1], vec![1, 1, 1], 8, 12);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);
    loop {
        clear_background(DARKGRAY);

        set_camera(&camera);
        draw_texture_ex(
            &texture,
            4.0,
            4.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width as f32 * 4.0, height as f32 * 4.0)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
