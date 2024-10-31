//! Contains the parser implementation

use dotstar_toolkit_utils::bytes::{
    endian::Endian,
    primitives::u32be,
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};
use test_eq::{test_any, test_eq, test_or};

use super::MovementSpaceMove;

impl<'de> BinaryDeserialize<'de> for MovementSpaceMove<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self, ReadError> {
        // Check the magic
        let unk1 = reader.read_at::<u32be>(position)?;
        test_any!(unk1, [0x1, 0x0100_0000])?;

        let endianness = if unk1 == 1 {
            Endian::Big
        } else {
            Endian::Little
        };

        // Version is 0x7 for JD2017 and newer
        let version = reader.read_at_with::<u32>(position, endianness)?;
        test_any!(version, [0x5, 0x6, 0x7])?;

        // There are always 64 bytes for the string, so we read untill the null byte.
        // If the null byte is past 64 bytes there's something wrong and we error.
        let start = *position;
        let name = reader.read_null_terminated_string_at(position)?;
        if position.checked_sub(start).unwrap() > 64 {
            return Err(ReadError::no_null_byte(start));
        }
        *position = start + 64;

        let start = *position;
        let map = reader.read_null_terminated_string_at(position)?;
        if position.checked_sub(start).unwrap() > 64 {
            return Err(ReadError::no_null_byte(start));
        }
        *position = start + 64;

        let start = *position;
        let device = reader.read_null_terminated_string_at(position)?;
        if position.checked_sub(start).unwrap() > 64 {
            return Err(ReadError::no_null_byte(start));
        }
        *position = start + 64;

        test_any!(device.as_ref(), ["Acc_Dev_Dir_NP", "Acc_Dev_Dir_10P"])?;

        let unk3 = reader.read_at_with::<f32>(position, endianness)?;
        test_any!(unk3, 0.0..=3.503_999_7)?;
        let unk4 = reader.read_at_with::<f32>(position, endianness)?;
        test_any!(
            unk4,
            [
                -1.0,
                0.4,
                0.7,
                0.799_999_95,
                0.8,
                0.9,
                1.0,
                1.1,
                1.199_999_9,
                1.2,
                1.3,
                1.300_000_1,
                1.4
            ]
        )?;
        let unk5 = reader.read_at_with::<f32>(position, endianness)?;
        test_any!(
            unk5,
            [-1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0]
        )?;

        let (unk6, unk7) = if version == 0x7 {
            let unk6 = reader.read_at_with::<f32>(position, endianness)?;
            test_any!(
                unk6,
                [-1.0, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3]
            )?;
            let unk7 = reader.read_at_with::<f32>(position, endianness)?;
            test_any!(
                unk7,
                [
                    -1.0,
                    0.0,
                    0.1,
                    0.2,
                    0.3,
                    0.4,
                    0.5,
                    0.6,
                    0.7,
                    0.8,
                    0.9,
                    0.900_000_04,
                    1.0
                ]
            )?;
            (Some(unk6), Some(unk7))
        } else {
            (None, None)
        };

        if endianness == Endian::Little {
            let unk7_5 = reader.read_at_with::<u32>(position, endianness)?;
            test_eq!(unk7_5, 0)?;
        }

        let unk8 = reader.read_at_with::<u32>(position, endianness)?;
        test_eq!(unk8, 0x211C_0000)?;
        let unk9 = reader.read_at_with::<u32>(position, endianness)?;
        test_eq!(unk9, 0)?;

        let unk10 = if endianness == Endian::Big {
            let unk10 = reader.read_at_with::<u32>(position, endianness)?;
            test_any!(unk10, 0x0..=0x3)?;
            Some(unk10)
        } else {
            None
        };

        let unk11 = reader.read_at_with::<u32>(position, endianness)?;
        // In steps of 5
        test_any!(unk11, 10..=210).and(test_eq!(unk11 % 5, 0))?;
        let unk12 = reader.read_at_with::<u32>(position, endianness)?;
        test_eq!(unk12, 2)?;
        let unk13 = reader.read_at_with::<u32>(position, endianness)?;
        // Only two in WiiU 2015 Bundle_0_WIIU.ipk/world/jd2015/loveisall/timeline/moves/wiiu/loveisall_transmutation.msm
        test_any!(unk13, [0, 2])?;

        let unk14 = reader.read_at_with::<f32>(position, endianness)?;
        test_any!(unk14, -212.423_26..=156.828_7)?;
        let unk15 = reader.read_at_with::<f32>(position, endianness)?;
        test_any!(unk15, -115.599_64..=54.538_467)?;

        let size = (reader.len()? - *position) / 4;
        assert_eq!(size % 2, 0, "Size should be a multiple of two");

        if unk13 == 0 {
            test_eq!(u64::from(unk11), size / 2)?;
        } else {
            // unk13 indicates a second movement pattern?
            // this includes a second unk14 and unk15
            // so we subtract the already parsed unk14 and unk15 from the calculation
            test_eq!(u64::from(unk11 * unk13 + unk13) - 1, size / 2)?;
        }

        let mut data = Vec::with_capacity(usize::try_from(unk11)?);
        for _ in 0..unk11 {
            let x = reader.read_at_with::<f32>(position, endianness)?;
            // Most of the time x is quite small, but sometimes very big
            test_or!(
                test_any!(x, -295.96005..=20007.822),
                test_any!(x, 172_796_720_000.0..=1_133_755_800_000.0)
            )?;
            let y = reader.read_at_with::<f32>(position, endianness)?;
            // Most of the time y is quite small, but sometimes very big
            test_or!(
                test_any!(y, -337.18118..=69163.67),
                test_any!(y, 49_857_210_000.0..=1_716_389_200_000.0)
            )?;
            data.push((x, y));
        }

        Ok(MovementSpaceMove {
            name,
            map,
            device,
            data,
            version,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk10,
            unk11,
            unk13,
            unk14,
            unk15,
        })
    }
}
