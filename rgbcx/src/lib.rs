/// # rgbcx v1.12
/// High-performance scalar BC1-5 encoders. Public Domain or MIT license (you choose - see below), written by Richard Geldreich 2020 <richgel99@gmail.com>.
///
/// Influential references:
/// <http://sjbrown.co.uk/2006/01/19/dxt-compression-techniques/>
/// <https://github.com/nothings/stb/blob/master/stb_dxt.h>
/// <https://gist.github.com/castano/c92c7626f288f9e99e158520b14a61cf>
/// <https://github.com/castano/icbc/blob/master/icbc.h>
/// <http://www.humus.name/index.php?page=3D&ID=79>
///
/// ## Instructions:
///
/// The library MUST be initialized by calling [`get_rgbcx()`] or by manually initializing [`RGBCX`] for custom approximation modes.
/// You can then use the various encode and decode functions on the [`Rgbcx`] type.
///
/// Common options:
/// - `level`: ranges from [`MIN_LEVEL`] to [`MAX_LEVEL`]. The higher the level, the slower the encoder goes, but the higher the average quality.
///     levels [0,4] are fast and compete against stb_dxt (default and HIGHQUAL). The remaining levels compete against squish/NVTT/icbc and icbc HQ.
///     If in doubt just use level 10, set `allow_3color` to true and `use_transparent_texels_for_black` to false, and adjust as needed.
///
/// - `allow_3color`: If true the encoder will use 3-color blocks. This flag is ignored unless level is >= 5 (because lower levels compete against stb_dxt and it doesn't support 3-color blocks).
///     3-color block usage slows down encoding.
///
/// - `use_transparent_texels_for_black`: If true the encoder will use 3-color block transparent black pixels to code very dark or black texels. Your engine/shader MUST ignore the sampled
///     alpha value for textures encoded in this mode. This is how NVidia's classic "nvdxt" encoder (used by many original Xbox titles) used to work by default on DXT1C textures. It increases
///     average quality substantially (because dark texels/black are very common) and is highly recommended.
///
/// ## Approximation mode
/// Important: BC1/3 textures encoded using non-ideal BC1 approximation modes should only be sampled on parts from that vendor.
/// If you encode for AMD, average error on AMD parts will go down, but average error on NVidia parts will go up and vice versa.
/// If in doubt, encode in ideal BC1 mode.
///
use std::{ffi::c_void, sync::OnceLock};

use bitflags::bitflags;

/// There can only be one instance of the encoder, but it's safe to share across threads
pub static RGBCX: OnceLock<Rgbcx> = OnceLock::new();
/// Get an instance of the encoder
pub fn get_rgbcx() -> &'static Rgbcx {
    RGBCX.get_or_init(Rgbcx::new)
}
/// Get an instance of the encoder with the specified approximation mode.
///
/// See [`Bc1ApproxMode`] for details
///
/// # Panics
/// Will panic if there already is a `Rgbcx` object initialised which does not match the approximation mode.
pub fn get_rgbcx_with_bc1_approx_mode(bc1_approx_mode: Bc1ApproxMode) -> &'static Rgbcx {
    let rgbcx = RGBCX.get_or_init(|| Rgbcx::with_bc1_approx_mode(bc1_approx_mode));
    assert!(
        rgbcx.approx_mode != bc1_approx_mode,
        "There is already a Rgbcx object initialised that does not match {bc1_approx_mode:?}"
    );
    rgbcx
}

