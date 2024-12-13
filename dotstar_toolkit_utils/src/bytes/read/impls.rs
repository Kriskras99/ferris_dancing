use std::{borrow::Cow, fs::File, mem::MaybeUninit, ptr};

use hipstr::HipStr;
use positioned_io::{RandomAccessFile, ReadAt as PRead, Size};

use super::{BinaryDeserialize, ReadAt, ReadAtExt, ReadError};

// Add when specialization works
// impl<const N: usize> BinaryDeserialize<'_> for [u8; N] {
//     fn deserialize_at_with_ctx<'de>(
//         reader: &'de (impl ReadAtExt + ?Sized),
//         position: &mut u64,
//         _ctx: &(),
//     ) -> Result<Self, ReadError> {
//         let bytes = reader.read_slice_at(position, N)?;
//         let fixed_slice: [u8; N] = bytes.as_ref().try_into().unwrap_or_else(|_| unreachable!());
//         Ok(fixed_slice)
//     }
// }

impl<'de, T, const N: usize> BinaryDeserialize<'de> for [T; N]
where
    T: BinaryDeserialize<'de>,
    T::Output: Copy,
    T::Ctx: Copy,
{
    type Ctx = T::Ctx;
    type Output = [T::Output; N];

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut array = [const { MaybeUninit::<T::Output>::uninit() }; N];
        let mut i = 0;
        let old_position = *position;
        let result: Result<_, ReadError> = try {
            #[expect(clippy::arithmetic_side_effects, reason = "It's checked by N")]
            while i < N {
                let data: T::Output = reader.read_at_with::<T>(position, ctx)?;
                array[i] = MaybeUninit::new(data);
                i += 1;
            }
        };
        match result {
            Ok(()) => {
                // This would be better but the compiler can't proof that MaybeUninit<T> and T are the same size.
                // let array = unsafe { std::mem::transmute::<_, [T; N]>(array) };
                let array = unsafe {
                    *ptr::from_ref::<[MaybeUninit<T::Output>; N]>(&array).cast::<[T::Output; N]>()
                };
                Ok(array)
            }
            Err(err) => {
                *position = old_position;
                for mut item in array.into_iter().take(i) {
                    // Make sure any drop code is called
                    unsafe { item.assume_init_drop() }
                }
                Err(err)
            }
        }
    }
}

impl<'de, T1> BinaryDeserialize<'de> for (T1,)
where
    T1: BinaryDeserialize<'de>,
{
    type Ctx = (T1::Ctx,);
    type Output = (T1::Output,);
    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut tuple_1 = MaybeUninit::<T1::Output>::uninit();
        let old_position = *position;
        let mut i = 0;
        let result: Result<_, ReadError> = try {
            tuple_1.write(reader.read_at_with::<T1>(position, ctx.0)?);
            i = 1;
        };
        match result {
            Ok(()) => {
                let tuple = unsafe { (tuple_1.assume_init(),) };
                Ok(tuple)
            }
            Err(err) => {
                *position = old_position;
                // Drop any init tuple parts
                unsafe {
                    if i >= 1 {
                        tuple_1.assume_init_drop();
                    }
                }
                Err(err)
            }
        }
    }
}

impl<'de, T1, T2> BinaryDeserialize<'de> for (T1, T2)
where
    T1: BinaryDeserialize<'de>,
    T2: BinaryDeserialize<'de>,
{
    type Ctx = (T1::Ctx, T2::Ctx);
    type Output = (T1::Output, T2::Output);
    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut tuple_1 = MaybeUninit::<T1::Output>::uninit();
        let mut tuple_2 = MaybeUninit::<T2::Output>::uninit();
        let old_position = *position;
        let mut i = 0;
        let result: Result<_, ReadError> = try {
            tuple_1.write(reader.read_at_with::<T1>(position, ctx.0)?);
            i = 1;
            tuple_2.write(reader.read_at_with::<T2>(position, ctx.1)?);
            i = 2;
        };
        match result {
            Ok(()) => {
                let tuple = unsafe { (tuple_1.assume_init(), tuple_2.assume_init()) };
                Ok(tuple)
            }
            Err(err) => {
                *position = old_position;
                // Drop any init tuple parts
                unsafe {
                    if i >= 1 {
                        tuple_1.assume_init_drop();
                    }
                    if i >= 2 {
                        tuple_2.assume_init_drop();
                    }
                }
                Err(err)
            }
        }
    }
}

