pub mod config;

use embassy_stm32::*;
use crate::icm45686::config::*;

// very incomplete driver but good enough for me
// this was pure pain
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

    pub async fn init_int1(&mut self) {
        // enable int1 interrupt on data ready, push pull pulse mode active low
        let write_buf: [u8; 4] = [0x16, 0b0000_0100, 0, 0];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn get_report(&mut self) -> SixAxisReport {
        let mut write_buf= [0b1000_0000];
        let mut read_buf = [0u8; 13]; // extra byte bc address transfer

        self.spi.transfer(&mut write_buf, &mut read_buf).await.unwrap();

        // report starts on second byte, first is garbage from sending address
        SixAxisReport {
            x_accel:    i16::from_le_bytes([read_buf[1], read_buf[2]]),
            y_accel:    i16::from_le_bytes([read_buf[3], read_buf[4]]),
            z_accel:    i16::from_le_bytes([read_buf[5], read_buf[6]]),
            x_gyro:     i16::from_le_bytes([read_buf[7], read_buf[8]]),
            y_gyro:     i16::from_le_bytes([read_buf[9], read_buf[10]]),
            z_gyro:     i16::from_le_bytes([read_buf[11], read_buf[12]]),
        }
    }

    pub async fn get_report_and_temperature(&mut self) -> SixAxisTemperatureReport {
        let mut write_buf= [0b1000_0000];
        let mut read_buf = [0u8; 15]; // extra byte bc address transfer

        self.spi.transfer(&mut write_buf, &mut read_buf).await.unwrap();

        // report starts on second byte, first is garbage from sending address
        SixAxisTemperatureReport {
            x_accel:    i16::from_le_bytes([read_buf[1], read_buf[2]]),
            y_accel:    i16::from_le_bytes([read_buf[3], read_buf[4]]),
            z_accel:    i16::from_le_bytes([read_buf[5], read_buf[6]]),
            x_gyro:     i16::from_le_bytes([read_buf[7], read_buf[8]]),
            y_gyro:     i16::from_le_bytes([read_buf[9], read_buf[10]]),
            z_gyro:     i16::from_le_bytes([read_buf[11], read_buf[12]]),
            temp:      (i16::from_le_bytes([read_buf[13], read_buf[14]]) as f32) / 128.0 + 25.0,
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

    pub async fn set_osc_source(&mut self, source: OscSource) {
        let write_buf: [u8; 2] = [0x35, source as u8];
        self.spi.write(&write_buf).await.unwrap();
    }

    /// external oscillator should be exactly 32khz for normal operation
    pub async fn set_osc_external(&mut self) {
        // configure accel to be in the right mode for clkin
        self.ireg_write(0xa500 + 0x7b, 2).await;
        // configure gyro to be in the right mode for clkin
        self.ireg_write(0xa400 + 0xa6, 2).await;

        // enable realign
        let write_buf: [u8; 2] = [0x26, 0b0010_0000];
        self.spi.write(&write_buf).await.unwrap();

        // enable int2 pin
        let write_buf: [u8; 2] = [0x31, 0b0000_0100];
        self.spi.write(&write_buf).await.unwrap();
        // set int2 as clkin
        let write_buf: [u8; 2] = [0x31, 0b0000_0110];
        self.spi.write(&write_buf).await.unwrap();

        // request external oscillator as source
        self.set_osc_source(OscSource::Ext).await
    }

    pub async fn set_spi_slew(&mut self, slew: SpiSlew) {
        let write_buf: [u8; 2] = [0x32, (slew as u8) << 1];
        self.spi.write(&write_buf).await.unwrap();
    }

    pub async fn set_slew(&mut self, slew: Slew) {
        let write_buf: [u8; 2] = [0x34, slew as u8];
        self.spi.write(&write_buf).await.unwrap();
    }


    /// must be after set_osc_external has been called
    pub fn realign(&mut self) {
        let write_buf: [u8; 2] = [0x26, 0b0110_0000];
        // todo this unwrap is NOT completely sound since blocking write MAY error
        self.spi.blocking_write(&write_buf).unwrap();
    }

    /// reads whoami register and returns true if we get the expected output
    pub async fn whoami(&mut self) -> bool {
        let write_buf: [u8; 1] = [0b1000_0000 | 0x72];
        let mut read_buf: [u8; 2]  = [0; 2];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        read_buf[1] == 0xE9
    }

    async fn ireg_write(&mut self, addr: u16, data: u8) {
        // wait until not busy
        let write_buf: [u8; 1] = [0b1000_0000 | 0x7f];
        let mut read_buf: [u8; 2] = [0; 2];

        while read_buf[1] & 0b0000_0001 != 1 {
            self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        }

        // not busy, write data and address
        let write_buf: [u8; 4] = [0x7c, addr.to_le_bytes()[0], addr.to_le_bytes()[1], data];
        self.spi.write(&write_buf).await.unwrap();
    }

    async fn ireg_read(&mut self, addr: u16) -> u8 {
        // wait until not busy
        let write_buf: [u8; 1] = [0b1000_0000 | 0x7f];
        let mut read_buf: [u8; 2] = [0; 2];

        while read_buf[1] & 0b0000_0001 != 1 {
            self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        }

        let write_buf: [u8; 3] = [0x7c, addr.to_le_bytes()[0], addr.to_le_bytes()[1]];
        self.spi.write(&write_buf).await.unwrap();

        // wait until done reading
        let write_buf: [u8; 1] = [0b1000_0000 | 0x7f];
        let mut read_buf: [u8; 2] = [0; 2];

        while read_buf[1] & 0b0000_0001 != 1 {
            self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        }

        // done reading, actually read the data now
        let write_buf: [u8; 1] = [0b1000_0000 | 0x7e];
        let mut read_buf: [u8; 2] = [0; 2];
        self.spi.transfer(&mut read_buf, &write_buf).await.unwrap();
        read_buf[1]
    }
}

// todo do we want to convert this into g/dps or leave it raw? i think g/dps is more useful
pub struct SixAxisReport {
    x_accel: i16,
    y_accel: i16,
    z_accel: i16,
    x_gyro: i16,
    y_gyro: i16,
    z_gyro: i16,
}
pub struct SixAxisTemperatureReport {
    x_accel: i16,
    y_accel: i16,
    z_accel: i16,
    x_gyro: i16,
    y_gyro: i16,
    z_gyro: i16,
    temp: f32
}