/// Minimum total orderings that will be tried
pub const MIN_TOTAL_ORDERINGS: u32 = rgbcx_sys::rgbcx_MIN_TOTAL_ORDERINGS;
/// Maximum total orderings that will be tried for 4-color blocks
pub const MAX_TOTAL_ORDERINGS: u32 = rgbcx_sys::rgbcx_MAX_TOTAL_ORDERINGS4;
/// Maximum total orderings that will be tried for 3-color blocks
pub const MAX_TOTAL_ORDERINGS3: u32 = rgbcx_sys::rgbcx_MAX_TOTAL_ORDERINGS3;
/// Default total orderings that will be tried for 4-color blocks
/// Is around 3x faster than libsquish at slightly higher average quality. 10-16 is a good range to start to compete against libsquish.
pub const DEFAULT_TOTAL_ORDERINGS_TO_TRY: u32 = rgbcx_sys::rgbcx_DEFAULT_TOTAL_ORDERINGS_TO_TRY;
/// Default total orderings that will be tried for 3-color blocks
pub const DEFAULT_TOTAL_ORDERINGS_TO_TRY3: u32 = rgbcx_sys::rgbcx_DEFAULT_TOTAL_ORDERINGS_TO_TRY3;
/// The minimum level supported by the level-based functions
#[allow(
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    reason = "Try from is not const yet, manually checked"
)]
pub const MIN_LEVEL: u8 = rgbcx_sys::rgbcx_MIN_LEVEL as u8;
/// The maximum level supported by the level-based functions
#[allow(
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    reason = "Try from is not const yet, manually checked"
)]
pub const MAX_LEVEL: u8 = rgbcx_sys::rgbcx_MAX_LEVEL as u8;

/// The encoder with it's global state
pub struct Rgbcx {
    /// The approximation mode the encoder is set to
    approx_mode: Bc1ApproxMode,
}

impl Rgbcx {
    /// Initialize the Rgbcx encoder with the default approximation mode
    ///
    /// This function is not thread-safe!
    #[must_use]
    fn new() -> Self {
        unsafe { rgbcx_sys::rgbcx_init(rgbcx_sys::rgbcx_bc1_approx_mode_cBC1Ideal) };
        Self {
            approx_mode: Bc1ApproxMode::Ideal,
        }
    }

    /// Initialize the Rgbcx encoder with a approximation mode
    ///
    /// This function is not thread-safe!
    #[must_use]
    fn with_bc1_approx_mode(bc1_approx_mode: Bc1ApproxMode) -> Self {
        unsafe { rgbcx_sys::rgbcx_init(bc1_approx_mode.into()) };
        Self {
            approx_mode: bc1_approx_mode,
        }
    }

    /// Optimally encodes a solid color block to BC1 format.
    #[must_use]
    pub fn encode_bc1_solid_block(&self, fr: u32, fg: u32, fb: u32, allow_3color: bool) -> [u8; 8] {
        let mut buf = [0; 8];
        self.encode_bc1_solid_block_mut(&mut buf, fr, fg, fb, allow_3color);
        buf
    }

