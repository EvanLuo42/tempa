use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};
use std::thread;
use std::time::Duration;
use tempa_ui::event::EventLoop;
use tempa_ui::widgets::button::ButtonBuilder;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let mut event_loop = EventLoop::new();
    let button = ButtonBuilder::new(BinaryColor::On)
        .position(50, 50)
        .size(16, 16)
        .label("Button")
        .stroke_color(BinaryColor::Off)
        .fill_color(BinaryColor::Off)
        .build();
    event_loop.add_widget(Box::new(button));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();
    let mut window = Window::new("Hello World", &output_settings);
    window.update(&display);
    'running: loop {
        display.clear(BinaryColor::On)?;
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }
        event_loop.render(&mut display)?;
        window.update(&display);
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}
