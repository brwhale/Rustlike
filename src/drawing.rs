use graphics::{Context, Rectangle};
use opengl_graphics::{GlGraphics};
// implementations for piston library stuff
pub trait DrawRectangle {
    fn draw_rectangle(
        &mut self, 
        r: [f64; 4], 
        color: [f32; 4], c: &Context);
}

impl DrawRectangle for GlGraphics {
    fn draw_rectangle(
        &mut self, 
        r: [f64; 4], 
        color: [f32; 4], 
        c: &Context) {
        Rectangle::new(color)
            .draw(r, &c.draw_state, c.transform, self);
    }
}