    /// Optimally encodes a solid color block to BC1 format.
    pub fn encode_bc1_solid_block_mut(
        &self,
        encoded: &mut [u8; 8],
        fr: u32,
        fg: u32,
        fb: u32,
        allow_3color: bool,
    ) {
        let buf_cvoid = encoded.as_mut_ptr().cast::<c_void>();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc1_solid_block(buf_cvoid, fr, fg, fb, allow_3color);
        }
    }

    /// Encodes a 4x4 block of RGBX (X=ignored) pixels to BC1 format.
    ///
    /// This is the simplified interface for BC1 encoding, which accepts a level parameter and converts that to the best overall flags.
    /// The pixels are in RGBA format, where R is first in memory. The BC1 encoder completely ignores the alpha channel (i.e. there is no punchthrough alpha support).
    /// This is the recommended function to use for BC1 encoding, becuase it configures the encoder for you in the best possible way (on average).
    /// Note that the 3 color modes won't be used at all until level 5 or higher.
    /// No transparency supported, however if you set `use_transparent_texels_for_black` to true the encoder will use transparent selectors on very dark/black texels to reduce MSE.
    #[must_use]
    pub fn encode_bc1_block(
        &self,
        pixels: &[u8; 64],
        level: u8,
        allow_3color: bool,
        use_transparent_texels_for_black: bool,
    ) -> [u8; 8] {
        let mut buf = [0; 8];
        self.encode_bc1_block_mut(
            pixels,
            &mut buf,
            level,
            allow_3color,
            use_transparent_texels_for_black,
        );
        buf
    }

    /// Encodes a 4x4 block of RGBX (X=ignored) pixels to BC1 format.
    ///  
    /// This is the simplified interface for BC1 encoding, which accepts a level parameter and converts that to the best overall flags.
    /// The pixels are in RGBA format, where R is first in memory. The BC1 encoder completely ignores the alpha channel (i.e. there is no punchthrough alpha support).
    /// This is the recommended function to use for BC1 encoding, becuase it configures the encoder for you in the best possible way (on average).
    /// Note that the 3 color modes won't be used at all until level 5 or higher.
    /// No transparency supported, however if you set `use_transparent_texels_for_black` to true the encoder will use transparent selectors on very dark/black texels to reduce MSE.
    ///
    /// # Panics
    /// Will panic if `level` is larger than [`MAX_LEVEL`]
    pub fn encode_bc1_block_mut(
        &self,
        pixels: &[u8; 64],
        encoded: &mut [u8; 8],
        level: u8,
        allow_3color: bool,
        use_transparent_texels_for_black: bool,
    ) {
        assert!(level <= MAX_LEVEL + 1, "Level is too big!");
        let level = u32::from(level);
        let buf_cvoid = encoded.as_mut_ptr().cast::<c_void>();
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc1(
                level,
                buf_cvoid,
                pixels,
                allow_3color,
                use_transparent_texels_for_black,
            );
        }
    }

    /// Low-level interface for BC1 encoding.
    ///
    /// Always returns a 4 color block, unless cEncodeBC1Use3ColorBlocksForBlackPixels or cEncodeBC1Use3ColorBlock flags are specified.
    /// `total_orderings_to_try` controls the perf. vs. quality tradeoff on 4-color blocks when the [`Flags::EncodeBC1UseLikelyTotalOrderings`] flag is used. It must range between [[`MIN_TOTAL_ORDERINGS`], [`MAX_TOTAL_ORDERINGS`]].
    /// `total_orderings_to_try3` controls the perf. vs. quality tradeoff on 3-color bocks when the [`Flags::EncodeBC1UseLikelyTotalOrderings`] and the [`Flags::EncodeBC1Use3ColorBlocks`] flags are used. Valid range is [0,[`MAX_TOTAL_ORDERINGS3`]] (0=disabled).
    #[must_use]
    pub fn encode_bc1_block_with_flags(
        &self,
        pixels: &[u8; 64],
        flags: &Flags,
        total_orderings_to_try: u32,
        total_orderings_to_try3: u32,
    ) -> [u8; 8] {
        let mut buf = [0; 8];
        self.encode_bc1_block_with_flags_mut(
            pixels,
            &mut buf,
            flags,
            total_orderings_to_try,
            total_orderings_to_try3,
        );
        buf
    }

    /// Low-level interface for BC1 encoding.
    ///
    /// Always returns a 4 color block, unless cEncodeBC1Use3ColorBlocksForBlackPixels or cEncodeBC1Use3ColorBlock flags are specified.
    /// `total_orderings_to_try` controls the perf. vs. quality tradeoff on 4-color blocks when the [`Flags::EncodeBC1UseLikelyTotalOrderings`] flag is used. It must range between [[`MIN_TOTAL_ORDERINGS`], [`MAX_TOTAL_ORDERINGS`]].
    /// `total_orderings_to_try3` controls the perf. vs. quality tradeoff on 3-color bocks when the [`Flags::EncodeBC1UseLikelyTotalOrderings`] and the [`Flags::EncodeBC1Use3ColorBlocks`] flags are used. Valid range is [0,[`MAX_TOTAL_ORDERINGS3`]] (0=disabled).
    pub fn encode_bc1_block_with_flags_mut(
        &self,
        pixels: &[u8; 64],
        encoded: &mut [u8; 8],
        flags: &Flags,
        total_orderings_to_try: u32,
        total_orderings_to_try3: u32,
    ) {
        let flags = flags.bits();
        let buf_cvoid = encoded.as_mut_ptr().cast::<c_void>();
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc11(
                buf_cvoid,
                pixels,
                flags,
                total_orderings_to_try,
                total_orderings_to_try3,
            );
        }
    }

    /// Encodes a 4x4 block of RGBA pixels to BC3 format.
    ///
    /// This is the recommended function, which accepts a level parameter.
    #[must_use]
    pub fn encode_bc3_block(&self, pixels: &[u8; 64], level: u8) -> [u8; 16] {
        let mut buf = [0; 16];
        self.encode_bc3_block_mut(pixels, &mut buf, level);
        buf
    }

    /// Encodes a 4x4 block of RGBA pixels to BC3 format.
    ///
    /// This is the recommended function, which accepts a level parameter.
    ///
    /// # Panics
    /// Will panic if `level` is larger than [`MAX_LEVEL`]
    pub fn encode_bc3_block_mut(&self, pixels: &[u8; 64], encoded: &mut [u8; 16], level: u8) {
        assert!(level <= MAX_LEVEL + 1, "Level is too big!");
        let level = u32::from(level);
        let buf_cvoid = encoded.as_mut_ptr().cast::<c_void>();
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc3(level, buf_cvoid, pixels);
        }
    }

    /// Encodes a 4x4 block of RGBA pixels to BC3 format.
    ///
    /// This is  a low-level version that allows fine control over BC1 encoding.
    #[must_use]
    pub fn encode_bc3_block_with_flags(
        &self,
        pixels: &[u8; 64],
        flags: &Flags,
        total_orderings_to_try: u32,
    ) -> [u8; 16] {
        let mut buf = [0; 16];
        self.encode_bc3_block_with_flags_mut(pixels, &mut buf, flags, total_orderings_to_try);
        buf
    }

    /// Encodes a 4x4 block of RGBA pixels to BC3 format.
    ///
    /// This is  a low-level version that allows fine control over BC1 encoding.
    pub fn encode_bc3_block_with_flags_mut(
        &self,
        pixels: &[u8; 64],
        encoded: &mut [u8; 16],
        flags: &Flags,
        total_orderings_to_try: u32,
    ) {
        let flags = flags.bits();
        let buf_cvoid = encoded.as_mut_ptr().cast::<c_void>();
        let pixels = pixels.as_ptr();
        unsafe {
            rgbcx_sys::rgbcx_encode_bc31(buf_cvoid, pixels, flags, total_orderings_to_try);
        }
    }

    /// Unpack a BC1 block to a 4x4 block of RGBA pixels.
    ///
    /// Returns true if the block uses 3 color punchthrough alpha mode.
    pub fn unpack_bc1_mut(
        &self,
        encoded: &[u8; 8],
        pixels: &mut [u8; 64],
        set_alpha: Option<bool>,
    ) -> bool {
        let set_alpha = set_alpha.unwrap_or(true);
        let mode = self.approx_mode;
        let encoded = encoded.as_ptr().cast::<c_void>();
        let pixels = pixels.as_mut_ptr().cast::<c_void>();
        unsafe { rgbcx_sys::rgbcx_unpack_bc1(encoded, pixels, set_alpha, mode.into()) }
    }

    /// Unpack a BC3 block to a 4x4 block of RGBA pixels.
    ///
    /// Returns true if the block uses 3 color punchthrough alpha mode.
    pub fn unpack_bc3_mut(&self, encoded: &[u8; 16], pixels: &mut [u8; 64]) -> bool {
        let mode = self.approx_mode;
        let encoded = encoded.as_ptr().cast::<c_void>();
        let pixels = pixels.as_mut_ptr().cast::<c_void>();
        unsafe { rgbcx_sys::rgbcx_unpack_bc3(encoded, pixels, mode.into()) }
    }
}

