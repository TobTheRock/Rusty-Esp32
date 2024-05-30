#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
use embassy_executor::Spawner;

use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Ipv4Address, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, rng::Rng,
    systimer::SystemTimer, timer::TimerGroup, Async, Blocking,
};
use esp_println::logger;
use esp_wifi::wifi::{
    ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiState,
};
use esp_wifi::{wifi::WifiStaDevice, wifi_interface::WifiStack, EspWifiInitFor};
use log::info;
use static_cell::make_static;

// https://github.com/esp-rs/esp-wifi/blob/main/esp-wifi/examples/embassy_dhcp.rs

mod web_server;
mod wifi;

#[main]
async fn main(spawner: Spawner) {
    // Initialize the logger (needs feature log from esp_info)
    logger::init_logger(log::LevelFilter::Info);
    info!("Init!");
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    // maximum clock speed
    let clocks = ClockControl::max(system.clock_control).freeze();

    // can we also use another timer? E.g. async?
    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    let init = esp_wifi::initialize(
        EspWifiInitFor::Wifi,
        systimer.alarm0,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let wifi = peripherals.WIFI;
    let (wifi_interface, controller) =
        esp_wifi::wifi::new_with_mode(&init, wifi, WifiStaDevice).unwrap();

    // TimerGroup::new_async creates the driver in non blocking mode
    // needs feature: embassy-time-timg0
    // one could also use the system timer
    // needs feature: "embassy-time-systick-16mhz",
    // let systimer = SystemTimer::new_async(peripherals.SYSTIMER);
    let timer_group0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0);

    // Init network stack
    let config = Config::dhcpv4(Default::default());
    let seed: u64 = 1234; // very random, very secure seed
    let max_sockets = 16;
    let stack = make_static!(Stack::new(
        wifi_interface,
        config,
        make_static!(StackResources::<16>::new()),
        seed
    ));

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(stack)).ok();

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    // TODO use async function
    // stack.
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
    }

    web_server::start_webserver(spawner, stack);

    // loop {
    //     Timer::after(Duration::from_millis(1_000)).await;

    //     let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);

    //     socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    //     let remote_endpoint = (Ipv4Address::new(142, 250, 185, 115), 80);
    //     info!("connecting...");
    //     let r = socket.connect(remote_endpoint).await;
    //     if let Err(e) = r {
    //         info!("connect error: {:?}", e);
    //         continue;
    //     }
    //     info!("connected!");
    //     let mut buf = [0; 1024];
    //     loop {
    //         use embedded_io_async::Write;
    //         let r = socket
    //             .write_all(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
    //             .await;
    //         if let Err(e) = r {
    //             info!("write error: {:?}", e);
    //             break;
    //         }
    //         let n = match socket.read(&mut buf).await {
    //             Ok(0) => {
    //                 info!("read EOF");
    //                 break;
    //             }
    //             Ok(n) => n,
    //             Err(e) => {
    //                 info!("read error: {:?}", e);
    //                 break;
    //             }
    //         };
    //         info!("{}", core::str::from_utf8(&buf[..n]).unwrap());
    //     }
    //     Timer::after(Duration::from_millis(3000)).await;
    // }
}

#[embassy_executor::task]
async fn connection(controller: WifiController<'static>) {
    info!("start connection task");
    info!("Device capabilities: {:?}", controller.get_capabilities());
    let mut wifi_connection = wifi::ClientWifiConnection::start_new(controller).await;
    loop {
        wifi_connection.connect().await;
        Timer::after(Duration::from_millis(5000)).await
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    stack.run().await
}