impl<'de, T1, T2, T3> BinaryDeserialize<'de> for (T1, T2, T3)
where
    T1: BinaryDeserialize<'de>,
    T2: BinaryDeserialize<'de>,
    T3: BinaryDeserialize<'de>,
{
    type Ctx = (T1::Ctx, T2::Ctx, T3::Ctx);
    type Output = (T1::Output, T2::Output, T3::Output);
    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut tuple_1 = MaybeUninit::<T1::Output>::uninit();
        let mut tuple_2 = MaybeUninit::<T2::Output>::uninit();
        let mut tuple_3 = MaybeUninit::<T3::Output>::uninit();
        let old_position = *position;
        let mut i = 0;
        let result: Result<_, ReadError> = try {
            tuple_1.write(reader.read_at_with::<T1>(position, ctx.0)?);
            i = 1;
            tuple_2.write(reader.read_at_with::<T2>(position, ctx.1)?);
            i = 2;
            tuple_3.write(reader.read_at_with::<T3>(position, ctx.2)?);
            i = 3;
        };
        match result {
            Ok(()) => {
                let tuple = unsafe {
                    (
                        tuple_1.assume_init(),
                        tuple_2.assume_init(),
                        tuple_3.assume_init(),
                    )
                };
                Ok(tuple)
            }
            Err(err) => {
                *position = old_position;
                // Drop any init tuple parts
                unsafe {
                    if i >= 1 {
                        tuple_1.assume_init_drop();
                    }
                    if i >= 2 {
                        tuple_2.assume_init_drop();
                    }
                    if i >= 3 {
                        tuple_3.assume_init_drop();
                    }
                }
                Err(err)
            }
        }
    }
}