/// How to approximate colors
///
/// Important: If you encode textures for a specific vendor's GPU's, beware that using that texture data on other GPU's may result in ugly artifacts.
/// Encode to [`Bc1ApproxMode::Ideal`] unless you know the texture data will only be deployed or used on a specific vendor's GPU.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Bc1ApproxMode {
    /// The default mode. No rounding for 4-color colors 2,3. Older tools/compressors use this mode.
    /// This matches the D3D10 docs on BC1.
    #[default]
    Ideal,
    /// This mode matches AMD Compressonator's output. It rounds 4-color colors 2,3 (not 3-color color 2).
    /// This matches the D3D9 docs on DXT1.
    IdealRound4,
    /// AMD GPU mode.
    Amd,
    /// NVidia GPU mode.
    Nvidia,
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
    /// BC1 low-level API encoder flags. You can ignore this if you use the simple level API
    #[derive(Debug, Clone, Copy)]
    pub struct Flags: u32 {
        /// Try to improve quality using the most likely total orderings.
        /// The total_orderings_to_try parameter will then control the number of total orderings to try for 4 color blocks, and the
        /// total_orderings_to_try3 parameter will control the number of total orderings to try for 3 color blocks (if they are enabled).
        const EncodeBC1UseLikelyTotalOrderings = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseLikelyTotalOrderings;
        /// Use 2 least squares pass, instead of one (same as stb_dxt's HIGHQUAL option).
        /// Recommended if you're enabling cEncodeBC1UseLikelyTotalOrderings.
        const EncodeBC1TwoLeastSquaresPasses = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1TwoLeastSquaresPasses;
        /// cEncodeBC1Use3ColorBlocksForBlackPixels allows the BC1 encoder to use 3-color blocks for blocks containing black or very dark pixels.
        /// You shader/engine MUST ignore the alpha channel on textures encoded with this flag.
        /// Average quality goes up substantially for my 100 texture corpus (~.5 dB), so it's worth using if you can.
        /// Note the BC1 encoder does not actually support transparency in 3-color mode.
        /// Don't set when encoding to BC3.
        const EncodeBC1Use3ColorBlocksForBlackPixels = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use3ColorBlocksForBlackPixels;
        /// If cEncodeBC1Use3ColorBlocks is set, the encoder can use 3-color mode for a small but noticeable gain in average quality, but lower perf.
        /// If you also specify the cEncodeBC1UseLikelyTotalOrderings flag, set the total_orderings_to_try3 paramter to the number of total orderings to try.
        /// Don't set when encoding to BC3.
        const EncodeBC1Use3ColorBlocks = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use3ColorBlocks;
        /// cEncodeBC1Iterative will greatly increase encode time, but is very slightly higher quality.
        /// Same as squish's iterative cluster fit option. Not really worth the tiny boost in quality, unless you just don't care about perf. at all.
        const EncodeBC1Iterative = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Iterative;
        /// cEncodeBC1BoundingBox enables a fast all-integer PCA approximation on 4-color blocks.
        /// At level 0 options (no other flags), this is ~15% faster, and higher *average* quality.
        const EncodeBC1BoundingBox = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1BoundingBox;
        /// Use a slightly lower quality, but ~30% faster MSE evaluation function for 4-color blocks.
        const EncodeBC1UseFasterMSEEval = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseFasterMSEEval;
        /// Examine all colors to compute selectors/MSE (slower than default)
        const EncodeBC1UseFullMSEEval = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1UseFullMSEEval;
        /// Use 2D least squares+inset+optimal rounding (the method used in Humus's GPU texture encoding demo), instead of PCA.
        /// Around 18% faster, very slightly lower average quality to better (depends on the content).
        const EncodeBC1Use2DLS = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use2DLS;
        /// Use 6 power iterations vs. 4 for PCA.
        const EncodeBC1Use6PowerIters = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Use6PowerIters;
        /// Check all total orderings - *very* slow. The encoder is not designed to be used in this way.
        const EncodeBC1Exhaustive = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1Exhaustive;
        /// Try 2 different ways of choosing the initial endpoints.
        const EncodeBC1TryAllInitialEndponts = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1TryAllInitialEndponts;
        /// Same as cEncodeBC1BoundingBox, but implemented using integer math (faster, slightly less quality)
        const EncodeBC1BoundingBoxInt = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1BoundingBoxInt;
        /// Try refining the final endpoints by examining nearby colors.
        const EncodeBC1EndpointSearchRoundsShift = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1EndpointSearchRoundsShift;
        /// Try refining the final endpoints by examining nearby colors.
        const EncodeBC1EndpointSearchRoundsMask = rgbcx_sys::rgbcx_AdvancedSettings_cEncodeBC1EndpointSearchRoundsMask;
    }
}
