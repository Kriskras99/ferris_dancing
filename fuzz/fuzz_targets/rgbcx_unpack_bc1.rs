#![no_main]

use libfuzzer_sys::fuzz_target;
use image::Rgba;

fuzz_target!(|data: [u8; 8]| {
    let mut pixels_sys = [0; 64];

    let sys = rgbcx::get_rgbcx();
    let ret_sys = sys.unpack_bc1_mut(&data, &mut pixels_sys, Some(true));
    let pixels_sys: Vec<_> = pixels_sys.chunks_exact(4).map(|b| Rgba([b[0], b[1], b[2], b[3]])).collect();

    let mut pixels_rs = [Rgba([0,0,0,0]); 16];
    let ret_rs = rgbcx_rs::unpack_bc1_mut(&data, &mut pixels_rs, true);

    assert_eq!(&pixels_rs, pixels_sys.as_slice());
    assert_eq!(ret_rs, ret_sys);

    let mut pixels_sys = [0; 64];

    let sys = rgbcx::get_rgbcx();
    let ret_sys = sys.unpack_bc1_mut(&data, &mut pixels_sys, Some(false));
    let pixels_sys: Vec<_> = pixels_sys.chunks_exact(4).map(|b| Rgba([b[0], b[1], b[2], b[3]])).collect();

    let mut pixels_rs = [Rgba([0,0,0,0]); 16];
    let ret_rs = rgbcx_rs::unpack_bc1_mut(&data, &mut pixels_rs, false);

    assert_eq!(&pixels_rs, pixels_sys.as_slice());
    assert_eq!(ret_rs, ret_sys);

});
