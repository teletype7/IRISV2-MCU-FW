pub mod config;

use crate::bmp581::config::*;
use embassy_stm32::*;

// todo finish implementing
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
}

// todo should this be f32 or u32?
pub struct BaroReport {
    press: f32,
    temp: f32
}