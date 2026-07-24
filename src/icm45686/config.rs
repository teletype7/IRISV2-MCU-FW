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
    Range4000   = 0b0000,
    Range2000   = 0b0001,
    Range1000   = 0b0010,
    Range500    = 0b0011,
    Range250    = 0b0100,
    Range125    = 0b0101,
    Range62_5   = 0b0110,
    Range31_25  = 0b0111,
    Range15_625 = 0b1000,
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