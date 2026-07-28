pub mod ubx;
pub mod config;

use embassy_embedded_hal::SetConfig;
use embassy_stm32::*;
use crate::neom9n::config::*;

// todo finish implementing this horrible driver
pub struct Neo {
    // write cannot error, read has errors to check for
    uart: usart::Uart<'static, mode::Async>
}

impl Neo {
    pub fn new(uart: usart::Uart<'static, mode::Async>) -> Self {
        Self {
            uart
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
}