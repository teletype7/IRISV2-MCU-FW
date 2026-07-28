#![no_main]
#![no_std]
#![feature(proc_macro_hygiene)]

// mostly here so I don't forget to await a Future
#![deny(unused_must_use)]

extern crate alloc;

use rtic_monotonics::stm32::prelude::*;
use lib as _; // global logger + panicking-behavior + memory layout

// configure our monotonic timer
stm32_tim5_monotonic!(Mono, 1_000_000);
defmt::timestamp!("{=usize}", {
    Mono::now().ticks() as usize
});

pub mod embassy_pac {
    pub use embassy_stm32::pac::Interrupt as interrupt;
    pub use embassy_stm32::pac::*;
}

#[rtic::app(
    device = embassy_pac,
    peripherals = false,
    // todo find more dispatchers to use
    dispatchers = [WWDG, IWDG, CRS]
)]
mod app {
    use lib::icm45686::config::Slew;
    use defmt::{info, warn, error, debug, Format};
    use embassy_stm32::*;
    use embassy_stm32::gpio::OutputType;
    use rtic_monotonics::rtic_time::embedded_hal_async::delay::DelayNs;
    use lib::icm45686::config::{AccelMode, AccelOdr, AccelRange, GyroMode, GyroOdr, GyroRange, SpiSlew};
    use lib::icm45686::Icm45686;
    use lib::mmc5983::config::{MagAutoSetReset, MagBw, MagOdr};
    use lib::mmc5983::Mmc5983;
    use lib::bmp581::config::{CompensationConfig, OdrConfig, PowerConfig, PressIirConfig, PressOsrConfig, TempIirConfig, TempOsrConfig};
    use lib::bmp581::Bmp581;
    use lib::neom9n::config::DynamicModel;
    use lib::neom9n::Neo;
    use super::*;

    bind_interrupts!(struct Irqs {
        GPDMA1_CHANNEL0 => dma::InterruptHandler<peripherals::GPDMA1_CH0>;
        GPDMA1_CHANNEL1 => dma::InterruptHandler<peripherals::GPDMA1_CH1>;

        GPDMA1_CHANNEL2 => dma::InterruptHandler<peripherals::GPDMA1_CH2>;
        GPDMA1_CHANNEL3 => dma::InterruptHandler<peripherals::GPDMA1_CH3>;

        GPDMA1_CHANNEL4 => dma::InterruptHandler<peripherals::GPDMA1_CH4>;
        GPDMA1_CHANNEL5 => dma::InterruptHandler<peripherals::GPDMA1_CH5>;

        GPDMA1_CHANNEL6 => dma::InterruptHandler<peripherals::GPDMA1_CH6>;
        GPDMA1_CHANNEL7 => dma::InterruptHandler<peripherals::GPDMA1_CH7>;

        GPDMA2_CHANNEL0 => dma::InterruptHandler<peripherals::GPDMA2_CH0>;
        GPDMA2_CHANNEL1 => dma::InterruptHandler<peripherals::GPDMA2_CH1>;

        GPDMA2_CHANNEL2 => dma::InterruptHandler<peripherals::GPDMA2_CH2>;
        GPDMA2_CHANNEL3 => dma::InterruptHandler<peripherals::GPDMA2_CH3>;

        GPDMA2_CHANNEL4 => dma::InterruptHandler<peripherals::GPDMA2_CH4>;
        GPDMA2_CHANNEL5 => dma::InterruptHandler<peripherals::GPDMA2_CH5>;

        GPDMA2_CHANNEL6 => dma::InterruptHandler<peripherals::GPDMA2_CH6>;
        GPDMA2_CHANNEL7 => dma::InterruptHandler<peripherals::GPDMA2_CH7>;

        FDCAN1_IT0 => can::IT0InterruptHandler<peripherals::FDCAN1>;
        FDCAN1_IT1 => can::IT1InterruptHandler<peripherals::FDCAN1>;

        USART3 => usart::InterruptHandler<peripherals::USART3>;
    });

    // Shared resources go here
    #[shared]
    struct Shared {

    }

    // Local resources go here
    #[local]
    struct Local {
        loop_count: usize
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        cx.core.SCB.enable_fpu();
        cx.core.SCB.enable_icache();
        cx.core.SCB.enable_dcache(&mut cx.core.CPUID);

