pub trait Widget {
    fn draw(&self, x: f32, y: f32, w: f32, h: f32);
    fn get_height(&self) -> f32 {
        self.get_dimensions().1
    }
    fn get_width(&self) -> f32 {
        self.get_dimensions().0
    }
    fn get_dimensions(&self) -> (f32, f32);
}
