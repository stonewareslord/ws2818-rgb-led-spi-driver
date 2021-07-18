//! Example that definitely works on Raspberry Pi.
//! Make sure you have "SPI" on your Pi enabled and that MOSI-Pin is connected
//! with DIN-Pin. You just need DIN pin, no clock. WS2818 uses one-wire-protocol.
//! See the specification for details

use std::ops::Add;
use std::time::{Duration, Instant};
use std::thread;
use ws2818_examples::{get_led_num_from_args, sleep_busy_waiting_ms};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
use rand::Rng;

// Example that shows a single moving pixel though the 8x8 led matrix.
fn main() {
    println!("make sure you have \"SPI\" on your Pi enabled and that MOSI-Pin is connected with DIN-Pin!");
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let num_leds = get_led_num_from_args();
    let mut rng = rand::thread_rng();

    // note we first aggregate all data and write then all at
    // once! otherwise timings would be impossible to reach

    let mut i = 0;
    loop {
        let mut data = vec![];
        for j in 0..num_leds {
            // fill num_leds-1 pixels with black; one with white
            if i == j {
                data.extend_from_slice(&encode_rgb(50, 50, 50));
            } else {
                data.extend_from_slice(&encode_rgb(0, 0, 0));
            }
        }
        adapter.write_encoded_rgb(&data).unwrap();

        i = (i + 1) % num_leds;
        let ms = 1000 / 10 + rng.gen_range(0, 10); // 100ms / 10Hz
        let before = Instant::now(); // For printing time this takes

        // Using thread::sleep() - DOES NOT WORK - lights turn solid white
        thread::sleep(Duration::from_millis(ms));

        // Original code - WORKS FINE
        /*
        let target_time = Instant::now().add(Duration::from_millis(ms));
        loop {
            if Instant::now() >= target_time {
                break;
            }
        }
        */

        let after = Instant::now();

        // With original code:
        //     100.0010 to 100.0025 (about 1 to 2.5 microseconds extra)
        // With thread::sleep
        //     100.09 to 100.1 (about 90-100 microseconds extra)
        println!("{:?}", after - before);
    }
}
