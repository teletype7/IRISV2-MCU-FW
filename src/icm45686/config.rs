#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GyroOdr {
    Odr6400     = 0b0011,
    Odr3200     = 0b0100,
    Odr1600     = 0b0101,
    Odr800      = 0b0110,
    Odr400      = 0b0111,
    Odr200      = 0b1000,
    Odr100      = 0b1001,
    Odr50       = 0b1010,
    Odr25       = 0b1011,
    Odr12_5     = 0b1100,
    Odr6_25     = 0b1101,
    Odr3_125    = 0b1110,
    Odr1_5625   = 0b1111,
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccelOdr {
    Odr6400     = 0b0011,
    Odr3200     = 0b0100,
    Odr1600     = 0b0101,
    Odr800      = 0b0110,
    Odr400      = 0b0111,
    Odr200      = 0b1000,
    Odr100      = 0b1001,
    Odr50       = 0b1010,
    Odr25       = 0b1011,
    Odr12_5     = 0b1100,
    Odr6_25     = 0b1101,
    Odr3_125    = 0b1110,
    Odr1_5625   = 0b1111,
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GyroRange {
    Range4000   = 0b0000, //
    Range2000   = 0b0001, // 16.384 lsb/dps
    Range1000   = 0b0010, // 32.768 lsb/dps
    Range500    = 0b0011,
    Range250    = 0b0100,
    Range125    = 0b0101,
    Range62_5   = 0b0110,
    Range31_25  = 0b0111,
    Range15_625 = 0b1000,
}
impl GyroRange {
    pub fn to_lsb_dps(&self) -> f32 {
        match self {
            GyroRange::Range4000 =>   { 8.192 }
            GyroRange::Range2000 =>   { 16.384 }
            GyroRange::Range1000 =>   { 32.768 }
            GyroRange::Range500 =>    { 65.536 }
            GyroRange::Range250 =>    { 131.072 }
            GyroRange::Range125 =>    { 262.144 }
            GyroRange::Range62_5 =>   { 524.288 }
            GyroRange::Range31_25 =>  { 1048.576 }
            GyroRange::Range15_625 => { 2097.152 }
        }
    }
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccelRange {
    Range32 = 0b000,
    Range16 = 0b001,
    Range8  = 0b010,
    Range4  = 0b011,
    Range2  = 0b100,
}
impl AccelRange {
    pub fn to_lsb_g(&self) -> f32 {
        match self {
            AccelRange::Range32 => { 1024.0 }
            AccelRange::Range16 => { 2048.0 }
            AccelRange::Range8 =>  { 4096.0 }
            AccelRange::Range4 =>  { 8192.0 }
            AccelRange::Range2 =>  { 16384.0 }
        }
    }
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GyroMode {
    Off         = 0b00,
    Standby     = 0b01,
    LowPower    = 0b10,
    LowNoise    = 0b11
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccelMode {
    Off         = 0b00,
    Standby     = 0b01,
    LowPower    = 0b10,
    LowNoise    = 0b11
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OscSource {
    Auto    = 0b0000,
    Mems    = 0b0010,
    Ext     = 0b1000
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpiSlew {
    Minimum     = 0b000, // 12ns min, 38ns typ, 106ns max
    VerySlow    = 0b001, // 4ns min, 14ns typ, 45ns max
    Slow        = 0b010, // 3ns min, 10ns typ, 37ns max
    Medium      = 0b011, // 2ns min, 7ns typ, 25ns max
    Fast        = 0b100, // 1ns min, 5ns typ, 17ns max
    VeryFast    = 0b101, // 1ns min, 4ns typ, 14ns max
    Maximum     = 0b110  // 0.1ns min, 0.5ns typ, 6ns max
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Slew {
    Minimum     = 0b000, // 12ns min, 38ns typ, 106ns max
    VerySlow    = 0b001, // 4ns min, 14ns typ, 45ns max
    Slow        = 0b010, // 3ns min, 10ns typ, 37ns max
    Medium      = 0b011, // 2ns min, 7ns typ, 25ns max
    Fast        = 0b100, // 1ns min, 5ns typ, 17ns max
    VeryFast    = 0b101, // 1ns min, 4ns typ, 14ns max
    Maximum     = 0b110  // 0.1ns min, 0.5ns typ, 6ns max
}