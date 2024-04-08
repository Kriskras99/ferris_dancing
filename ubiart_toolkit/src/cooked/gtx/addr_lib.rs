use dotstar_toolkit_utils::{bytes::read::ReadError, testing::test_le};

use super::types::Format;

const FORMAT_HW_INFO: &[u32] = &[
    0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x01, 0x08, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x01, 0x10, 0x07, 0x00, 0x00, 0x10, 0x03, 0x00, 0x01, 0x10, 0x03, 0x00, 0x01,
    0x10, 0x0B, 0x00, 0x01, 0x10, 0x01, 0x00, 0x01, 0x10, 0x03, 0x00, 0x01, 0x10, 0x03, 0x00, 0x01,
    0x10, 0x03, 0x00, 0x01, 0x20, 0x03, 0x00, 0x00, 0x20, 0x07, 0x00, 0x00, 0x20, 0x03, 0x00, 0x00,
    0x20, 0x03, 0x00, 0x01, 0x20, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x03, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x20, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x01, 0x20, 0x0B, 0x00, 0x01, 0x20, 0x0B, 0x00, 0x01, 0x20, 0x0B, 0x00, 0x01,
    0x40, 0x05, 0x00, 0x00, 0x40, 0x03, 0x00, 0x00, 0x40, 0x03, 0x00, 0x00, 0x40, 0x03, 0x00, 0x00,
    0x40, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x10, 0x01, 0x00, 0x00,
    0x10, 0x01, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x60, 0x01, 0x00, 0x00,
    0x60, 0x01, 0x00, 0x00, 0x40, 0x01, 0x00, 0x01, 0x80, 0x01, 0x00, 0x01, 0x80, 0x01, 0x00, 0x01,
    0x40, 0x01, 0x00, 0x01, 0x80, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const FORMAT_EX_INFO: &[u32] = &[
    0x00, 0x01, 0x01, 0x03, 0x08, 0x01, 0x01, 0x03, 0x08, 0x01, 0x01, 0x03, 0x08, 0x01, 0x01, 0x03,
    0x00, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03,
    0x10, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03, 0x10, 0x01, 0x01, 0x03,
    0x10, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03,
    0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03,
    0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03,
    0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03,
    0x40, 0x01, 0x01, 0x03, 0x40, 0x01, 0x01, 0x03, 0x40, 0x01, 0x01, 0x03, 0x40, 0x01, 0x01, 0x03,
    0x40, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03, 0x80, 0x01, 0x01, 0x03, 0x80, 0x01, 0x01, 0x03,
    0x00, 0x01, 0x01, 0x03, 0x01, 0x08, 0x01, 0x05, 0x01, 0x08, 0x01, 0x06, 0x10, 0x01, 0x01, 0x07,
    0x10, 0x01, 0x01, 0x08, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03, 0x20, 0x01, 0x01, 0x03,
    0x18, 0x03, 0x01, 0x04, 0x30, 0x03, 0x01, 0x04, 0x30, 0x03, 0x01, 0x04, 0x60, 0x03, 0x01, 0x04,
    0x60, 0x03, 0x01, 0x04, 0x40, 0x04, 0x04, 0x09, 0x80, 0x04, 0x04, 0x0A, 0x80, 0x04, 0x04, 0x0B,
    0x40, 0x04, 0x04, 0x0C, 0x40, 0x04, 0x04, 0x0D, 0x40, 0x04, 0x04, 0x0D, 0x40, 0x04, 0x04, 0x0D,
    0x00, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03,
    0x00, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03, 0x40, 0x01, 0x01, 0x03, 0x00, 0x01, 0x01, 0x03,
];

pub fn surface_get_bits_per_pixel(format: u32) -> Result<u32, ReadError> {
    Ok(FORMAT_HW_INFO[usize::try_from((format & 0x3F) * 4)?])
}

pub fn get_surface_info(
    surface_format: Format,
    surface_width: u32,
    surface_height: u32,
    surface_depth: u32,
    surface_dim: u32,
    surface_tile_mode: u32,
    level: u32,
) -> Result<SurfaceOut, ReadError> {
    let hw_format = u32::from(surface_format) & 0x3F;

    let mut surface_out = SurfaceOut {
        size: 96,
        ..Default::default()
    };

    if surface_tile_mode == 16 {
        let num_samples = 1u32;
        let block_size = if (0x31..=0x35).contains(&hw_format) {
            4u32
        } else {
            1
        };
        let width = !(block_size - 1) & (1.max(surface_width >> level) + block_size - 1);

        surface_out.bpp = FORMAT_HW_INFO[usize::try_from(hw_format * 4)?];
        surface_out.pitch = width / block_size;
        surface_out.pixel_bits = FORMAT_HW_INFO[usize::try_from(hw_format * 4)?];
        surface_out.base_align = 1;
        surface_out.pitch_align = 1;
        surface_out.height_align = 1;
        surface_out.depth_align = 1;

        match surface_dim {
            0 => {
                surface_out.height = 1;
                surface_out.depth = 1;
            }
            1 | 6 => {
                surface_out.height = 1.max(surface_height >> level);
                surface_out.depth = 1;
            }
            2 => {
                surface_out.height = 1.max(surface_height >> level);
                surface_out.depth = 1.max(surface_depth >> level);
            }
            3 => {
                surface_out.height = 1.max(surface_height >> level);
                surface_out.depth = 6.max(surface_depth);
            }
            4 => {
                surface_out.height = 1;
                surface_out.depth = surface_depth;
            }
            5 | 7 => {
                surface_out.height = 1.max(surface_height >> level);
                surface_out.depth = surface_depth;
            }
            _ => {}
        }
    } else {
        let mut surface_in = SurfaceIn {
            size: 60,
            tile_mode: surface_tile_mode & 0xF,
            format: hw_format,
            bpp: FORMAT_HW_INFO[usize::try_from(hw_format * 4)?],
            num_samples: 1,
            num_frags: 1,
            width: 1.max(surface_width >> level),
            slice: 1,
            mip_level: level,
            ..Default::default()
        };
        match surface_dim {
            0 => {
                surface_in.height = 1;
                surface_in.num_slices = 1;
            }
            1 | 6 => {
                surface_in.height = 1.max(surface_height >> level);
                surface_in.num_slices = 1;
            }
            2 => {
                surface_in.height = 1.max(surface_height >> level);
                surface_in.num_slices = 1.max(surface_depth >> level);
                surface_in.flags |= 0x20;
            }
            3 => {
                surface_in.height = 1.max(surface_height >> level);
                surface_in.num_slices = 6.max(surface_depth);
                surface_in.flags |= 0x10;
            }
            4 => {
                surface_in.height = 1;
                surface_in.num_slices = surface_depth;
            }
            5 | 7 => {
                surface_in.height = 1.max(surface_height >> level);
                surface_in.num_slices = surface_depth;
            }
            _ => {}
        }

        if level == 0 {
            surface_in.flags = (1 << 12) | surface_in.flags & 0xFFFF_EFFF;
        } else {
            surface_in.flags &= 0xFFFF_EFFF;
        }

        compute_surface_info(surface_in, &mut surface_out)?;
    }

    todo!()
}

fn compute_surface_info(
    mut surface_in: SurfaceIn,
    surface_out: &mut SurfaceOut,
) -> Result<(), ReadError> {
    test_le(&surface_in.bpp, &0x80)?;
    compute_mip_level(&mut surface_in);

    surface_out.pixel_bits = surface_in.bpp;

    let mut bpp = surface_in.bpp;
    let mut expand_x = 1;
    let mut expand_y = 1;
    let mut elem_mode = 0;

    if surface_in.format != 0 {
        (bpp, expand_x, expand_y, elem_mode) = get_bits_per_pixel(surface_in.format);

        if elem_mode == 4 && expand_x == 3 && surface_in.tile_mode == 1 {
            surface_in.flags |= 0x200;
        }

        bpp = adjust_surface_info(&mut surface_in, elem_mode, expand_x, expand_y, bpp);
    } else if surface_in.bpp != 0 {
        surface_in.width = 1.max(surface_in.width);
        surface_in.height = 1.max(surface_in.height);
    } else {
        return Err(ReadError::custom("Texture is corrupt".to_string()));
    }

    todo!()
}

fn compute_mip_level(surface_in: &mut SurfaceIn) {
    if (49 <= surface_in.format || surface_in.format <= 55)
        && (surface_in.mip_level != 0 || ((surface_in.flags >> 12) & 1) == 1)
    {
        surface_in.width = pow_2_align(surface_in.width, 4);
        surface_in.height = pow_2_align(surface_in.height, 4);
    }
    let hwl_handled = hwl_compute_mip_level(surface_in);
    if !hwl_handled && surface_in.mip_level != 0 && ((surface_in.flags >> 12) & 1) == 1 {
        let mut width = 1.max(surface_in.width >> surface_in.mip_level);
        let mut height = 1.max(surface_in.height >> surface_in.mip_level);
        let mut slices = 1.max(surface_in.num_slices);

        if ((surface_in.flags >> 4) & 1) == 0 {
            slices = 1.max(slices >> surface_in.mip_level);
        }

        if surface_in.format != 47 && surface_in.format != 48 {
            width = next_pow_2(width);
            height = next_pow_2(height);
            slices = next_pow_2(slices);
        }
        surface_in.width = width;
        surface_in.height = height;
        surface_in.num_slices = slices;
    }
}

fn hwl_compute_mip_level(surface_in: &mut SurfaceIn) -> bool {
    if 49 <= surface_in.format || surface_in.format <= 55 {
        if surface_in.mip_level != 0 {
            let mut width = surface_in.width;
            let mut height = surface_in.height;
            let mut slices = surface_in.num_slices;

            if ((surface_in.flags >> 12) & 1) == 1 {
                width = 1.max(width >> surface_in.mip_level);
                height = 1.max(height >> surface_in.mip_level);

                if ((surface_in.flags >> 4) & 1) == 0 {
                    slices = 1.max(slices >> surface_in.mip_level);
                }
            }

            surface_in.width = next_pow_2(width);
            surface_in.height = next_pow_2(height);
            surface_in.num_slices = slices;
        }

        true
    } else {
        false
    }
}

fn get_bits_per_pixel(format: u32) -> (u32, u32, u32, u32) {
    let fmt_idx = usize::try_from(format * 4).unwrap_or_else(|_| unreachable!());
    (
        FORMAT_EX_INFO[fmt_idx],
        FORMAT_EX_INFO[fmt_idx + 1],
        FORMAT_EX_INFO[fmt_idx + 2],
        FORMAT_EX_INFO[fmt_idx + 3],
    )
}

const fn pow_2_align(x: u32, align: u32) -> u32 {
    !(align - 1) & (x + align - 1)
}

const fn next_pow_2(dim: u32) -> u32 {
    let mut new_dim = 1;
    if dim <= 0x7FFF_FFFF {
        while new_dim < dim {
            new_dim *= 2;
        }
    } else {
        new_dim = 0x8000_0000;
    }

    new_dim
}

fn adjust_surface_info(
    surface_in: &mut SurfaceIn,
    elem_mode: u32,
    expand_x: u32,
    expand_y: u32,
    bpp: u32,
) -> u32 {
    let width = surface_in.width;
    let height = surface_in.height;
    let bcn_format = bpp != 0 && [9, 10, 11, 12, 13].contains(&elem_mode);

    if width != 0 && height != 0 && (expand_x > 1 || expand_y > 1) {
        let (width, height) = if elem_mode == 4 {
            (expand_x * width, expand_y * height)
        } else if bcn_format {
            (width / expand_x, height / expand_y)
        } else {
            (
                (width + expand_x - 1) / expand_x,
                (height + expand_y - 1) / expand_y,
            )
        };

        surface_in.width = 1.max(width);
        surface_in.height = 1.max(height);
    }

    if bpp != 0 {
        match elem_mode {
            4 => surface_in.bpp = bpp / expand_x / expand_y,
            5 | 6 => surface_in.bpp = expand_x * expand_y * bpp,
            9 | 12 => surface_in.bpp = 64,
            10 | 11 | 13 => surface_in.bpp = 128,
            _ => surface_in.bpp = bpp,
        }
        return surface_in.bpp;
    }

    0
}

// fn compute_surface_info_ex(surface_in: &mut SurfaceIn) {
//     let num_samples = 1.max(surface_in.num_samples);
//     let mut pad_dims = 0;

//     if ((surface_in.flags >> 4) & 1) == 1 && surface_in.mip_level == 0 {
//         pad_dims = 2;
//     }

//     let tile_mode = if ((surface_in.flags >> 6) & 1) == 1 {
//         convert_to_non_bank_swapped_mode(surface_in.tile_mode)
//     } else {
//         compute_surface_mip_level_tile_mode(surface_in.tile_mode, surface_in.bpp, surface_in.mip_level, surface_in.width, surface_in.height, surface_in.num_slices, num_samples, (surface_in.flags >> 1 & 1) == 1, false)
//     };

//     if tile_mode == 0 || tile_mode == 1 {
//         compute_surface_info_linear()
//     }
// }

// fn convert_to_non_bank_swapped_mode(tile_mode: u32) -> u32 {
//     match tile_mode {
//         8 => 4,
//         9 => 5,
//         10 => 6,
//         11 => 7,
//         14 => 12,
//         15 => 13,
//         _ => tile_mode,
//     }
// }

// fn compute_surface_mip_level_tile_mode(base_tile_mode: u32, mut bpp: u32, level: u32, width: u32, height: u32, num_slices: u32, num_samples: u32, is_depth: bool, no_recursive: bool) -> u32 {
//     let mut width_align_factor = 1;
//     let mut macro_tile_width = 32;
//     let mut macro_tile_height = 16;
//     let tile_slices = compute_surface_tile_slices(base_tile_mode, bpp, num_samples);
//     let mut exp_tile_mode = base_tile_mode;

//     if num_samples > 1 || tile_slices > 1 || is_depth {
//         match base_tile_mode {
//             7 => exp_tile_mode = 4,
//             13 => exp_tile_mode = 12,
//             11 => exp_tile_mode = 8,
//             15 => exp_tile_mode = 14,
//             _ => {}
//         }
//     }

//     if base_tile_mode == 2 && num_samples > 1 {
//         exp_tile_mode = 4;
//     } else if base_tile_mode == 3{
//         if num_samples > 1 || is_depth {
//             exp_tile_mode = 2;
//         }

//         if num_samples == 2 || num_samples == 4 {
//             exp_tile_mode = 7;
//         }
//     }

//     if no_recursive || level == 0 {
//         return exp_tile_mode;
//     }

//     if [24, 48, 96].contains(&bpp) {
//         bpp = bpp / 3;
//     }

//     let widtha = next_pow_2(width);
//     let heighta = next_pow_2(height);
//     let num_slicesa = next_pow_2(num_slices);

//     exp_tile_mode = convert_to_non_bank_swapped_mode(exp_tile_mode);
//     let thickness = compute_surface_thickness(exp_tile_mode);
//     let micro_tile_bytes = (num_samples * bpp * (thickness << 6) + 7) >> 3;

//     if micro_tile_bytes < 256 {
//         width_align_factor = 1.max(256 / micro_tile_bytes);
//     }

//     if exp_tile_mode == 4 || exp_tile_mode == 12 {
//         if (widtha < width_align_factor * macro_tile_width) || heighta < macro_tile_height {
//             exp_tile_mode = 2;
//         }

//     } else if exp_tile_mode == 5 {
//         macro_tile_width = 16;
//         macro_tile_height = 32;

//         if (widtha < width_align_factor * macro_tile_width) || heighta < macro_tile_height {
//             exp_tile_mode = 2;
//         }

//     } else if exp_tile_mode == 6 {
//         macro_tile_width = 8;
//         macro_tile_height = 64;

//         if (widtha < width_align_factor * macro_tile_width) || heighta < macro_tile_height {
//             exp_tile_mode = 2;
//         }
//     } else if exp_tile_mode == 7 || exp_tile_mode == 13{
//         if (widtha < width_align_factor * macro_tile_width) || heighta < macro_tile_height {
//             exp_tile_mode = 3;
//         }
//     }
//     if num_slicesa < 4 {
//         if exp_tile_mode == 3 {
//             exp_tile_mode = 2
//         } else if exp_tile_mode == 7 {
//             exp_tile_mode = 4
//         } else if exp_tile_mode == 13 {
//             exp_tile_mode = 12
//         }
//     }

//     return compute_surface_mip_level_tile_mode(
//         exp_tile_mode,
//         bpp,
//         level,
//         widtha,
//         heighta,
//         num_slicesa,
//         num_samples,
//         is_depth,
//         true)
// }

// fn compute_surface_tile_slices(tile_mode: u32, bpp: u32, mut num_samples: u32) -> u32 {
//     let byte_per_sample = ((bpp << 6) + 7) >> 3;
//     let mut tileSlices = 1;

//     if compute_surface_thickness(tile_mode) > 1 {
//         num_samples = 4
//     }

//     if byte_per_sample != 0 {
//         let sample_per_tile = 2048 / byte_per_sample;
//         if sample_per_tile != 0 {
//             tileSlices = 1.max(num_samples / sample_per_tile)
//         }
//     }

//     tileSlices
// }

// fn compute_surface_thickness(tile_mode: u32) -> u32 {
//     match tile_mode {
//         3 | 7 | 11 | 13 | 15 => 4,
//         16 | 17 => 8,
//         _ => 1,
//     }
// }

// fn compute_surface_info_linear(tileMode: u32, bpp: u32, numSamples: u32, pitch: u32, height: u32, numSlices: u32, mipLevel: u32, padDims: u32, flags: u32) {
//     global expPitch
//     global expHeight
//     global expNumSlices

//     expPitch = pitch
//     expHeight = height
//     expNumSlices = numSlices

//     microTileThickness = computeSurfaceThickness(tileMode)

//     baseAlign, pitchAlign, heightAlign = computeSurfaceAlignmentsLinear(tileMode, bpp, flags)

//     if (flags.value >> 9) & 1 and not mipLevel:
//         expPitch //= 3
//         expPitch = nextPow2(expPitch)

//     if mipLevel:
//         expPitch = nextPow2(expPitch)
//         expHeight = nextPow2(expHeight)

//         if (flags.value >> 4) & 1:
//             expNumSlices = numSlices

//             if numSlices <= 1:
//                 padDims = 2

//             else:
//                 padDims = 0

//         else:
//             expNumSlices = nextPow2(numSlices)

//     expPitch, expHeight, expNumSlices = padDimensions(
//         tileMode,
//         padDims,
//         (flags.value >> 4) & 1,
//         pitchAlign,
//         heightAlign,
//         microTileThickness)

//     if (flags.value >> 9) & 1 and not mipLevel:
//         expPitch *= 3

//     slices = expNumSlices * numSamples // microTileThickness
//     pOut.pitch = expPitch
//     pOut.height = expHeight
//     pOut.depth = expNumSlices
//     pOut.surfSize = (expHeight * expPitch * slices * bpp * numSamples + 7) // 8
//     pOut.baseAlign = baseAlign
//     pOut.pitchAlign = pitchAlign
//     pOut.heightAlign = heightAlign
//     pOut.depthAlign = microTileThickness
// }

#[derive(Debug, Clone, Copy, Default)]
pub struct SurfaceOut {
    pub size: u32,
    pub pitch: u32,
    pub height: u32,
    pub depth: u32,
    pub surface_size: u32,
    pub tile_mode: u32,
    pub base_align: u32,
    pub pitch_align: u32,
    pub height_align: u32,
    pub depth_align: u32,
    pub bpp: u32,
    pub pixel_pitch: u32,
    pub pixel_height: u32,
    pub pixel_bits: u32,
    pub slice_size: u32,
    pub pitch_tile_max: u32,
    pub height_tile_max: u32,
    pub slice_tile_max: u32,
    pub p_tile_info: TileInfo,
    pub tile_type: u32,
    pub tile_index: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TileInfo {
    pub banks: u32,
    pub bank_width: u32,
    pub bank_height: u32,
    pub macro_aspect_ratio: u32,
    pub tile_split_bytes: u32,
    pub pipe_config: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SurfaceIn {
    pub size: u32,
    pub tile_mode: u32,
    pub format: u32,
    pub bpp: u32,
    pub num_samples: u32,
    pub width: u32,
    pub height: u32,
    pub num_slices: u32,
    pub slice: u32,
    pub mip_level: u32,
    pub flags: u32,
    pub num_frags: u32,
    pub tile_info: TileInfo,
    pub tile_index: u32,
}
