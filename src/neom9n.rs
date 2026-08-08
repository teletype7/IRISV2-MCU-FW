pub mod ubx;
pub mod config;

use defmt::debug;
use embassy_embedded_hal::SetConfig;
use embassy_stm32::*;
use crate::neom9n::config::*;

// todo finish implementing this horrible driver
pub struct Neo {
    // write cannot error, read has errors to check for
    uart: usart::Uart<'static, mode::Async>
}

// todo go through default config and fix stuff that I don't like (pg 240 of interface description)
// todo wait for ack/nack
impl Neo {
    pub fn new(uart: usart::Uart<'static, mode::Async>) -> Self {
        Self {
            uart
        }
    }

    pub async fn get_nav_solution(&mut self) -> NavSolution {
        let mut read_buf: [u8; 96] = [0; 96];
        self.uart.read(&mut read_buf).await.unwrap();

        todo!("implement rest of gps driver");
        NavSolution {
            lat: 0.0,
            long: 0.0,
            height: 0.0,
            height_asl: 0.0,
            horiz_acc: 0.0,
            vert_acc: 0.0,
            vel_north: 0.0,
            vel_east: 0.0,
            vel_down: 0.0,
            vel_gnd: 0.0,
            vel_heading: 0.0,
            vel_acc: 0.0,
            heading_acc: 0.0,
            pos_dop: 0.0,
        }
    }

    pub async fn init(&mut self) {
        // uart1 baudrate config to 115200 baud
        let payload: [u8; 8] = [0x40, 0x52, 0x00, 0x01, 0x00, 0x01, 0xC2, 0x00];
        let mut write_buf = [0u8; 16];
        ubx::ubx_cfg_valset(CfgStorage::Ram, &payload, &mut write_buf);

        self.uart.write(&write_buf).await.unwrap();

        // reconfigure uart
        let mut gps_config = <usart::Uart<'_, mode::Async> as embassy_embedded_hal::SetConfig>::Config::default();
        gps_config.baudrate = 115200;
        gps_config.data_bits = usart::DataBits::DataBits8;
        gps_config.parity = usart::Parity::ParityNone;
        gps_config.stop_bits = usart::StopBits::STOP1;

        self.uart.set_config(&gps_config).unwrap();

        // enable ubx output, disable nmea output
        let payload: [u8; 10] = [
            0x10, 0x74, 0x00, 0x01, 0x01,
            0x10, 0x74, 0x00, 0x02, 0x00
        ];
        let mut write_buf = [0u8; 24];
        ubx::ubx_cfg_valset(CfgStorage::Ram, &payload, &mut write_buf);

        self.uart.write(&write_buf).await.unwrap();
    }

    pub async fn set_model(&mut self, model: DynamicModel) {
        let payload: [u8; 5] = [0x20, 0x11, 0x00, 0x21, model as u8];
        let mut write_buf = [0u8; 16];
        ubx::ubx_cfg_valset(CfgStorage::Ram, &payload, &mut write_buf);
        self.uart.write(&write_buf).await.unwrap();
    }

    /// use InfoVerbosity and bitwise or's to set this up
    pub async fn set_verbosity(&mut self, verbosity: u8) {
        debug_assert!(verbosity <= 0x1f && verbosity != 0, "verbosity out of range: {verbosity}");
        // only ubx on uart1
        let payload: [u8; 10] = [0, verbosity, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut write_buf = [0u8; 24];
        ubx::ubx_cfg_valset(CfgStorage::Ram, &payload, &mut write_buf);

        self.uart.write(&write_buf).await.unwrap();
    }

    pub async fn set_rate(&mut self, freq: f32) {
        debug_assert!(freq <= 25.0, "frequency out of range: {freq} hz");
        let period = (1000.0 / freq) as u16;
        debug_assert!(period <= 40, "period out of range: {period} ms");
        let period: [u8; 2] = period.to_le_bytes();

        // period, 1 measurement per solution, aligned to utc
        let payload: [u8; 5] = [period[1], period[0], 0, 1, 0];
        let mut write_buf = [0u8; 16];
        ubx::ubx_cfg_valset(CfgStorage::Ram, &payload, &mut write_buf);

        self.uart.write(&write_buf).await.unwrap();
    }

    pub async fn set_timepulse(&mut self) {
        // 5.9.26 of interface description, page 227
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct NavSolution {
    lat:        f64,
    long:       f64,
    height:     f64,
    height_asl: f64,
    horiz_acc:  f64,
    vert_acc:   f64,
    vel_north:  f64,
    vel_east:   f64,
    vel_down:   f64,
    vel_gnd:    f64,
    vel_heading:f64,
    vel_acc:    f64,
    heading_acc:f64,
    pos_dop:    f32
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, defmt::Format)]
pub struct NavSolutionTime {
    pos: NavSolution,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    tow: f64
}