//! Sensor drivers and abstractions
//!
//! This module provides async drivers for various environmental sensors:
//! - BME280: Temperature, humidity, and pressure
//! - TSL2591: Light intensity
//! - Additional sensors can be easily integrated

pub mod sensor_trait;

// Future sensor implementations
// pub mod bme280;
// pub mod tsl2591;

pub use sensor_trait::{Sensor, SensorData, SensorError};