        // configure stm32
        let mut cfg: Config = Default::default();
        cfg.rcc.hse = Some(rcc::Hse {
            freq: time::mhz(25),
            mode: rcc::HseMode::Oscillator,
        });
        cfg.rcc.ls = rcc::LsConfig {
            rtc: pac::rcc::vals::Rtcsel::DISABLE,
            lsi: false,
            lse: Some(rcc::LseConfig {
                frequency: time::Hertz(32768),
                mode: rcc::LseMode::Oscillator(rcc::LseDrive::MediumHigh),
            }),
            enable_backup_sram: false,
        };
        cfg.rcc.pll1 = Some(rcc::Pll {
            source: pac::rcc::vals::Pllsrc::HSE,
            prediv: pac::rcc::vals::Pllm::DIV2,
            mul: pac::rcc::vals::Plln::MUL40,
            divp: Some(pac::rcc::vals::Plldiv::DIV2),
            divq: Some(pac::rcc::vals::Plldiv::DIV25),
            divr: Some(pac::rcc::vals::Plldiv::DIV2),
        });
        cfg.rcc.pll2 = Some(rcc::Pll {
            source: pac::rcc::vals::Pllsrc::HSE,
            prediv: pac::rcc::vals::Pllm::DIV2,
            mul: pac::rcc::vals::Plln::MUL24,
            divp: Some(pac::rcc::vals::Plldiv::DIV25),
            divq: Some(pac::rcc::vals::Plldiv::DIV10),
            divr: Some(pac::rcc::vals::Plldiv::DIV2),
        });
        cfg.rcc.pll3 = Some(rcc::Pll {
            source: pac::rcc::vals::Pllsrc::HSE,
            prediv: pac::rcc::vals::Pllm::DIV5,
            mul: pac::rcc::vals::Plln::MUL48,
            divp: Some(pac::rcc::vals::Plldiv::DIV10),
            divq: Some(pac::rcc::vals::Plldiv::DIV10),
            divr: Some(pac::rcc::vals::Plldiv::DIV30),
        });

        cfg.rcc.sys = pac::rcc::vals::Sw::PLL1_P;
        cfg.rcc.apb1_pre = pac::rcc::vals::Ppre::DIV8;
        cfg.rcc.apb2_pre = pac::rcc::vals::Ppre::DIV8;
        cfg.rcc.apb3_pre = pac::rcc::vals::Ppre::DIV8;
        // apb1, apb2, apb3 are all 31.25mhz (61.5mhz timer)

        cfg.rcc.mux.usart1sel = pac::rcc::vals::Usart1sel::PLL2_Q;
        cfg.rcc.mux.usart2sel = pac::rcc::vals::Usartsel::PLL2_Q;
        cfg.rcc.mux.usart3sel = pac::rcc::vals::Usartsel::PLL2_Q;
        cfg.rcc.mux.uart4sel = pac::rcc::vals::Usartsel::PLL2_Q;

        cfg.rcc.mux.spi1sel = pac::rcc::vals::Spi1sel::PLL3_P;
        cfg.rcc.mux.spi2sel = pac::rcc::vals::Spi2sel::PLL3_P;
        cfg.rcc.mux.spi3sel = pac::rcc::vals::Spi3sel::PLL2_P;
        cfg.rcc.mux.spi4sel = pac::rcc::vals::Spi4sel::PLL2_Q;
        cfg.rcc.mux.spi6sel = pac::rcc::vals::Spi6sel::PLL3_Q;

        cfg.rcc.mux.fdcan12sel = pac::rcc::vals::Fdcansel::PLL1_Q;

        // etc
        let p: Peripherals = embassy_stm32::init(cfg);
        // 62.5 mhz, apb1 timer clock
        Mono::start(62_500_000);
        // 0.014552s rollover period, we'll have to see if that causes issues (it may)

        info!("init start");

