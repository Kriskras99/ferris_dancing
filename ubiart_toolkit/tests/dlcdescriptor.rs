#![allow(clippy::needless_pass_by_value, reason = "Needed by test runner")]

use std::path::Path;

use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt;
use ubiart_toolkit::cooked::dlcdescriptor::DlcDescriptor;

fn dlcdescriptor_parse_wiiu2015(_path: &Path, data: Vec<u8>) -> datatest_stable::Result<()> {
    DlcDescriptor::deserialize(&data)?;
    Ok(())
}

datatest_stable::harness!(
    dlcdescriptor_parse_wiiu2015,
    "files/wiiu2015",
    r"dlc.*/dlcdescriptor.ckd/.*",
);
