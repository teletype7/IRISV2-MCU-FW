#![no_main]
#![no_std]

pub mod config;

use embassy_stm32::*;
use crate::icm45686::config::*;

pub struct Icm45686 {
    spi: spi::Spi<'static, mode::Async, spi::mode::Master>,
    gyro_mode: GyroMode,
    accel_mode: AccelMode,
}
impl Icm45686 {
    pub fn new(spi: spi::Spi<'static, mode::Async, spi::mode::Master>) -> Self {
        Self {
            spi,
            gyro_mode: GyroMode::Off,
            accel_mode: AccelMode::Off,
        }
    }

    pub async fn init(&mut self) {
        // enable int1 interrupt on data ready, push pull pulse mode active high
        let write_buf: [u8; 4] = [0x16, 0b0000_0100, 0, 0b0000_0001];
        self.spi.write(&write_buf).await.unwrap();
    }
    pub async fn get_report(&mut self) -> SixAxisReport {
        let mut write_buf= [0b1000_0000];
        let mut read_buf = [0u8; 12];

        // never returns Err, so this is sound
        self.spi.transfer(&mut write_buf, &mut read_buf).await.unwrap();

        unsafe {
            // todo ensure soundness
            core::mem::transmute(read_buf)
        }
    }
    pub async fn set_power_mode(&mut self, gyro: GyroMode, accel: AccelMode) {
        self.gyro_mode = gyro;
        self.accel_mode = accel;
        let write_buf = [0x10, ((gyro as u8) << 2) | (accel as u8)];
        self.spi.write(&write_buf).await.unwrap();
    }
    pub async fn set_range_odr(&mut self, accel_range: AccelRange, accel_odr: AccelOdr, gyro_range: GyroRange, gyro_odr: GyroOdr) {
        // will get compiled out of release builds, but ensure we uphold the odr requirements of power mode
        if accel_odr as u8 >= AccelOdr::Odr6_25 as u8 {
            debug_assert!(self.accel_mode == AccelMode::LowPower)
        } else if accel_odr as u8 <= AccelOdr::Odr800 as u8 {
            debug_assert!(self.accel_mode == AccelMode::LowNoise)
        }
        if gyro_odr as u8 >= GyroOdr::Odr6_25 as u8 {
            debug_assert!(self.gyro_mode == GyroMode::LowPower)
        } else if gyro_odr as u8 <= GyroOdr::Odr800 as u8 {
            debug_assert!(self.gyro_mode == GyroMode::LowNoise)
        }

        let write_buf: [u8; 3] = [0x1b, ((accel_range as u8) << 4) | (accel_odr as u8), ((gyro_range as u8) << 4) | (gyro_odr as u8)];
        self.spi.write(&write_buf).await.unwrap();
    }
}

#[repr(C)]
pub struct SixAxisReport {
    x_accel: i16,
    y_accel: i16,
    z_accel: i16,
    x_gyro: i16,
    y_gyro: i16,
    z_gyro: i16,
}