//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_any, test_eq, test_le,
};

use super::MovementSpaceMove;

impl<'de> BinaryDeserialize<'de> for MovementSpaceMove<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        // Check the magic
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x1)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, [0x5, 0x6, 0x7])?;

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

        let unk3 = reader.read_at::<u32be>(position)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        let unk7 = reader.read_at::<u32be>(position)?;

        let unk8 = reader.read_at::<u32be>(position)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_any!(unk9, [0, 20, 25, 30, 40, 45, 50, 55, 60, 65, 85])?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_le!(unk10, 0x3)?;
        let points = reader.read_at::<u32be>(position)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        let unk13 = reader.read_at::<u32be>(position)?;

        let unk14 = reader.read_at::<u32be>(position)?;
        let unk15 = reader.read_at::<u32be>(position)?;

        let mut data = Vec::with_capacity(usize::try_from(points)?);
        for _ in 0..points {
            let x = reader.read_at::<u32be>(position)?;
            let y = reader.read_at::<u32be>(position)?;
            data.push((x, y));
        }

        Ok(MovementSpaceMove {
            name,
            map,
            device,
            data,
            points,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            unk9,
            unk10,
            unk12,
            unk13,
            unk14,
            unk15,
        })
    }
}
