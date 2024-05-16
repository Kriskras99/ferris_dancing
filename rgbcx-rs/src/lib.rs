#![allow(clippy::inline_always)]

use image::Rgba;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    #[inline(always)]
    #[must_use]
    pub fn rgba(self) -> Rgba<u8> {
        self.into()
    }

    #[inline(always)]
    #[must_use]
    pub fn interpolate(&self, rhs: &Self) -> (Self, Self) {
        (
            Self {
                red: u8::try_from((u16::from(self.red) * 2 + u16::from(rhs.red)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                green: u8::try_from((u16::from(self.green) * 2 + u16::from(rhs.green)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                blue: u8::try_from((u16::from(self.blue) * 2 + u16::from(rhs.blue)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                alpha: u8::try_from((u16::from(self.alpha) * 2 + u16::from(rhs.alpha)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
            },
            Self {
                red: u8::try_from((u16::from(rhs.red) * 2 + u16::from(self.red)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                green: u8::try_from((u16::from(rhs.green) * 2 + u16::from(self.green)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                blue: u8::try_from((u16::from(rhs.blue) * 2 + u16::from(self.blue)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
                alpha: u8::try_from((u16::from(rhs.alpha) * 2 + u16::from(self.alpha)) / 3)
                    .unwrap_or_else(|_| unreachable!()),
            },
        )
    }

    #[inline(always)]
    #[must_use]
    pub fn midpoint(&self, rhs: &Self) -> Self {
        Self {
            red: u8::try_from((u16::from(self.red) + u16::from(rhs.red)) / 2)
                .unwrap_or_else(|_| unreachable!()),
            green: u8::try_from((u16::from(self.green) + u16::from(rhs.green)) / 2)
                .unwrap_or_else(|_| unreachable!()),
            blue: u8::try_from((u16::from(self.blue) + u16::from(rhs.blue)) / 2)
                .unwrap_or_else(|_| unreachable!()),
            alpha: u8::try_from((u16::from(self.alpha) + u16::from(rhs.alpha)) / 2)
                .unwrap_or_else(|_| unreachable!()),
        }
    }
}

impl From<Color> for Rgba<u8> {
    #[inline(always)]
    fn from(value: Color) -> Self {
        Self([value.red, value.green, value.blue, value.alpha])
    }
}

pub fn unpack_bc1_mut(texel: &[u8; 8], pixels: &mut [Rgba<u8>; 16], set_alpha: bool) -> bool {
    let low_color = u16::from_le_bytes([texel[0], texel[1]]);
    let high_color = u16::from_le_bytes([texel[2], texel[3]]);

    let mut c0 = unpack_color(low_color);
    let mut c1 = unpack_color(high_color);

    let mut used_punchthrough = false;
    let (mut c2, mut c3) = if low_color > high_color {
        Color::interpolate(&c0, &c1)
    } else {
        used_punchthrough = true;
        (Color::midpoint(&c0, &c1), Color::default())
    };

    if set_alpha {
        c0.alpha = 255;
        c1.alpha = 255;
        c2.alpha = 255;
        if high_color >= low_color {
            c3.alpha = 0;
        } else {
            c3.alpha = 255;
        }
    }

    let colors = [c0.rgba(), c1.rgba(), c2.rgba(), c3.rgba()];

    for row in 0..4 {
        for column in 0..4 {
            let bits = (texel[4 + row] >> (column * 2)) & 0b11;
            let index = (row * 4) + column;
            pixels[index] = colors[usize::from(bits)];
        }
    }

    used_punchthrough
}

#[derive(Debug)]
pub struct UnexpectedPunchthrough;

pub fn unpack_bc3_mut(
    texel: &[u8; 16],
    pixels: &mut [Rgba<u8>; 16],
) -> Result<(), UnexpectedPunchthrough> {
    let (alpha_texel, rgb_texel) = texel.as_slice().split_at(8);
    let alpha_texel: &[u8; 8] = alpha_texel.try_into().unwrap_or_else(|_| unreachable!());
    let rgb_texel: &[u8; 8] = rgb_texel.try_into().unwrap_or_else(|_| unreachable!());

    if unpack_bc1_mut(rgb_texel, pixels, false) {
        return Err(UnexpectedPunchthrough);
    }

    let low_alpha = alpha_texel[0];
    let high_alpha = alpha_texel[1];

    let alphas = if low_alpha > high_alpha {
        [
            low_alpha,
            high_alpha,
            u8::try_from((u16::from(low_alpha) * 6 + u16::from(high_alpha)) / 7)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 5 + u16::from(high_alpha) * 2) / 7)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 4 + u16::from(high_alpha) * 3) / 7)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 3 + u16::from(high_alpha) * 4) / 7)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 2 + u16::from(high_alpha) * 5) / 7)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) + u16::from(high_alpha) * 6) / 7)
                .unwrap_or_else(|_| unreachable!()),
        ]
    } else {
        [
            low_alpha,
            high_alpha,
            u8::try_from((u16::from(low_alpha) * 4 + u16::from(high_alpha)) / 5)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 3 + u16::from(high_alpha) * 2) / 5)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) * 2 + u16::from(high_alpha) * 3) / 5)
                .unwrap_or_else(|_| unreachable!()),
            u8::try_from((u16::from(low_alpha) + u16::from(high_alpha) * 4) / 5)
                .unwrap_or_else(|_| unreachable!()),
            0,
            255,
        ]
    };

    let sels = [
        u32::from_le_bytes([alpha_texel[2], alpha_texel[3], alpha_texel[4], 0]),
        u32::from_le_bytes([alpha_texel[5], alpha_texel[6], alpha_texel[7], 0]),
    ];

    for (row, sel) in sels.iter().enumerate() {
        for column in 0..8 {
            let bit_index = column;
            let bits = (sel >> (bit_index * 3)) & 0b111;
            let index = (row * 8) + column;
            pixels[index].0[3] = alphas[usize::try_from(bits).unwrap_or_else(|_| unreachable!())];
        }
    }

    Ok(())
}

