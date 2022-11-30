#![allow(dead_code)]

pub fn convert_to_unsigned_4(x: [i8; 4]) -> [u8; 4] {
    let mut out: [u8; 4] = [0u8; 4];
    for i in 0..4 {
        out[i] = x[i] as u8;
    }
    out
}

pub fn convert_to_unsigned_16(x: [i8; 16]) -> [u8; 16] {
    let mut out: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        out[i] = x[i] as u8;
    }
    out
}

pub fn get_netmask_from_prefix_length_v4(out: &mut [u8; 4], mut plen: i16) {
    for outb in out.iter_mut() {
        *outb = if plen >= 8 {
            plen -= 8;
            255u8
        } else if plen <= 0 {
            0u8
        } else {
            let v = 255u8 << (8 - plen);
            plen = 0;
            v
        }
    }
}

pub fn get_netmask_from_prefix_length_v6(out: &mut [u8; 16], mut plen: i16) {
    for outb in out.iter_mut() {
        *outb = if plen >= 8 {
            plen -= 8;
            255u8
        } else if plen == 0 {
            0u8
        } else {
            let v = 255u8 << (8 - plen);
            plen = 0;
            v
        }
    }
}