        // cs sck mosi miso, interrupt
        // spi1 is icm2, using pa4 pa5 pa7 pa6, pc4 int
        // spi2 is icm1, using pb12 pb13 pb15 pb14, pd10 int
        // spi3 is bmp, using pa15 pc10 pc12 pc11, pd2 int
        // spi4 is mag, using pe4 pe2 pe6 pe5, pe3 int
        // spi6 is mcu->ppu, using none pb3 pb5 pb4
        // icm fsync is pe14
        let mut icm2_config: spi::Config = Default::default();
        icm2_config.frequency = time::mhz(24);
        icm2_config.nss_polarity = spi::SlaveSelectPolarity::ActiveLow;
        let mut icm1_config: spi::Config = Default::default();
        icm1_config.frequency = time::mhz(24);
        icm1_config.nss_polarity = spi::SlaveSelectPolarity::ActiveLow;
        let mut bmp_config: spi::Config = Default::default();
        bmp_config.frequency = time::mhz(12);
        let mut mag_config: spi::Config = Default::default();
        mag_config.frequency = time::mhz(10);
        let mut interconnect_config: spi::Config = Default::default();
        interconnect_config.frequency = time::mhz(24);
        interconnect_config.nss_output_disable = true;
        // todo go through and determine correct gpio pull and trigger edge
        let mut icm2: spi::Spi<mode::Async, spi::mode::Master> = spi::Spi::new(
            p.SPI1, p.PA5, p.PA7, p.PA6, p.GPDMA1_CH0, p.GPDMA1_CH1, Irqs, icm2_config
        );
        let mut icm2_int: exti::ExtiInput<mode::Blocking> = exti::ExtiInput::new_blocking(p.PC4, p.EXTI4, gpio::Pull::Up, exti::TriggerEdge::Falling);
        let mut icm1: spi::Spi<mode::Async, spi::mode::Master> = spi::Spi::new(
            p.SPI2, p.PB13, p.PB15, p.PB14, p.GPDMA1_CH2, p.GPDMA1_CH3, Irqs, icm1_config
        );
        let mut icm1_int: exti::ExtiInput<mode::Blocking> = exti::ExtiInput::new_blocking(p.PD10, p.EXTI10, gpio::Pull::Up, exti::TriggerEdge::Falling);
        let mut bmp: spi::Spi<mode::Async, spi::mode::Master> = spi::Spi::new(
            p.SPI3, p.PC10, p.PC12, p.PC11, p.GPDMA1_CH4, p.GPDMA1_CH5, Irqs, bmp_config
        );
        let mut bmp_int: exti::ExtiInput<mode::Blocking> = exti::ExtiInput::new_blocking(p.PD2, p.EXTI2, gpio::Pull::Up, exti::TriggerEdge::Falling);
        let mut mag: spi::Spi<mode::Async, spi::mode::Master> = spi::Spi::new(
            p.SPI4, p.PE2, p.PE6, p.PE5, p.GPDMA1_CH6, p.GPDMA1_CH7, Irqs, mag_config
        );
        let mut mag_int: exti::ExtiInput<mode::Blocking> = exti::ExtiInput::new_blocking(p.PE3, p.EXTI3, gpio::Pull::Up, exti::TriggerEdge::Falling);
        let mut interconnect: spi::Spi<mode::Async, spi::mode::Master> = spi::Spi::new(
            p.SPI6, p.PB3, p.PB5, p.PB4, p.GPDMA2_CH0, p.GPDMA2_CH1, Irqs, interconnect_config,
        );

        let mut fdcan = can::CanConfigurator::new(p.FDCAN1, p.PD0, p.PD1, Irqs);
        fdcan.set_bitrate(2_000_000);
        // todo for real firmware switch this to fdcan.into_normal_mode();
        let mut fdcan = fdcan.into_internal_loopback_mode();

        // gps is usart3, using PD9 PD8
        // WHY WAS THE COMPILER SO ANGRY ABOUT THIS
        let mut gps_config = <usart::Uart<'_, mode::Async> as embassy_embedded_hal::SetConfig>::Config::default();
        gps_config.baudrate = 38400;
        gps_config.data_bits = usart::DataBits::DataBits8;
        gps_config.parity = usart::Parity::ParityNone;
        gps_config.stop_bits = usart::StopBits::STOP1;
        let mut gps: usart::Uart<mode::Async> = usart::Uart::new(p.USART3, p.PD9, p.PD8, p.GPDMA2_CH2, p.GPDMA2_CH3, Irqs, gps_config).unwrap();

        // todo should this just use simplepwm for everything?
        // use simplepwm to set the pin up correctly but low level timer api to configure the clock
        let _icm_clk_pin = timer::simple_pwm::PwmPin::new(p.PE14, OutputType::PushPull);

        let icm_clk = timer::low_level::Timer::new(p.TIM1);
        icm_clk.set_frequency(time::khz(64), timer::low_level::RoundTo::Slower);
        icm_clk.set_compare_value(timer::Channel::Ch4, 32768); // nearly exactly 50% duty cycle
        icm_clk.set_output_compare_mode(timer::Channel::Ch4, timer::low_level::OutputCompareMode::Toggle);
        icm_clk.start();

        let pwm1_pin1 = timer::simple_pwm::PwmPin::new(p.PA0, OutputType::PushPull);
        let pwm1_pin2 = timer::simple_pwm::PwmPin::new(p.PA1, OutputType::PushPull);
        let pwm1_pin3 = timer::simple_pwm::PwmPin::new(p.PA2, OutputType::PushPull);
        let pwm1_pin4 = timer::simple_pwm::PwmPin::new(p.PA3, OutputType::PushPull);
        let mut pwm1 = timer::simple_pwm::SimplePwm::new(
            p.TIM2, Some(pwm1_pin1), Some(pwm1_pin2), Some(pwm1_pin3), Some(pwm1_pin4), time::hz(300), timer::low_level::CountingMode::EdgeAlignedUp
        );

