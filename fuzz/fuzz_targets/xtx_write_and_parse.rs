#![no_main]

extern crate libfuzzer_sys;

use ubiart_toolkit::cooked::png::Png;
use libfuzzer_sys::fuzz_target;
use dotstar_toolkit_utils::bytes::write::BinarySerialize;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;

fuzz_target!(|png: Png| {
    if let Ok(vec) = ubiart_toolkit::cooked::png::create_vec(&png) {
        let new_png = Png::deserialize(&vec).unwrap_or_else(|error| panic!("{error:#?}\n\n{vec:02X?}"));
        assert_eq!(png, new_png);
    }
});
