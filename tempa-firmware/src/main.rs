#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pull};
use embassy_time::Timer;

pub use defmt_rtt;
use embassy_rp::adc::{Adc, Channel, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
pub use panic_probe;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripheral = embassy_rp::init(Default::default());
    let input = Output::new(peripheral.PIN_26, Level::High);
    let mut adc = Adc::new(peripheral.ADC, Irqs, Config::default());
    let mut p27 = Channel::new_pin(peripheral.PIN_27, Pull::None);

    loop {
        Timer::after_secs(1).await;
        let value = adc.read(&mut p27).await.unwrap();
        info!("{}", value);
    }
}