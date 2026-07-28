// u1 = u8
// i1 = i8
// x1 = 8 bit bitfield (u8)
// u2 = u16
// i2 = i16
// x2 = 16 bit bitfield (u16)
// u4 = u32
// i4 = i32
// x4 = 32 bit bitfield (u32)
// r4 = f32
// r8 = f64
// ch = char
// u_n = n bit bitfield
// i_n = n bit signed int
// s_n = n bit signed int, msb is sign (NOT two's compliment)

use crate::neom9n::config::*;

pub fn checksum(data: &[u8]) -> (u8, u8) {
    let mut check_a: u8 = 0;
    let mut check_b: u8 = 0;
    for i in 0..data.len() {
        check_a = check_a.wrapping_add(data[i]);
        check_b = check_b.wrapping_add(check_a);
    }
    (check_a, check_b)
}

// I hate this in every way. the fact that I need to rely on the buffer being big enough just kills me.
pub fn ubx_cfg_valset(layers: CfgStorage, payload: &[u8], buf: &mut [u8]) {
    // ubx header
    buf[0] = 0xb5;
    buf[1] = 0x62;

    // cfg valset id
    buf[2] = 0x06;
    buf[3] = 0x8a;

    // payload length
    buf[4] = (4 + payload.len()) as u8;

    // version
    buf[5] = 0x00;

    // layers
    buf[6] = layers as u8;

    // reserved
    buf[7] = 0;
    buf[8] = 0;

    // payload (can't really use a type to represent it as payloads are unfortunately variable sized)
    for (i, byte) in payload.iter().enumerate() {
        buf[8 + i] = *byte;
    }
}