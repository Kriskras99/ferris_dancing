mod parser;
mod types;
mod writer;

use std::num::TryFromIntError;

pub use parser::*;
pub use types::*;
pub use writer::*;

const fn pow_2_roundup(mut v: u32) -> u32 {
    v -= 1;

    v |= (v + 1) >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;

    v + 1
}

fn count_zeros(v: u32) -> u32 {
    let mut zeros = 0;
    for i in 0..32 {
        if v & (1 << i) != 0 {
            break;
        }
        zeros += 1;
    }
    zeros
}

const fn is_pow_2(v: u32) -> bool {
    (v != 0) && ((v & (v - 1)) == 0)
}

const fn round_size(mut size: u32, pad: u32) -> u32 {
    let mask = pad - 1;
    if size & mask > 0 {
        size &= !mask;
        size += pad;
    }
    size
}

/// Get the address in the (de)swizzled data
fn get_addr(
    mut x: u32,
    mut y: u32,
    xb: u32,
    yb: u32,
    rounded_width: u32,
    x_base: u32,
) -> Result<usize, TryFromIntError> {
    let mut x_cnt = x_base;
    let mut y_cnt = 1;
    let mut x_used = 0;
    let mut y_used = 0;
    let mut address = 0;
    while (x_used < x_base + 2) && (x_used + x_cnt < xb) {
        let x_mask = (1 << x_cnt) - 1;
        let y_mask = (1 << y_cnt) - 1;

        address |= (x & x_mask) << (x_used + y_used);
        address |= (y & y_mask) << (x_used + y_used + x_cnt);

        x >>= x_cnt;
        y >>= y_cnt;

        x_used += x_cnt;
        y_used += y_cnt;

        x_cnt = 1.min(xb - x_used);
        y_cnt = (yb - y_used).min(y_cnt << 1);
    }

    address |= (x + (y * (rounded_width >> x_used))) << (x_used + y_used);

    usize::try_from(address)
}