        let pwm2_pin1 = timer::simple_pwm::PwmPin::new(p.PC6, OutputType::PushPull);
        let pwm2_pin2 = timer::simple_pwm::PwmPin::new(p.PC7, OutputType::PushPull);
        let pwm2_pin3 = timer::simple_pwm::PwmPin::new(p.PC8, OutputType::PushPull);
        let pwm2_pin4 = timer::simple_pwm::PwmPin::new(p.PC9, OutputType::PushPull);
        let mut pwm2 = timer::simple_pwm::SimplePwm::new(
            p.TIM3, Some(pwm2_pin1), Some(pwm2_pin2), Some(pwm2_pin3), Some(pwm2_pin4), time::hz(300), timer::low_level::CountingMode::EdgeAlignedUp
        );

        // needs to be reconfigured for new operations when needed - todo figure out what most useful is
        let mut cordic_config = cordic::Config::new(cordic::Function::Cos, Default::default(), Default::default());
        let mut cordic = cordic::Cordic::new(p.CORDIC, cordic_config.unwrap());

        let icm1 = Icm45686::new(icm1);
        let icm2 = Icm45686::new(icm2);
        let mag = Mmc5983::new(mag);
        let bmp = Bmp581::new(bmp);
        let gps = Neo::new(gps);

        // MUST NEVER SPAWN ANOTHER TASK FROM INIT
        postinit::spawn(icm1, icm2, mag, bmp, gps).ok();

        info!("init done");
        (
            Shared {

            },
            Local {
                loop_count: 0
            },
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        info!("idle");

        loop {
            cortex_m::asm::nop()
        }
    }

    #[task(priority = 15)]
    /**
     * inits all hardware stuff, MUST BE RUN IMMEDIATELY AFTER INIT WITH NO OTHER TASKS RUNNING
     * basically just acts as an async part of init
     */
    async fn postinit(_cx: postinit::Context, mut icm1: Icm45686, mut icm2: Icm45686, mut mag: Mmc5983, mut bmp: Bmp581, mut gps: Neo) {
        icm1.init_int1().await;
        icm2.init_int1().await;
        icm1.set_spi_slew(SpiSlew::Medium).await;
        icm2.set_spi_slew(SpiSlew::Medium).await;
        icm1.set_slew(Slew::Medium).await;
        icm2.set_slew(Slew::Medium).await;
        icm1.set_power_mode(GyroMode::LowNoise, AccelMode::LowNoise).await;
        icm2.set_power_mode(GyroMode::LowNoise, AccelMode::LowNoise).await;
        icm1.set_range_odr(AccelRange::Range8, AccelOdr::Odr6400, GyroRange::Range1000, GyroOdr::Odr6400).await;
        icm2.set_range_odr(AccelRange::Range32, AccelOdr::Odr6400, GyroRange::Range4000, GyroOdr::Odr6400).await;
        icm1.set_osc_external().await;
        icm2.set_osc_external().await;
        // todo ensure the cs doesn't cause issues with spi
        cortex_m::interrupt::free(|_| {
            // there still will be some desync, but this is the best we can do
            icm1.realign();
            icm2.realign();
        });
        // safe to panic here, we've only just started the firmware (not in flight loop yet)
        if !(icm1.whoami().await && icm2.whoami().await ) {
            panic!("both icms didn't respond correctly to whoami!")
        }

        mag.init().await;
        mag.set_bw(MagBw::Bw800).await;
        // set/reset every 2 seconds, may lower that later
        mag.set_odr_sr(MagOdr::Odr1000, MagAutoSetReset::Sr2000).await;
        mag.set().await;
        mag.reset().await;
        mag.auto_sr(true).await;

        bmp.init().await;
        // no low pass filter for now, will see if it helps later (prob only coeff 1 at most)
        bmp.set_iir(TempIirConfig::Bypass, PressIirConfig::Bypass).await;
        bmp.set_dsp(false, CompensationConfig::Both).await;
        // allows for 251hz odr, so should be compatible with desired odr
        // see page 19 of datasheet (table 9)
        bmp.set_oversample(PressOsrConfig::Osr4, TempOsrConfig::Osr1).await;
        // normal not continous, since we need *consistent* sample times, not just fast as possible
        bmp.set_power_odr(PowerConfig::Normal, OdrConfig::Odr240).await;
        // only runs in debug firmware, which is fine since we'll test with debug firmware before flight
        debug_assert!(bmp.check_odr_osr_valid().await.0, "invalid osr/odr config!");

        gps.init().await;
        gps.set_model(DynamicModel::Air4g).await;

        // spawm all tasks from here
        task1::spawn().ok();
        task2::spawn().ok();
    }

    // TODO: Add tasks
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        info!("Hello from task1!");
    }

    #[task(priority = 15, local = [loop_count])]
    async fn task2(cx: task2::Context) {
        loop {
            info!("task2: before delay");
            Mono.delay_ms(1000).await;
            info!("task2: after delay");
            *cx.local.loop_count += 1;
        }
    }
}
