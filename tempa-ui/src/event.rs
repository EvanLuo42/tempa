use crate::widgets::Widget;
use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::PixelColor;

pub enum Event {
    Test
}

pub struct EventLoop<Display, C> {
    widgets: Vec<Box<dyn Widget<Display, C>>>
}

impl<Display, C> EventLoop<Display, C>
where
    Display: DrawTarget<Color = C>,
    C: PixelColor
{
    pub fn new() -> Self {
        EventLoop {
            widgets: Vec::new()
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget<Display, C>>) {
        self.widgets.push(widget);
    }

    pub fn handle_event(&mut self, event: Event) {
        for widget in &mut self.widgets {
            widget.on_event(&event);
        }
    }

    pub fn render(&self, display: &mut Display) -> Result<(), Display::Error> {
        for widget in &self.widgets {
            widget.draw(display)?;
        }
        Ok(())
    }
}