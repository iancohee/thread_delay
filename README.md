# thread-delay ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![thread-delay on crates.io](https://img.shields.io/crates/v/thread-delay)](https://crates.io/crates/thread-delay) [![thread-delay on docs.rs](https://docs.rs/thread-delay/badge.svg)](https://docs.rs/thread-delay)

This crate provides a blocking implementation of DelayNs from
[`embedded_hal`][__link0] that can be used on Linux systems using the
Linux standard library. It is intended to be a drop-in replacement
for application using libraries that were written for embedded
devices but have access to the standard library, such as a single
board computer (SBC) running Linux.

The Delay implementation is like the one from [`linux_embedded_hal`][__link1] but
provides all the `delay_*` functions defined by the trait.

The functions are:

1. `delay_ns` for nanosecond delays
1. `delay_us` for microsecond delays
1. `delay_ms` for milliseconds delays

### Example

This example is an application running on a Raspberry Pi Zero 2 W
that takes readings from a `bme280` atmospheric sensor.

```rust
use thread_delay::Delay;

use bme280::i2c::BME280;
use rppal::i2c::I2c;
use std::time::Instant;
 
enum LoopState {
    Waiting,
    Measureing,
}

fn main() -> ! {
    let i2c = I2c::with_bus(1).expect("failed to find i2c bus 1");
    assert!(i2c.bus() == 1_u8);

    let mut delay = Delay {};

    let mut bme280 = BME280::new_primary(i2c);

    bme280.init(&mut delay).expect("failed to initialize bme280 sensor");

    let mut state = LoopState::Waiting;

    let delay_millis: u128 = 1_000;

    let mut last_update = Instant::now();

    loop {
        let elapsed = last_update.elapsed();

        match state {
            LoopState::Waiting => {
                if elapsed.as_millis() >= delay_millis {
                    state = LoopState::Measureing
                }
            }

            LoopState::Measureing => {
                let measurements = bme280
                    .measure(&mut delay)
                    .expect("failed to read measurements from bme280");

                println!("Temp:     |{:-10.3} C |", measurements.temperature);
                println!("Humidity: |{:-10.3} % |", measurements.humidity);
                println!("Pressure: |{:-10.3} Pa|", measurements.pressure);
                println!("-----");

                last_update = Instant::now();
                state = LoopState::Waiting;
            }
        }
    }
}
```


 [__cargo_doc2readme_dependencies_info]: ggGkYW0CYXSEG9qP0jVqEegAG9_0_pNDgzx8G50Fu7kvw9l7G41RRbSvpQtwYXKEG-0EJxRCtRzAG50sBKin-op1G-JQL3JtD4fOG_QMsypOphyZYWSCgmxlbWJlZGRlZF9oYWxlMS4wLjCCcmxpbnV4X2VtYmVkZGVkX2hhbPY
 [__link0]: https://crates.io/crates/embedded_hal/1.0.0
 [__link1]: https://crates.io/crates/linux_embedded_hal
