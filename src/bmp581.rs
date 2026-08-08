pub mod config;

use crate::bmp581::config::*;
use embassy_stm32::*;

pub struct Bmp581 {
    spi: spi::Spi<'static, mode::Async, spi::mode::Master>,
}

impl Bmp581 {
    pub fn new(spi: spi::Spi<'static, mode::Async, spi::mode::Master>) -> Self {
        Self {
            spi
        }
    }

    pub async fn init(&mut self) {
        // ensure the chip is in spi mode - dsheet recommends doing a read of CHIP_ID to ensure 16
        // sclk periods have passed before trying any real communication
        let write_buf: [u8; 1] = [0b1000_0000 | 0x01];
        let mut read_buf: [u8; 2] = [0; 2];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
    }

    pub async fn get_report(&mut self) -> BaroReport {
        let write_buf: [u8; 1] = [0b1000_0000 | 0x01d];
        let mut read_buf: [u8; 7] = [0; 7];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        BaroReport {
            press: (u32::from_le_bytes([0, read_buf[6], read_buf[5], read_buf[4]]) as f32) / (2 << 6) as f32,
            temp: (u32::from_le_bytes([0, read_buf[3], read_buf[2], read_buf[1]]) as f32) / (2 << 16) as f32,
        }
    }

    pub async fn set_dsp(&mut self, use_iir: bool, compensation: CompensationConfig) {
        // set iir enable/disable for both temp and press simultaneously
        let write_buf: [u8; 2] = [0x30, ((use_iir as u8) << 6) | ((use_iir as u8) << 4) | (compensation as u8)];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_iir(&mut self, temp: TempIirConfig, press: PressIirConfig) {
        let write_buf: [u8; 2] = [0x31, ((press as u8) << 3) | (temp as u8)];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_oversample(&mut self, press_osr: PressOsrConfig, temp_osr: TempOsrConfig) {
        // enable pressure readings as well
        let write_buf: [u8; 2] = [0x36, 0b0100_0000 | (((press_osr as u8) << 3) | (temp_osr as u8))];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_power_odr(&mut self, power: PowerConfig, odr: OdrConfig) {
        let write_buf: [u8; 2] = [0x37, ((odr as u8) << 2) | (power as u8)];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn check_odr_osr_valid(&mut self) -> (bool, PressOsrConfig, TempOsrConfig) {
        let write_buf: [u8; 1] = [0b1000_0000 | 0x38];
        let mut read_buf: [u8; 2] = [0; 2];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();

        // todo verify
        // soundness - bitwise and'd to only use the required bits, the enum is exhaustive for all 3 bit values
        // only 3 bits are used, so this is sound even with chip errors
        let effective_press_osr = unsafe { core::mem::transmute((read_buf[1] & 0b0011_1000) >> 3) };
        let effective_temp_osr = unsafe { core::mem::transmute(read_buf[1] & 0b0000_0111) };

        let odr_osr_valid = read_buf[1] & 0b0100_0000 == 1;

        (odr_osr_valid, effective_press_osr, effective_temp_osr)
    }
}

/// press in Pa, temp in C
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct BaroReport {
    press: f32,
    temp: f32
}