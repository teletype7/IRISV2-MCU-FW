pub mod config;

use crate::mmc5983::config::*;
use embassy_stm32::*;

pub struct Mmc5983 {
    spi: spi::Spi<'static, mode::Async, spi::mode::Master>,
    auto_sr: bool,
}

impl Mmc5983 {
    pub fn new(spi: spi::Spi<'static, mode::Async, spi::mode::Master>) -> Self {
        Self {
            spi,
            auto_sr: false,
        }
    }

    pub async fn get_report(&mut self) -> MagReport {
        let write_buf: [u8; 1] = [0b1000_0000];
        let mut read_buf: [u8; 8] = [0; 8];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        
        // zero range is 2^17 counts
        let middle: f32 = 131072.0;
        // range of +-8 gauss and 18 bit sensitivity
        let lsb_gauss: f32 = 16384.0;
        
        MagReport {
            x: ((u32::from_le_bytes([0, read_buf[1], read_buf[2], read_buf[7] & 0b1100_0000]) >> 6)        as f32 - middle) / lsb_gauss,
            y: ((u32::from_le_bytes([0, read_buf[3], read_buf[4], (read_buf[7] & 0b0011_0000) << 2]) >> 6) as f32 - middle) / lsb_gauss,
            z: ((u32::from_le_bytes([0, read_buf[5], read_buf[6], (read_buf[7] & 0b0000_1100) << 4]) >> 6) as f32 - middle) / lsb_gauss,
        }
    }

    pub async fn get_report_temperature(&mut self) -> MagReportTemperature {
        let write_buf: [u8; 1] = [0b1000_0000];
        let mut read_buf: [u8; 9] = [0; 9];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();

        // zero range is 2^17 counts
        let middle: f32 = 131072.0;
        // range of +-8 gauss and 18 bit sensitivity
        let lsb_gauss: f32 = 16384.0;
        
        MagReportTemperature {
            x: ((u32::from_le_bytes([0, read_buf[1], read_buf[2], read_buf[7] & 0b1100_0000]) >> 6)        as f32 - middle) / lsb_gauss,
            y: ((u32::from_le_bytes([0, read_buf[3], read_buf[4], (read_buf[7] & 0b0011_0000) << 2]) >> 6) as f32 - middle) / lsb_gauss,
            z: ((u32::from_le_bytes([0, read_buf[5], read_buf[6], (read_buf[7] & 0b0000_1100) << 4]) >> 6) as f32 - middle) / lsb_gauss,
            temp: (read_buf[8] as f32) * 0.8 - 75.0
        }
    }

    pub async fn get_report_16(&mut self) -> MagReport16 {
        let write_buf: [u8; 1] = [0b1000_0000];
        let mut read_buf: [u8; 7] = [0; 7];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        
        // zero range is 2^15 counts
        let middle: f32 = 32768.0;
        // range of +-8 gauss and 16 bit sensitivity
        let lsb_gauss: f32 = 4096.0;
        
        MagReport16 {
            x: (u16::from_le_bytes([read_buf[1], read_buf[2]]) as f32 - middle) / lsb_gauss,
            y: (u16::from_le_bytes([read_buf[3], read_buf[4]]) as f32 - middle) / lsb_gauss,
            z: (u16::from_le_bytes([read_buf[5], read_buf[6]]) as f32 - middle) / lsb_gauss,
        }
    }

    /// enables interrupts on measurement done
    pub async fn init(&mut self) {
        let write_buf: [u8; 2] = [0x09, 0b0000_0100];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_odr_sr(&mut self, odr: MagOdr, sr: MagAutoSetReset) {
        let write_buf: [u8; 2] = [0x09, 0b1000_1000 | (sr as u8) << 4 | odr as u8];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_bw(&mut self, bw: MagBw) {
        let write_buf: [u8; 2] = [0x0a, bw as u8];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn soft_reset(&mut self) {
        let write_buf: [u8; 2] = [0x0a, 0b1000_0000];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn auto_sr(&mut self, auto: bool) {
        self.auto_sr = auto;
        let write_buf: [u8; 2] = [0x09, (auto as u8) << 5];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set(&mut self) {
        let write_buf: [u8; 2] = [0x09, 0b0000_1000 | ((self.auto_sr as u8) << 5)];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn reset(&mut self) {
        let write_buf: [u8; 2] = [0x09, 0b0001_0000 | ((self.auto_sr as u8) << 5)];
        self.spi.write(&write_buf).await.unwrap();
    }
}

/// gauss
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct MagReport {
    x: f32,
    y: f32,
    z: f32
}

/// gauss and C
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct MagReportTemperature {
    x: f32,
    y: f32,
    z: f32,
    temp: f32
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct MagReport16 {
    x: f32,
    y: f32,
    z: f32
}