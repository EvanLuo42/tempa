use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::PixelColor;
use crate::event::Event;

pub mod button;

pub trait Widget<Display, C>
where
    Display: DrawTarget<Color = C>,
    C: PixelColor
{
    fn draw(&self, display: &mut Display) -> Result<(), Display::Error>;
    fn on_event(&mut self, event: &Event);
}

pub trait Position {
    fn position(&self) -> Point;
}

pub trait Size {
    type Unit;
    fn size(&self) -> embedded_graphics::geometry::Size;
}