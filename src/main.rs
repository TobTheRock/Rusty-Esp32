#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, systimer::SystemTimer,
};
use log::info;

#[embassy_executor::task]
async fn run() {
    loop {
        info!("Hello world from embassy using esp-hal-async!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    // Initialize the logger (needs feature log from esp_println)
    esp_println::logger::init_logger(log::LevelFilter::Info);
    info!("Init!");
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // TimerGroup::new_async creates the driver in non blocking mode
    // needs feature: embassy-time-timg0
    // let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    // embassy::init(&clocks, timg0);
    // one could also use the system timer
    let systimer = SystemTimer::new_async(peripherals.SYSTIMER);
    embassy::init(&clocks, systimer);

    spawner.spawn(run()).ok();

    loop {
        info!("Bing!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
