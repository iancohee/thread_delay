//! This crate provides a blocking implementation of DelayNs from
//! [`embedded_hal`] that can be used on Linux systems using the
//! Linux standard library. It is intended to be a drop-in replacement
//! for application using libraries that were written for embedded
//! devices but have access to the standard library, such as a single 
//! board computer (SBC) running Linux.
//! 
//! The Delay implementation is like the one from [`linux_embedded_hal`] but
//! provides all the `delay_*` functions defined by the trait.
//! 
//! The functions are:
//! 
//! 1. `delay_ns` for nanosecond delays
//! 2. `delay_us` for microsecond delays
//! 3. `delay_ms` for milliseconds delays
//! 
//! ## Example
//!
//! This example is an application running on a Raspberry Pi Zero 2 W
//! that takes readings from a `bme280` atmospheric sensor.
//! 
//! ```rust,ignore
//! use thread_delay::Delay;
//!
//! use bme280::i2c::BME280;
//! use rppal::i2c::I2c;
//! use std::time::Instant;
//!  
//! enum LoopState {
//!     Waiting,
//!     Measureing,
//! }
//!
//! fn main() -> ! {
//!     let i2c = I2c::with_bus(1).expect("failed to find i2c bus 1");
//!     assert!(i2c.bus() == 1_u8);
//!
//!     let mut delay = Delay {};
//!
//!     let mut bme280 = BME280::new_primary(i2c);
//!
//!     bme280.init(&mut delay).expect("failed to initialize bme280 sensor");
//!
//!     let mut state = LoopState::Waiting;
//!
//!     let delay_millis: u128 = 1_000;
//!
//!     let mut last_update = Instant::now();
//!
//!     loop {
//!         let elapsed = last_update.elapsed();
//!
//!         match state {
//!             LoopState::Waiting => {
//!                 if elapsed.as_millis() >= delay_millis {
//!                     state = LoopState::Measureing
//!                 }
//!             }
//!
//!             LoopState::Measureing => {
//!                 let measurements = bme280
//!                     .measure(&mut delay)
//!                     .expect("failed to read measurements from bme280");
//!
//!                 println!("Temp:     |{:-10.3} C |", measurements.temperature);
//!                 println!("Humidity: |{:-10.3} % |", measurements.humidity);
//!                 println!("Pressure: |{:-10.3} Pa|", measurements.pressure);
//!                 println!("-----");
//!
//!                 last_update = Instant::now();
//!                 state = LoopState::Waiting;
//!             }
//!         }
//!     }
//! }
//! ```
use std::{thread::sleep, time::Duration};

use embedded_hal::delay::DelayNs;

#[allow(unused)]
/// Empty struct that implements the `DelayNs` trait using `std::thread::sleep`
pub struct ThreadDelay;

#[allow(unused)]
/// A convienvce alias for keeping the `Delay` struct name
pub type Delay = ThreadDelay;

/// `DelayNs` trait from [`embedded_hal`] implemented using sleep.
/// The thread may sleep longer than the duration specified due to
/// scheduling specifics or platform-dependent functionality.
/// It will never sleep less.
/// 
/// This function is blocking, and should not be used in async functions.
impl DelayNs for ThreadDelay {
    /// Pauses execution for at minimum `ns` nanoseconds.
    fn delay_ns(&mut self, ns: u32) {
        sleep(Duration::from_nanos(ns as u64));
    }

    /// Pauses execution for at minimum `us` microseconds.
    fn delay_us(&mut self, us: u32) {
        sleep(Duration::from_micros(us as u64));
    }

    /// Pauses execution for at minimum `ms` milliseconds.
    fn delay_ms(&mut self, ms: u32) {
        sleep(Duration::from_millis(ms as u64));
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_delay_ms() {
        let millis = [500_u32, 127, 0, 1];
        let mut delay = Delay {};

        for milli in millis {
            let start = Instant::now();
            delay.delay_ms(milli);
            let elapsed = start.elapsed();
            assert!(elapsed.as_millis() >= milli as u128);
        }
    }

    #[test]
    fn test_delay_ns() {
        let nanos = [500_u32, 127, 0, 454];
        let mut delay = Delay {};

        for nano in nanos {
            let start = Instant::now();
            delay.delay_ns(nano);
            assert!(start.elapsed().as_nanos() >= nano as u128);
        }
    }

    #[test]
    fn test_delay_us() {
        let micros = [250_u32, 128, 324, 98123];
        let mut delay = Delay {};

        for micro in micros {
            let start = Instant::now();
            delay.delay_us(micro);
            assert!(start.elapsed().as_micros() >= micro as u128);
        }
    }
}
