///! A trait defines something that could be drawn
use crate::canvas::Canvas;
use crate::canvas::Result;

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas) -> Result<()>;
}

impl<T: Draw> Draw for &T {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        (*self).draw(canvas)
    }
}
