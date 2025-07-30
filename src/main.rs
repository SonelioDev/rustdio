#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    main,
    time::{Duration, Instant},
};

use esp_println as _;
use defmt::info;

// use rgb_led::{RGB8, WS2812RMT};

extern crate alloc;

#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    // info!("Pre led");
    // // Wrap the led in an Arc<Mutex<...>>
    // let led = Arc::new(Mutex::new(WS2812RMT::new(
    //     peripherals.pins.gpio8,
    //     peripherals.rmt.channel0,
    // )?));
    // {
    //     let mut led = led.lock().unwrap();
    //     led.set_pixel(RGB8::new(50, 0, 0))?;
    // }
    // info!("Post led");

    loop {
        info!("Hello world from Sonelio!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
