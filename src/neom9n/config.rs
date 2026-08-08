#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CfgStorage {
    Ram     = 0b001,
    Bbr     = 0b010,
    Flash   = 0b100,

    RamBbr = 0b011,
    RamFlash = 0b101,
    RamBbrFlash = 0b111,
    BbrFlash = 0b110,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicModel {
    Portable    = 0,
    Stationary  = 2,
    Pedestrian  = 3,
    Automotive  = 4,
    Sea         = 5,
    Air1g       = 6,
    Air2g       = 7,
    Air4g       = 8,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfoVerbosity {
    Error   = 0x01,
    Warning = 0x02,
    Notice  = 0x04,
    Test    = 0x08,
    Debug   = 0x10,
}