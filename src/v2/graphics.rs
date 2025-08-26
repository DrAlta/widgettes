use crate::v2::util::wrapped_rem;
use macroquad::{color::Color, texture::Image}; // Importing types for color and image manipulation // Custom modulo function that wraps the remander of negative numbers around

/// Trait defining basic graphics operations
pub trait Graphics {
    /// Returns the width of the drawable surface
    fn width(&self) -> i32;
    /// Returns the height of the drawable surface
    fn height(&self) -> i32;
    /// Draws a single pixel at (x, y) with the specified color
    fn draw_pixel(&mut self, x: i32, y: i32, color: Color);
    /// Draws a filled rectangle starting at (x, y) with given dimensions and color
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        for row in 0..height {
            let y1 = y + row;
            for column in 0..width {
                // Draw each pixel in the rectangle
                self.draw_pixel(x + column, y1, color);
            }
        }
    }

    // Draws a line using a modified Bresenham algorithm with custom error offset
    fn draw_line_ex(
        &mut self,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
        mut start_error: i32,
        color: Color,
    ) {
        // Calculate absolute differences in x and y
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        // Determine step direction for x and y
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        // Determine which axis is dominant and set up stepping logic
        let (steps, major_step, minor_step, minor_delta) = if dx > dy {
            (dx, (sx, 0), (0, sy), dy) // X is dominant
        } else {
            (dy, (0, sy), (sx, 0), dx) // Y is dominant
        };

        let threshold = steps; // Used as the denominator for error correction
        start_error = wrapped_rem(start_error, threshold); // Wrap error to stay within bounds

        // Loop through each step to draw the line
        for _ in 0..=steps {
            self.draw_pixel(x0, y0, color); // Draw current pixel

            start_error += minor_delta; // Accumulate error
            if start_error >= threshold {
                start_error -= threshold;
                // Step along the minor axis when error exceeds threshold
                x0 += minor_step.0;
                y0 += minor_step.1;
            }
            // Always step along the major axis
            x0 += major_step.0;
            y0 += major_step.1;
        }
    }
    /// Draws a line on the given image using Bresenham's algorithm.
    fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs(); // Absolute horizontal distance
        let dy = -(y1 - y0).abs(); // Negative vertical distance for error calculation
        let sx = if x0 < x1 { 1 } else { -1 }; // Step direction in x
        let sy = if y0 < y1 { 1 } else { -1 }; // Step direction in y
        let mut err = dx + dy; // Initial error value

        for _ in 0..dx.max(dy) + 1 {
            // Only draw pixel if within bounds
            if x0 >= 0 && y0 >= 0 && x0 < self.width() && y0 < self.height() {
                self.draw_pixel(x0, y0, color);
            }

            let e2 = 2 * err; // Double error for comparison
            if e2 >= dy {
                err += dy;
                x0 += sx; // Step in x direction
            }
            if e2 <= dx {
                err += dx;
                y0 += sy; // Step in y direction
            }
        }
    }
}

// Implementation of the Graphics trait for macroquad's Image type
impl Graphics for Image {
    fn width(&self) -> i32 {
        self.width as i32
    }

    fn height(&self) -> i32 {
        self.height as i32
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 {
            self.set_pixel(x as u32, y as u32, color);
        }
    }
}