#[allow(clippy::as_conversions)]
#[inline(always)]
#[must_use]
pub const fn unpack_color(color: u16) -> Color {
    let red5 = ((color >> 11) & 31) as u8;
    let green6 = ((color >> 5) & 63) as u8;
    let blue5 = (color & 31) as u8;
    let red = (red5 << 3) | (red5 >> 2);
    let green = (green6 << 2) | (green6 >> 4);
    let blue = (blue5 << 3) | (blue5 >> 2);

    Color {
        red,
        green,
        blue,
        alpha: 0,
    }
}

#[allow(clippy::missing_panics_doc)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_bc1_fuzz_1() {
        let data = [41, 10, 11, 10, 158, 158, 11, 0];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        unpack_bc1_mut(&data, &mut pixels, false);
        assert_eq!(
            &pixels,
            &[
                Rgba([8, 67, 79, 0]),
                Rgba([8, 66, 84, 0]),
                Rgba([8, 65, 90, 0]),
                Rgba([8, 67, 79, 0]),
                Rgba([8, 67, 79, 0]),
                Rgba([8, 66, 84, 0]),
                Rgba([8, 65, 90, 0]),
                Rgba([8, 67, 79, 0]),
                Rgba([8, 66, 84, 0]),
                Rgba([8, 67, 79, 0]),
                Rgba([8, 69, 74, 0]),
                Rgba([8, 69, 74, 0]),
                Rgba([8, 69, 74, 0]),
                Rgba([8, 69, 74, 0]),
                Rgba([8, 69, 74, 0]),
                Rgba([8, 69, 74, 0])
            ]
        );
    }

    #[test]
    fn test_unpack_bc1_fuzz_2() {
        let data = [10, 128, 5, 0, 0, 0, 0, 0];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        unpack_bc1_mut(&data, &mut pixels, false);
        assert_eq!(
            &pixels,
            &[
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0]),
                Rgba([132, 0, 82, 0])
            ]
        );
    }

    #[test]
    fn test_unpack_bc1_fuzz_3() {
        let data = [10, 156, 102, 99, 156, 156, 156, 0];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        unpack_bc1_mut(&data, &mut pixels, false);
        assert_eq!(
            &pixels,
            &[
                Rgba([156, 130, 82, 0]),
                Rgba([118, 116, 60, 0]),
                Rgba([99, 109, 49, 0]),
                Rgba([137, 123, 71, 0]),
                Rgba([156, 130, 82, 0]),
                Rgba([118, 116, 60, 0]),
                Rgba([99, 109, 49, 0]),
                Rgba([137, 123, 71, 0]),
                Rgba([156, 130, 82, 0]),
                Rgba([118, 116, 60, 0]),
                Rgba([99, 109, 49, 0]),
                Rgba([137, 123, 71, 0]),
                Rgba([156, 130, 82, 0]),
                Rgba([156, 130, 82, 0]),
                Rgba([156, 130, 82, 0]),
                Rgba([156, 130, 82, 0])
            ]
        );
    }

    #[test]
    fn test_unpack_bc1_fuzz_4() {
        let data = [10, 156, 156, 10, 156, 156, 188, 0];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        unpack_bc1_mut(&data, &mut pixels, true);
        assert_eq!(
            &pixels,
            &[
                Rgba([156, 130, 82, 255]),
                Rgba([57, 97, 181, 255]),
                Rgba([8, 81, 231, 255]),
                Rgba([106, 113, 131, 255]),
                Rgba([156, 130, 82, 255]),
                Rgba([57, 97, 181, 255]),
                Rgba([8, 81, 231, 255]),
                Rgba([106, 113, 131, 255]),
                Rgba([156, 130, 82, 255]),
                Rgba([57, 97, 181, 255]),
                Rgba([57, 97, 181, 255]),
                Rgba([106, 113, 131, 255]),
                Rgba([156, 130, 82, 255]),
                Rgba([156, 130, 82, 255]),
                Rgba([156, 130, 82, 255]),
                Rgba([156, 130, 82, 255])
            ]
        );
    }

    #[test]
    fn test_unpack_bc3_fuzz_1() {
        let data = [180, 0, 0, 255, 247, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        assert!(unpack_bc3_mut(&data, &mut pixels).is_err());
    }

    #[test]
    fn test_unpack_bc3_fuzz_2() {
        let data = [
            167, 167, 167, 167, 167, 167, 167, 167, 167, 167, 135, 167, 167, 167, 167, 10,
        ];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        assert!(unpack_bc3_mut(&data, &mut pixels).is_ok());
        assert_eq!(
            &pixels,
            &[
                Rgba([165, 244, 57, 255]),
                Rgba([165, 243, 57, 167]),
                Rgba([165, 245, 57, 0]),
                Rgba([165, 245, 57, 167]),
                Rgba([165, 244, 57, 167]),
                Rgba([165, 243, 57, 255]),
                Rgba([165, 245, 57, 167]),
                Rgba([165, 245, 57, 167]),
                Rgba([165, 244, 57, 255]),
                Rgba([165, 243, 57, 167]),
                Rgba([165, 245, 57, 0]),
                Rgba([165, 245, 57, 167]),
                Rgba([165, 245, 57, 167]),
                Rgba([165, 245, 57, 255]),
                Rgba([165, 247, 57, 167]),
                Rgba([165, 247, 57, 167])
            ]
        );
    }

    #[test]
    fn test_unpack_bc3_fuzz_3() {
        let data = [
            167, 79, 88, 240, 255, 255, 255, 88, 255, 0, 165, 0, 252, 167, 0, 0,
        ];
        let mut pixels = [Rgba([0, 0, 0, 0]); 16];
        assert!(unpack_bc3_mut(&data, &mut pixels).is_ok());
        assert_eq!(
            &pixels,
            &[
                Rgba([0, 28, 255, 167]),
                Rgba([0, 22, 112, 141]),
                Rgba([0, 22, 112, 79]),
                Rgba([0, 22, 112, 167]),
                Rgba([0, 22, 112, 91]),
                Rgba([0, 20, 41, 91]),
                Rgba([0, 25, 183, 91]),
                Rgba([0, 25, 183, 91]),
                Rgba([0, 28, 255, 91]),
                Rgba([0, 28, 255, 91]),
                Rgba([0, 28, 255, 91]),
                Rgba([0, 28, 255, 91]),
                Rgba([0, 28, 255, 91]),
                Rgba([0, 28, 255, 79]),
                Rgba([0, 28, 255, 104]),
                Rgba([0, 28, 255, 154])
            ]
        );
    }
}
