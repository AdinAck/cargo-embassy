#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    self,
    clock::CpuClock,
    gpio::{Level, Output},
    timer::timg::TimerGroup,
};
use esp_hal_embassy::main;
use esp_println::println;

#[main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let mut config = esp_hal::Config::default();
    config.cpu_clock = CpuClock::max();
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let mut led = Output::new(peripherals.GPIO17, Level::High);
    loop {
        println!("Hello, World!");
        led.toggle();
        Timer::after_millis(1_000).await;
    }
}