impl<'de> BinaryDeserialize<'de> for HipStr<'de> {
    type Ctx = usize;
    type Output = Self;
    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        len: usize,
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            match reader.read_slice_at(position, len)? {
                Cow::Borrowed(slice) => std::str::from_utf8(slice)
                    .map(HipStr::borrowed)
                    .map_err(ReadError::from)?,
                Cow::Owned(vec) => String::from_utf8(vec)
                    .map(HipStr::from)
                    .map_err(ReadError::from)?,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl ReadAt for File {
    #[inline]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<HipStr<'static>, ReadError> {
        // Buffer used to read parts from the file
        let mut read_buf = vec![0; 0x10];
        // Buffer that stores the resulting string
        let mut result_buf = Vec::new();
        // Keep track of search position here, so that the original position is not affected
        let mut new_position = *position;
        loop {
            let bytes_read =
                PRead::read_at(self, new_position, &mut read_buf).map_err(ReadError::from)?;
            let bytes_read = u64::try_from(bytes_read)?;
            if bytes_read == 0 {
                // End of file reached, give up
                return Err(ReadError::no_null_byte(*position));
            }
            if let Some(found) = read_buf.iter().position(|b| *b == 0x0) {
                // Found null byte, add everything upto the null byte in `result_buf`
                result_buf.extend_from_slice(&read_buf[0..found]);
                let found = u64::try_from(found)?;
                let end_position = new_position
                    .checked_add(found)
                    .ok_or_else(ReadError::int_under_overflow)?;
                let string = String::from_utf8(result_buf).map_err(ReadError::from)?;
                // Set position past the null byte
                *position = end_position
                    .checked_add(1)
                    .ok_or_else(ReadError::int_under_overflow)?;
                return Ok(HipStr::from(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position
                .checked_add(bytes_read)
                .ok_or_else(ReadError::int_under_overflow)?;
        }
    }

    #[inline]
    fn read_slice_at(
        &self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_u64 = u64::try_from(len)?;
        let new_position = position
            .checked_add(len_u64)
            .ok_or_else(ReadError::int_under_overflow)?;
        let mut buf = vec![0; len];
        PRead::read_exact_at(self, *position, &mut buf).map_err(ReadError::from)?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

    fn len(&self) -> Result<u64, ReadError> {
        Ok(self.metadata()?.len())
    }
}

impl ReadAt for RandomAccessFile {
    #[inline]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<HipStr<'static>, ReadError> {
        // Buffer used to read parts from the file
        let mut read_buf = vec![0; 0x10];
        // Buffer that stores the resulting string
        let mut result_buf = Vec::new();
        // Keep track of search position here, so that the original position is not affected
        let mut new_position = *position;
        loop {
            let bytes_read =
                PRead::read_at(self, new_position, &mut read_buf).map_err(ReadError::from)?;
            let bytes_read = u64::try_from(bytes_read)?;
            if bytes_read == 0 {
                // End of file reached, give up
                return Err(ReadError::no_null_byte(*position));
            }
            if let Some(found) = read_buf.iter().position(|b| *b == 0x0) {
                // Found null byte, add everything upto the null byte in `result_buf`
                result_buf.extend_from_slice(&read_buf[0..found]);
                let found = u64::try_from(found)?;
                let end_position = new_position
                    .checked_add(found)
                    .ok_or_else(ReadError::int_under_overflow)?;
                let string = String::from_utf8(result_buf).map_err(ReadError::from)?;
                // Set position past the null byte
                *position = end_position
                    .checked_add(1)
                    .ok_or_else(ReadError::int_under_overflow)?;
                return Ok(HipStr::from(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position
                .checked_add(bytes_read)
                .ok_or_else(ReadError::int_under_overflow)?;
        }
    }

    #[inline]
    fn read_slice_at(
        &self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_u64 = u64::try_from(len)?;
        let new_position = position
            .checked_add(len_u64)
            .ok_or_else(ReadError::int_under_overflow)?;
        let mut buf = vec![0; len];
        PRead::read_exact_at(self, *position, &mut buf).map_err(ReadError::from)?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

    fn len(&self) -> Result<u64, ReadError> {
        self.size()?
            .ok_or_else(|| ReadError::custom("Unsupported".to_string()))
    }
}

impl ReadAt for [u8] {
    #[inline]
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<HipStr<'rf>, ReadError> {
        let position_usize = usize::try_from(*position)?;
        // Find the null byte, starting at `position_usize`
        let null_pos = self
            .iter()
            .skip(position_usize)
            .position(|b| b == &0)
            .and_then(|p| p.checked_add(position_usize));
        if let Some(null_pos) = null_pos {
            let null_pos_u64 = u64::try_from(null_pos)?;
            let string = HipStr::borrowed(std::str::from_utf8(&self[position_usize..null_pos])?);
            *position = null_pos_u64
                .checked_add(1)
                .ok_or_else(ReadError::int_under_overflow)?;
            Ok(string)
        } else {
            Err(ReadError::no_null_byte(*position))
        }
    }

    #[inline]
    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        let new_position = position
            .checked_add(u64::try_from(len)?)
            .ok_or_else(ReadError::int_under_overflow)?;
        let new_position_usize = usize::try_from(new_position)?;
        let position_usize = usize::try_from(*position)?;
        if self.len() < (new_position_usize) {
            Err(ReadError::unexpected_eof())
        } else {
            *position = new_position;
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    fn len(&self) -> Result<u64, ReadError> {
        u64::try_from(self.len()).map_err(ReadError::from)
    }
}

impl ReadAt for Vec<u8> {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<HipStr<'rf>, ReadError> {
        let position_usize = usize::try_from(*position)?;
        let null_pos = self
            .iter()
            .skip(position_usize)
            .position(|b| b == &0)
            .and_then(|p| p.checked_add(position_usize));
        if let Some(null_pos) = null_pos {
            let null_pos_u64 = u64::try_from(null_pos)?;
            let string = HipStr::borrowed(std::str::from_utf8(&self[position_usize..null_pos])?);
            *position = null_pos_u64
                .checked_add(1)
                .ok_or_else(ReadError::int_under_overflow)?;
            Ok(string)
        } else {
            Err(ReadError::no_null_byte(*position))
        }
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        let new_position = position
            .checked_add(u64::try_from(len)?)
            .ok_or_else(ReadError::int_under_overflow)?;
        let new_position_usize = usize::try_from(new_position)?;
        let position_usize = usize::try_from(*position)?;
        if self.len() < (new_position_usize) {
            Err(ReadError::unexpected_eof())
        } else {
            *position = new_position;
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    fn len(&self) -> Result<u64, ReadError> {
        u64::try_from(self.len()).map_err(ReadError::from)
    }
}

impl<R: ReadAt + ?Sized> ReadAt for &R {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<HipStr<'rf>, ReadError> {
        (*self).read_null_terminated_string_at(position)
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        (*self).read_slice_at(position, len)
    }

    fn len(&self) -> Result<u64, ReadError> {
        (*self).len()
    }
}
