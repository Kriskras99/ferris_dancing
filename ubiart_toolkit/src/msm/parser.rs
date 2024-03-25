//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::{test, test_le},
};

use super::MovementSpaceMove;

impl<'de> BinaryDeserialize<'de> for MovementSpaceMove<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        // Check the magic
        let unk1 = reader.read_at::<u32be>(position)?.into();
        test(&unk1, &0x1u32)?;
        let unk2 = reader.read_at::<u32be>(position)?.into();
        test(&unk2, &0x7u32)?;

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

        test(&device.as_ref(), &"Acc_Dev_Dir_NP")?;

        let unk3 = reader.read_at::<u32be>(position)?.into();
        let unk4 = reader.read_at::<u32be>(position)?.into();
        let unk5 = reader.read_at::<u32be>(position)?.into();
        let unk6 = reader.read_at::<u32be>(position)?.into();
        let unk7 = reader.read_at::<u32be>(position)?.into();

        let unk8 = reader.read_at::<u32be>(position)?.into();
        test(&unk8, &0x211C_0000u32)?;
        let unk9 = reader.read_at::<u32be>(position)?.into();
        test(&unk9, &0x0u32)?;
        let unk10 = reader.read_at::<u32be>(position)?.into();
        test_le(&unk10, &0x3u32)?;
        let points = reader.read_at::<u32be>(position)?.into();
        let unk12 = reader.read_at::<u32be>(position)?.into();
        test(&unk12, &0x2u32)?;
        let unk13 = reader.read_at::<u32be>(position)?.into();
        test(&unk13, &0x0u32)?;

        let unk14 = reader.read_at::<u32be>(position)?.into();
        let unk15 = reader.read_at::<u32be>(position)?.into();

        let mut data = Vec::with_capacity(usize::try_from(points)?);
        for _ in 0..points {
            let x = reader.read_at::<u32be>(position)?.into();
            let y = reader.read_at::<u32be>(position)?.into();
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
            unk10,
            unk14,
            unk15,
        })
    }
}
