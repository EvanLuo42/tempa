use crate::event::Event;
use crate::widgets::{Position, Widget};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Debug;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

pub struct Button<C> {
    position: Point,
    size: Size,
    label: String,
    on_click: Option<Box<dyn FnMut()>>,
    stroke_color: Option<C>,
    fill_color: Option<C>,
    text_color: C
}

impl<C> Button<C> {
    pub fn on_click<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static
    {
        self.on_click = Some(Box::new(callback));
    }
}

impl<C> Position for Button<C> {
    fn position(&self) -> Point {
        self.position
    }
}

impl<C> crate::widgets::Size for Button<C> {
    type Unit = Size;

    fn size(&self) -> Size {
        self.size
    }
}

impl<Display, C> Widget<Display, C> for Button<C>
where
    Display: DrawTarget<Color = C>,
    C: PixelColor + Debug
{
    fn draw(&self, display: &mut Display) -> Result<(), Display::Error> {
        let mut rect_style = PrimitiveStyle::new();
        rect_style.fill_color = self.fill_color;
        rect_style.stroke_color = self.stroke_color;
        let text_style = MonoTextStyle::new(&FONT_6X9, self.text_color);
        Rectangle::new(self.position, self.size)
            .into_styled(rect_style)
            .draw(display)?;
        Text::new(&self.label, self.position, text_style)
            .draw(display)?;
        Ok(())
    }

    fn on_event(&mut self, _event: &Event) {
        todo!()
    }
}

pub struct ButtonBuilder<C> {
    position: Point,
    size: Size,
    label: String,
    on_click: Option<Box<dyn FnMut()>>,
    stroke_color: Option<C>,
    fill_color: Option<C>,
    text_color: C
}

impl<C: PixelColor> ButtonBuilder<C> {
    pub fn new(text_color: C) -> ButtonBuilder<C> {
        ButtonBuilder {
            position: Default::default(),
            size: Default::default(),
            label: "".to_string(),
            on_click: None,
            stroke_color: None,
            fill_color: None,
            text_color,
        }
    }

    pub fn position(mut self, x: i32, y: i32) -> ButtonBuilder<C> {
        self.position = Point::new(x, y);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> ButtonBuilder<C> {
        self.size = Size::new(width, height);
        self
    }

    pub fn label(mut self, label: &str) -> ButtonBuilder<C> {
        self.label = label.into();
        self
    }

    pub fn stroke_color(mut self, stroke_color: C) -> ButtonBuilder<C> {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn fill_color(mut self, fill_color: C) -> ButtonBuilder<C> {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn build(self) -> Button<C> {
        Button {
            position: self.position,
            size: self.size,
            label: self.label,
            on_click: None,
            stroke_color: self.stroke_color,
            fill_color: self.fill_color,
            text_color: self.text_color,
        }
    }
}