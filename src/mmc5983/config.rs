#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MagOdr {
    OdrOff  = 0b000,
    Odr1    = 0b001,
    Odr10   = 0b010,
    Odr20   = 0b011,
    Odr50   = 0b100,
    Odr100  = 0b101,
    Odr200  = 0b110,
    Odr1000 = 0b111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MagBw {
    Bw100 = 0b00,
    Bw200 = 0b01,
    Bw400 = 0b10,
    Bw800 = 0b11
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MagAutoSetReset {
    Sr1     = 0b000,
    Sr25    = 0b001,
    Sr75    = 0b010,
    Sr100   = 0b011,
    Sr250   = 0b100,
    Sr500   = 0b101,
    Sr1000  = 0b110,
    Sr2000  = 0b111,
}