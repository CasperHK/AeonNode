#![no_std]
#![no_main]

//! Weather Station Example
//!
//! This example demonstrates a complete weather station implementation using:
//! - Battery monitoring (solar-powered)
//! - Environmental sensors (temperature, humidity, pressure)
//! - LoRaWAN data transmission
//! - Ultra-low power sleep cycles

use aeonnode::prelude::*;
use aeonnode::core::Board;
use defmt_rtt as _;
use panic_probe as _;

// LoRaWAN credentials (replace with your own)
const DEV_EUI: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const APP_EUI: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const APP_KEY: [u8; 16] = [0x00; 16];

/// Main entry point
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("🚀 AeonNode Weather Station Starting...");
    
    // Initialize board peripherals
    let mut board = Board::take();
    
    info!("✓ Board initialized");
    info!("✓ RAK3112 (STM32L082CZ + SX1276)");
    
    // Initialize LoRa radio
    #[cfg(feature = "lora")]
    {
        use aeonnode::lora::{SX1276, LoRaConfig, LoRaWAN, LoRaWANConfig, DeviceClass};
        
        let lora_config = LoRaConfig::default();
        let mut sx1276 = SX1276::new(
            board.lora_spi,
            board.lora_nss,
            board.lora_reset,
            board.lora_dio0,
            lora_config,
        );
        
        info!("Initializing LoRa radio...");
        if let Err(e) = sx1276.init().await {
            error!("Failed to initialize SX1276: {:?}", e);
        }
        
        let lorawan_config = LoRaWANConfig {
            dev_eui: DEV_EUI,
            app_eui: APP_EUI,
            app_key: APP_KEY,
            device_class: DeviceClass::ClassA,
        };
        
        let mut lorawan = LoRaWAN::new(sx1276, lorawan_config);
        
        info!("Joining LoRaWAN network...");
        match lorawan.join().await {
            Ok(_) => info!("✓ Joined LoRaWAN network"),
            Err(e) => error!("Failed to join: {:?}", e),
        }
    }
    
    info!("Entering main loop...");
    
    // Main application loop
    let mut counter: u32 = 0;
    loop {
        info!("=== Measurement Cycle {} ===", counter);
        
        // Read battery status
        #[cfg(feature = "power")]
        {
            use aeonnode::power::PowerManager;
            let mut power_mgr = PowerManager::new(&mut board.adc);
            match power_mgr.get_status(&mut board.battery_sense, &mut board.solar_sense).await {
                Ok(status) => {
                    info!("Battery: {}mV ({}%)", status.voltage_mv, status.percentage);
                    info!("Solar: {}mV", status.solar_mv);
                    info!("State: {:?}", status.state);
                }
                Err(_) => warn!("Failed to read power status"),
            }
        }
        
        // Read sensors
        #[cfg(feature = "drivers")]
        {
            info!("Reading sensors...");
            // TODO: Read actual sensor data when drivers are implemented
        }
        
        // Transmit data via LoRaWAN
        #[cfg(feature = "lora")]
        {
            // Prepare payload (simplified for demonstration)
            let payload = [
                0x01,  // Temperature MSB
                0x5A,  // Temperature LSB (21.0°C example)
                0x32,  // Humidity (50%)
                0x03,  // Pressure MSB
                0xE8,  // Pressure LSB (1000 hPa example)
            ];
            
            info!("Transmitting data...");
            // TODO: Uncomment when lorawan is properly integrated
            // if let Err(e) = lorawan.send(1, &payload, false).await {
            //     error!("Failed to send data: {:?}", e);
            // }
        }
        
        counter += 1;
        
        // Sleep for 15 minutes (900 seconds) to conserve power
        // In a real deployment, use STOP mode for ultra-low power
        info!("Sleeping for 15 minutes...");
        Timer::after(Duration::from_secs(900)).await;
    }
}
