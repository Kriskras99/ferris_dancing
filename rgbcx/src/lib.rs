use ::std::os::raw::c_void;
use std::sync::OnceLock;

use bitflags::bitflags;

pub static RGBCX: OnceLock<Rgbcx> = OnceLock::new();
pub fn get_rgbcx() -> &'static Rgbcx {
    RGBCX.get_or_init(Rgbcx::default)
}

pub const MIN_TOTAL_ORDERINGS: u32 = rgbcx_sys::rgbcx_MIN_TOTAL_ORDERINGS;
pub const MAX_TOTAL_ORDERINGS3: u32 = rgbcx_sys::rgbcx_MAX_TOTAL_ORDERINGS3;
pub const MAX_TOTAL_ORDERINGS4: u32 = rgbcx_sys::rgbcx_MAX_TOTAL_ORDERINGS4;
pub const DEFAULT_TOTAL_ORDERINGS_TO_TRY: u32 = rgbcx_sys::rgbcx_DEFAULT_TOTAL_ORDERINGS_TO_TRY;
pub const DEFAULT_TOTAL_ORDERINGS_TO_TRY3: u32 = rgbcx_sys::rgbcx_DEFAULT_TOTAL_ORDERINGS_TO_TRY3;
pub const MIN_LEVEL: u8 = rgbcx_sys::rgbcx_MIN_LEVEL as u8;
pub const MAX_LEVEL: u8 = rgbcx_sys::rgbcx_MAX_LEVEL as u8;

pub struct Rgbcx {
    _approx_mode: Bc1ApproxMode
}

impl Default for Rgbcx {
    fn default() -> Self {
        Self::new()
    }
}

impl Rgbcx {
    pub fn new() -> Self {
        unsafe { rgbcx_sys::rgbcx_init(rgbcx_sys::rgbcx_bc1_approx_mode_cBC1Ideal) };
        Self {
            _approx_mode: Bc1ApproxMode::Ideal
        }
    }

    pub fn with_bc1_approx_mode(bc1_approx_mode: Bc1ApproxMode) -> Self {
        unsafe { rgbcx_sys::rgbcx_init(bc1_approx_mode.into()) };
        Self {
            _approx_mode: bc1_approx_mode
        }
    }

    pub fn encode_bc1_solid_block(fr: u32, fg: u32, fb: u32, allow_3color: bool) -> [u8; 8] {
        let mut buf = [0; 8];
        let buf_cvoid: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        unsafe {
            rgbcx_sys::rgbcx_encode_bc1_solid_block(buf_cvoid, fr, fg, fb, allow_3color);
        }
        buf
    }

    pub fn encode_bc1(pixels: &[u8; 64], level: u8, allow_3color: bool, use_transparent_texels_for_black: bool) -> [u8; 8] {
        if level > MAX_LEVEL+1 {
            panic!("Level is too big!");
        }
        let level = u32::from(level);
        let mut buf = [0; 8];
        let buf_cvoid: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc1(level, buf_cvoid, pixels, allow_3color, use_transparent_texels_for_black);
        }
        buf
    }

    pub fn encode_bc1_with_flags(pixels: &[u8; 64], flags: Flags, total_orderings_to_try: u32, total_orderings_to_try3: u32) -> [u8; 8] {
        let flags = flags.bits();
        let mut buf = [0; 8];
        let buf_cvoid: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc11(buf_cvoid, pixels, flags, total_orderings_to_try, total_orderings_to_try3);
        }
        buf
    }

    pub fn encode_bc3(pixels: &[u8; 64], level: u8) -> [u8; 8] {
        if level > MAX_LEVEL+1 {
            panic!("Level is too big!");
        }
        let level = u32::from(level);
        let mut buf = [0; 8];
        let buf_cvoid: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc3(level, buf_cvoid, pixels);
        }
        buf
    }

    pub fn encode_bc3_with_flags(pixels: &[u8; 64], flags: Flags, total_orderings_to_try: u32) -> [u8; 8] {
        let flags = flags.bits();
        let mut buf = [0; 8];
        let buf_cvoid: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc31(buf_cvoid, pixels, flags, total_orderings_to_try);
        }
        buf
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bc1ApproxMode {
    Ideal,
    IdealRound4,
    Amd,
    Nvidia
}

impl From<Bc1ApproxMode> for rgbcx_sys::rgbcx_bc1_approx_mode {
    fn from(value: Bc1ApproxMode) -> Self {
        match value {
            Bc1ApproxMode::Ideal => rgbcx_sys::rgbcx_bc1_approx_mode_cBC1Ideal,
            Bc1ApproxMode::IdealRound4 => rgbcx_sys::rgbcx_bc1_approx_mode_cBC1IdealRound4,
            Bc1ApproxMode::Amd => rgbcx_sys::rgbcx_bc1_approx_mode_cBC1AMD,
            Bc1ApproxMode::Nvidia => rgbcx_sys::rgbcx_bc1_approx_mode_cBC1NVidia,
        }
    }
}

bitflags! {
    pub struct Flags: u32 {
        const EncodeBC1UseLikelyTotalOrderings = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseLikelyTotalOrderings;
        const EncodeBC1TwoLeastSquaresPasses = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1TwoLeastSquaresPasses;
        const EncodeBC1Use3ColorBlocksForBlackPixels = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use3ColorBlocksForBlackPixels;
        const EncodeBC1Use3ColorBlocks = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use3ColorBlocks;
        const EncodeBC1Iterative = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Iterative;
        const EncodeBC1BoundingBox = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1BoundingBox;
        const EncodeBC1UseFasterMSEEval = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseFasterMSEEval;
        const EncodeBC1UseFullMSEEval = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseFullMSEEval;
        const EncodeBC1Use2DLS = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use2DLS;
        const EncodeBC1Use6PowerIters = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use6PowerIters;
        const EncodeBC1Exhaustive = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Exhaustive;
        const EncodeBC1TryAllInitialEndponts = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1TryAllInitialEndponts;
        const EncodeBC1BoundingBoxInt = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1BoundingBoxInt;
        const EncodeBC1EndpointSearchRoundsShift = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1EndpointSearchRoundsShift;
        const EncodeBC1EndpointSearchRoundsMask = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1EndpointSearchRoundsMask;
    }
}
