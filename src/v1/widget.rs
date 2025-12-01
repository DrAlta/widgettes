use embedded_graphics::prelude::DrawTarget;

pub trait Widget<C> {
    fn draw<T: DrawTarget<Color = C>>(&self, x: i32, y: i32, w: u32, h: u32, draw_target: &mut T) -> Result<(), <T as DrawTarget>::Error>;
    fn get_height(&self) -> u32 {
        self.get_dimensions().1
    }
    fn get_width(&self) -> u32 {
        self.get_dimensions().0
    }
    fn get_dimensions(&self) -> (u32, u32);
}
