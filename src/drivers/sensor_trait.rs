//! Common trait for all sensor implementations

use core::fmt::Debug;

/// Unified sensor data structure
#[derive(Debug, Clone, Copy)]
pub enum SensorData {
    /// Temperature in degrees Celsius
    Temperature(f32),
    /// Relative humidity in percentage (0-100)
    Humidity(f32),
    /// Atmospheric pressure in hectopascals (hPa)
    Pressure(f32),
    /// Light intensity in lux
    LightIntensity(f32),
    /// Generic floating-point value
    Value(f32),
}

/// Sensor errors
#[derive(Debug, Clone, Copy)]
pub enum SensorError {
    /// I2C communication error
    I2cError,
    /// SPI communication error
    SpiError,
    /// Sensor not responding
    NoResponse,
    /// Invalid sensor data
    InvalidData,
    /// Sensor not initialized
    NotInitialized,
}

/// Common sensor interface for all environmental sensors
pub trait Sensor {
    /// Initialize the sensor
    async fn init(&mut self) -> Result<(), SensorError>;
    
    /// Read sensor data
    async fn read(&mut self) -> Result<SensorData, SensorError>;
    
    /// Put sensor into low-power sleep mode
    async fn sleep(&mut self) -> Result<(), SensorError>;
    
    /// Wake up sensor from sleep mode
    async fn wake(&mut self) -> Result<(), SensorError>;
}

/// Sensor hub for managing multiple sensors
pub struct SensorHub {
    // Future: Add sensor instances here
    // pub bme280: Option<BME280>,
    // pub tsl2591: Option<TSL2591>,
}

impl SensorHub {
    /// Create a new sensor hub
    pub fn new() -> Self {
        Self {
            // Initialize sensors
        }
    }

    /// Read all active sensors
    pub async fn read_all(&mut self) -> Result<(), SensorError> {
        // Future: Iterate through all sensors and read
        Ok(())
    }
}
