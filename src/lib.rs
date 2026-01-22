#![no_std]
#![doc = include_str!("../README.md")]

//! # AeonNode Framework
//!
//! A low-power IoT development framework based on Embassy.rs for RAK3112 (STM32L0 + SX1276).
//!
//! ## Architecture
//!
//! The framework is organized into several modules:
//! - [`core`]: Board initialization and low-level hardware abstraction
//! - [`power`]: Power management and battery monitoring
//! - [`drivers`]: Sensor drivers (BME280, TSL2591, etc.)
//! - [`lora`]: LoRaWAN protocol stack wrapper
//!
//! ## Usage
//!
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//!
//! use aeonnode::core::Board;
//! use embassy_executor::Spawner;
//!
//! #[embassy_executor::main]
//! async fn main(spawner: Spawner) {
//!     let board = Board::take();
//!     // Your application logic here
//! }
//! ```

pub mod core;

#[cfg(feature = "power")]
pub mod power;

#[cfg(feature = "drivers")]
pub mod drivers;

#[cfg(feature = "lora")]
pub mod lora;

/// Re-export commonly used types from Embassy
pub mod prelude {
    pub use embassy_executor::Spawner;
    pub use embassy_time::{Duration, Timer, Instant};
    pub use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    pub use embassy_sync::channel::Channel;
    pub use embassy_sync::signal::Signal;
    pub use defmt::{info, warn, error, debug};
}

