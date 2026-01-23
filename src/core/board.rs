//! Board-level initialization for RAK3112 (STM32L082CZ + SX1276)
//!
//! This module handles:
//! - Pin configuration and peripheral initialization
//! - Low-power clock setup (LSE/LSI for LPTIM)
//! - GPIO mapping for SX1276 LoRa module
//! - ADC configuration for battery monitoring

use embassy_stm32::{
    adc::Adc,
    bind_interrupts,
    gpio::{Level, Output, Speed, Input, Pull},
    i2c::{self, I2c},
    peripherals,
    spi::{self, Spi},
    Config as StmConfig,
};
use embassy_stm32::time::Hertz;

/// RAK3112 Board configuration and peripherals
pub struct Board {
    /// SPI bus for SX1276 LoRa radio
    pub lora_spi: Spi<'static, peripherals::SPI1, peripherals::DMA1_CH2, peripherals::DMA1_CH3>,
    
    /// SX1276 NSS (Chip Select)
    pub lora_nss: Output<'static>,
    
    /// SX1276 RESET pin
    pub lora_reset: Output<'static>,
    
    /// SX1276 DIO pins for interrupt handling
    pub lora_dio0: Input<'static>,
    pub lora_dio1: Input<'static>,
    
    /// I2C bus for environmental sensors (BME280, etc.)
    pub sensor_i2c: I2c<'static, peripherals::I2C1>,
    
    /// ADC for battery voltage monitoring
    pub adc: Adc<'static, peripherals::ADC1>,
    
    /// Battery voltage sense pin (ADC channel)
    pub battery_sense: peripherals::PA0,
    
    /// Solar panel voltage sense pin (ADC channel)
    pub solar_sense: peripherals::PA1,
}

impl Board {
    /// Initialize the RAK3112 board with optimal low-power settings
    pub fn take() -> Self {
        // Get STM32 peripherals
        let p = embassy_stm32::init(Self::config());

        // Configure SX1276 LoRa SPI (SPI1)
        // SPI1: SCK=PA5, MISO=PA6, MOSI=PA7
        let lora_spi = Spi::new(
            p.SPI1,
            p.PA5,  // SCK
            p.PA7,  // MOSI
            p.PA6,  // MISO
            p.DMA1_CH3,
            p.DMA1_CH2,
            Hertz(1_000_000), // 1 MHz SPI clock for stability
            spi::Config::default(),
        );

        // SX1276 control pins
        let lora_nss = Output::new(p.PA4, Level::High, Speed::VeryHigh);
        let lora_reset = Output::new(p.PB0, Level::High, Speed::Low);
        
        // SX1276 interrupt pins (DIO0 and DIO1)
        let lora_dio0 = Input::new(p.PB1, Pull::Down);
        let lora_dio1 = Input::new(p.PB10, Pull::Down);

        // Configure I2C for environmental sensors (I2C1)
        // I2C1: SCL=PB6, SDA=PB7
        let sensor_i2c = I2c::new(
            p.I2C1,
            p.PB6,  // SCL
            p.PB7,  // SDA
            i2c::Config::default(),
        );

        // Configure ADC for battery monitoring
        let adc = Adc::new(p.ADC1);

        Self {
            lora_spi,
            lora_nss,
            lora_reset,
            lora_dio0,
            lora_dio1,
            sensor_i2c,
            adc,
            battery_sense: p.PA0,
            solar_sense: p.PA1,
        }
    }

    /// Get STM32L0 configuration optimized for low power
    fn config() -> StmConfig {
        let mut config = StmConfig::default();
        
        // Use MSI (Multi-Speed Internal) oscillator at 2.097 MHz for ultra-low power
        // Embassy will handle the low-power timer configuration
        config.rcc.msi = Some(embassy_stm32::rcc::MSIRange::Range5); // 2.097 MHz
        config.rcc.hsi = false;  // Disable HSI to save power
        
        // Enable LSE (Low-Speed External) 32.768 kHz crystal for RTC and LPTIM
        // This is crucial for low-power wake-up timers
        config.rcc.lse = Some(embassy_stm32::rcc::LseCfg {
            frequency: Hertz(32_768),
            bypass: false,
            security: false,
        });
        
        config
    }
}
