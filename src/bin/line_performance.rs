use macroquad::prelude::*;
use macroquad::texture::Image;
use std::time::Instant;

use widgettes::v2::Graphics;

fn generate_random_lines(count: usize, width: i32, height: i32) -> Vec<((i32, i32), (i32, i32))> {
    let mut lines = Vec::with_capacity(count);
    for _ in 0..count {
        let x0 = rand::gen_range(0, width);
        let y0 = rand::gen_range(0, height);
        let x1 = rand::gen_range(0, width);
        let y1 = rand::gen_range(0, height);
        lines.push(((x0, y0), (x1, y1)));
    }
    lines
}

#[macroquad::main("Line Performance Comparison")]
async fn main() {
    let width = 800;
    let height = 600;
    let line_count = 10_000;

    // Create blank images for each test
    let mut image_standard = Image::gen_image_color(width, height, BLACK);
    let mut image_custom = Image::gen_image_color(width, height, BLACK);

    // Generate random lines
    let lines = generate_random_lines(line_count, width as i32, height as i32);

    // Measure draw_line() performance
    let start_standard = Instant::now();
    for &((x0, y0), (x1, y1)) in &lines {
        image_standard.draw_line(x0, y0, x1, y1, WHITE);
    }
    let duration_standard = start_standard.elapsed();

    // Measure draw_line_ex() performance
    let start_custom = Instant::now();
    for &((x0, y0), (x1, y1)) in &lines {
        image_custom.draw_line_ex(x0, y0, x1, y1, 0, WHITE);
    }
    let duration_custom = start_custom.elapsed();

    println!("draw_line() took:    {:?}", duration_standard);
    println!("draw_line_ex() took: {:?}", duration_custom);

    // Display the result images side by side
    let texture_standard = Texture2D::from_image(&image_standard);
    let texture_custom = Texture2D::from_image(&image_custom);

    loop {
        clear_background(GRAY);
        draw_texture(&texture_standard, 0.0, 0.0, WHITE);
        draw_texture(&texture_custom, width as f32 / 2.0, 0.0, WHITE);
        draw_text("Left: draw_line()", 20.0, 20.0, 20.0, WHITE);
        draw_text("Right: draw_line_ex()", width as f32 / 2.0 + 20.0, 20.0, 20.0, WHITE);
        next_frame().await;
    }
}
