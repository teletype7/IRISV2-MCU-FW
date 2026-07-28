#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompensationConfig {
    None    = 0b00,
    Temp    = 0b01,
    Press   = 0b10,
    Both    = 0b11,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TempIirConfig {
    Bypass      = 0b000,
    Coeff1      = 0b001,
    Coeff3      = 0b010,
    Coeff7      = 0b011,
    Coeff15     = 0b100,
    Coeff31     = 0b101,
    Coeff63     = 0b110,
    Coeff127    = 0b111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressIirConfig {
    Bypass      = 0b000,
    Coeff1      = 0b001,
    Coeff3      = 0b010,
    Coeff7      = 0b011,
    Coeff15     = 0b100,
    Coeff31     = 0b101,
    Coeff63     = 0b110,
    Coeff127    = 0b111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressOsrConfig {
    Osr1    = 0b000,
    Osr2    = 0b001,
    Osr4    = 0b010,
    Osr8    = 0b011,
    Osr16   = 0b100,
    Osr32   = 0b101,
    Osr64   = 0b110,
    Osr128  = 0b111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TempOsrConfig {
    Osr1    = 0b000,
    Osr2    = 0b001,
    Osr4    = 0b010,
    Osr8    = 0b011,
    Osr16   = 0b100,
    Osr32   = 0b101,
    Osr64   = 0b110,
    Osr128  = 0b111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OdrConfig {
    Odr240   = 0x00,
    Odr218_5 = 0x01,
    Odr199_1 = 0x02,
    Odr179_2 = 0x03,
    Odr160   = 0x04,
    Odr149_3 = 0x05,
    Odr140   = 0x06,
    Odr129_9 = 0x07,
    Odr120   = 0x08,
    Odr110_2 = 0x09,
    Odr100_3 = 0x0A,
    Odr89_6  = 0x0B,
    Odr80    = 0x0C,
    Odr70    = 0x0D,
    Odr60    = 0x0E,
    Odr50_1  = 0x0F,
    Odr45    = 0x10,
    Odr40    = 0x11,
    Odr35    = 0x12,
    Odr30    = 0x13,
    Odr25    = 0x14,
    Odr20    = 0x15,
    Odr15    = 0x16,
    Odr10    = 0x17,
    Odr5     = 0x18,
    Odr4     = 0x19,
    Odr3     = 0x1A,
    Odr2     = 0x1B,
    Odr1     = 0x1C,
    Odr0_5   = 0x1D,
    Odr0_25  = 0x1E,
    Odr0_125 = 0x1F,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerConfig {
    Standby = 0b00,
    Normal  = 0b01,
    Forced  = 0b10,
    Continuous = 0b11,
}