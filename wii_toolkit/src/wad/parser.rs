//! Contains the parser implementation
use std::{fs, path::Path};

use anyhow::{anyhow, Error};
use byteorder::BigEndian;
use cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use dotstar_toolkit_utils::{
    bytes::{read_slice_at, read_u16_at, read_u32_at, read_u64_at, read_u8_at},
    testing::test,
};
use memmap2::Mmap;
use yoke::Yoke;

use super::types::{
    AccessRights, Aes128CbcDec, Content, ContentMetadata, ContentType, InstallableArchive, Region,
    TicketMetadata, TitleMetadata, TitleType, WadArchive, WadArchiveOwned, WadType, MAGIC_BK,
    MAGIC_IB, MAGIC_IS,
};

/// Decrypt the data inplace
fn aes_128_cbc_decrypt_inplace(data: &mut [u8], iv: &[u8], key: &[u8]) {
    let mut decrypter = Aes128CbcDec::new(key.into(), iv.into());
    for chunk in data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        decrypter.decrypt_block_mut(block);
    }
}

/// Checks if this file is a WAD archive
#[must_use]
pub fn can_parse(first_bytes: &[u8; 0x8]) -> bool {
    first_bytes[..0x6] == MAGIC_IS
        || first_bytes[..0x6] == MAGIC_IB
        || first_bytes[..0x6] == MAGIC_BK
}

/// Open the file at the given path and parse it as a Wii WAD file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<WadArchiveOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse(data))?;
    Ok(WadArchiveOwned::from(yoke))
}

/// Parse a Wii WAD file
///
/// # Errors
/// Will error if `src` is not large enough
/// Will error if there is incorrect data
pub fn parse(src: &[u8]) -> Result<WadArchive<'_>, Error> {
    let mut position = 0;
    let size = read_u32_at::<BigEndian>(src, &mut position)?;
    let wad_type = WadType::try_from(read_u16_at::<BigEndian>(src, &mut position)?)?;
    match (size, wad_type) {
        (0x20, WadType::Bootable | WadType::Installable) => parse_installable(src, &mut position, wad_type),
        (0x70, WadType::Backup) => Err(anyhow!("Backup WAD parsing not yet implemented!")),
        _ => Err(anyhow!("Unknown WAD type found or file is not a WAD file! Metadata size: {size}, WAD type: {wad_type:?}")),
    }
}

/// Parse the `src` as an installable WAD archive
///
/// # Errors
/// Will error if `src` is not large enough
/// Will error if there is incorrect data
fn parse_installable<'a>(
    src: &'a [u8],
    position: &mut usize,
    wad_type: WadType,
) -> Result<WadArchive<'a>, Error> {
    // parse header
    let version = read_u16_at::<BigEndian>(src, position)?;
    let cert_chain_size = read_u32_at::<BigEndian>(src, position)?;
    let unk1 = read_u32_at::<BigEndian>(src, position)?;
    let ticket_size = read_u32_at::<BigEndian>(src, position)?;
    let tmd_size = read_u32_at::<BigEndian>(src, position)?;
    let _content_size = read_u32_at::<BigEndian>(src, position)?;
    let _footer_size = read_u32_at::<BigEndian>(src, position)?;

    // verify known constant values`
    test(&version, &0x0)?;
    test(&unk1, &0x0)?;

    // skip cert chain
    // TODO: implement verification of cert chain
    *position = position
        .checked_add(round_to_boundary(usize::try_from(cert_chain_size)?))
        .expect("Overflow occurred!");

    // parse ticket data
    let ticket_start = *position;
    let ticket_metadata = parse_ticket(src, position)?;
    *position = round_to_boundary(*position);
    let ticket_end = *position;
    test(
        &ticket_end.checked_sub(ticket_start),
        &Some(usize::try_from(ticket_size)?),
    )?;

    // parse title_metadata
    let tmd_start = *position;
    let title_metadata = parse_tmd(src, position)?;
    *position = round_to_boundary(*position);
    let tmd_end = *position;
    test(
        &tmd_end.checked_sub(tmd_start),
        &Some(usize::try_from(tmd_size)?),
    )?;

    let content = parse_content(src, position, &ticket_metadata, &title_metadata)?;

    // TODO: Parse footer?

    Ok(WadArchive::Installable(InstallableArchive {
        wad_type,
        ticket_metadata,
        title_metadata,
        content,
    }))
}

/// Wii Common Key, used to decrypt the title key
const COMMON_KEY: [u8; 0x10] = [
    0xEB, 0xE4, 0x2A, 0x22, 0x5E, 0x85, 0x93, 0xE4, 0x48, 0xD9, 0xC5, 0x45, 0x73, 0x81, 0xAA, 0xF7,
];

/// Parse the ticket metadata
///
/// # Errors
/// Will error if `src` is not large enough
/// Will error if there is incorrect data
fn parse_ticket(src: &[u8], position: &mut usize) -> Result<TicketMetadata, Error> {
    // skip signature, padding, issuer and ECDH data
    // TODO: Verify signature
    *position = position.checked_add(0x1BC).expect("Overflow occurred!");
    // Read the metadata
    let format_version = read_u8_at(src, position)?;
    let unk1 = read_u16_at::<BigEndian>(src, position)?;
    test(&unk1, &0x0)?;
    let title_key_orig: &[u8; 0x10] = read_slice_at(src, position)?;
    let mut title_key = *title_key_orig;
    let unk2 = read_u8_at(src, position)?; // TODO: Figure out this value
    test(&unk2, &0x0)?;
    let ticket_id = read_u64_at::<BigEndian>(src, position)?;
    let console_id = read_u32_at::<BigEndian>(src, position)?;
    let title_id = read_u64_at::<BigEndian>(src, position)?;
    let unk3 = read_u16_at::<BigEndian>(src, position)?;
    test(&unk3, &0xFFFF)?;
    let title_version = read_u16_at::<BigEndian>(src, position)?;
    let permitted_titles_mask = read_u64_at::<BigEndian>(src, position)?;
    let permit_mask = read_u64_at::<BigEndian>(src, position)?;
    let tea = read_u8_at(src, position)?;
    let title_export_allowed = if tea == 1 {
        true
    } else if tea == 0 {
        false
    } else {
        return Err(anyhow!("Title export allowed is not a boolean: {tea:x}"));
    };
    let common_key_index = read_u8_at(src, position)?;
    test(&common_key_index, &0x0)?;
    // Skip remainder of the header
    // TODO: Parse this?
    *position = position.checked_add(0xB2).expect("Overflow occurred!");

    if format_version > 0 {
        return Err(anyhow!("Ticket: V1 header not yet supported!"));
    }

    // decrypt title key
    let mut iv = title_id.to_be_bytes().to_vec();
    iv.resize(0x10, 0);
    aes_128_cbc_decrypt_inplace(&mut title_key, &iv, &COMMON_KEY);

    Ok(TicketMetadata {
        title_key,
        ticket_id,
        console_id,
        title_id,
        title_version,
        permitted_titles_mask,
        permit_mask,
        title_export_allowed,
    })
}

/// Parse the title metadata
///
/// # Errors
/// Will error if the `src` is not large enough
/// Will error if there is incorrect data
fn parse_tmd<'a>(src: &'a [u8], position: &mut usize) -> Result<TitleMetadata<'a>, Error> {
    let signature_type = read_u32_at::<BigEndian>(src, position)?;
    test(&signature_type, &0x10001)?;
    // skip signature, padding and issuer
    // TODO: Verify signature
    *position = position.checked_add(0x17C).expect("Overflow occurred!");
    let version = read_u8_at(src, position)?;
    test(&version, &0)?;
    let ca_crl_version = read_u8_at(src, position)?;
    let signer_crl_version = read_u8_at(src, position)?;
    let iw = read_u8_at(src, position)?;
    let is_vwii = if iw == 1 {
        true
    } else if iw == 0 {
        false
    } else {
        return Err(anyhow!("Is VWii is not a boolean: {iw:x}"));
    };
    let system_version = read_u64_at::<BigEndian>(src, position)?;
    let title_id = read_u64_at::<BigEndian>(src, position)?;
    let title_type = TitleType::try_from(read_u32_at::<BigEndian>(src, position)?)?;
    let group_id = read_u16_at::<BigEndian>(src, position)?;
    let unk1 = read_u16_at::<BigEndian>(src, position)?;
    test(&unk1, &0x0)?;
    let region = Region::try_from(read_u16_at::<BigEndian>(src, position)?)?;
    let ratings: &[u8; 0x10] = read_slice_at(src, position)?;
    let reserved1: &[u8; 0xC] = read_slice_at(src, position)?;
    test(reserved1, &[0; 0xC])?;
    let ipc_mask: &[u8; 0xC] = read_slice_at(src, position)?;
    let reserved2: &[u8; 0x12] = read_slice_at(src, position)?;
    test(reserved2, &[0; 0x12])?;
    let access_rights = AccessRights::try_from(read_u32_at::<BigEndian>(src, position)?)?;
    let title_version = read_u16_at::<BigEndian>(src, position)?;
    let number_of_contents = read_u16_at::<BigEndian>(src, position)?;
    let boot_index = read_u16_at::<BigEndian>(src, position)?;
    let minor_version = read_u16_at::<BigEndian>(src, position)?;
    test(&minor_version, &0x0)?;

    let mut contents = Vec::with_capacity(number_of_contents.into());

    for _ in 0..number_of_contents {
        let content_id = read_u32_at::<BigEndian>(src, position)?;
        let index = read_u16_at::<BigEndian>(src, position)?;
        let content_type = ContentType::try_from(read_u16_at::<BigEndian>(src, position)?)?;
        let size = read_u64_at::<BigEndian>(src, position)?;
        let sha1_hash: &[u8; 0x14] = read_slice_at(src, position)?;

        contents.push(ContentMetadata {
            content_id,
            content_type,
            index,
            size,
            sha1_hash,
        });
    }

    Ok(TitleMetadata {
        ca_crl_version,
        signer_crl_version,
        is_vwii,
        system_version,
        title_id,
        title_type,
        group_id,
        region,
        ratings,
        ipc_mask,
        access_rights,
        title_version,
        boot_index,
        contents,
    })
}

/// Parse the content part of the WAD archive
///
/// # Errors
/// Will error if the various contents are too large
fn parse_content<'a>(
    src: &'a [u8],
    position: &mut usize,
    ticket: &TicketMetadata,
    title: &TitleMetadata<'a>,
) -> Result<Vec<Content<'a>>, Error> {
    let mut contents = Vec::with_capacity(title.contents.len());
    let key = ticket.title_key;

    for metadata in &title.contents {
        let size = usize::try_from(metadata.size)?;
        let mut iv = [0; 0x10];
        iv[..2].copy_from_slice(&metadata.index.to_be_bytes());
        let start = *position;
        let end = start.checked_add(size).expect("Overflow occurred!");
        let data = &src[start..end];
        let new_content = Content {
            data,
            key,
            iv,
            metadata: *metadata,
        };
        *position = round_to_boundary(end);
        contents.push(new_content);
    }

    Ok(contents)
}

/// Round address to the next boundary
///
/// # Panics
/// Will panic if the rounding would overflow
fn round_to_boundary(n: usize) -> usize {
    n.checked_add(0x3F)
        .map(|n| n & (!0x3F))
        .expect("Overflow occurred!")
}

#[cfg(test)]
mod tests {
    use super::round_to_boundary;

    #[test]
    fn test_rounding() {
        assert_eq!(round_to_boundary(0x0), 0x0);
        assert_eq!(round_to_boundary(0x1), 0x40);
        assert_eq!(round_to_boundary(0x40), 0x40);
        assert_eq!(round_to_boundary(0x41), 0x80);
        assert_eq!(round_to_boundary(0xA00), 0xA00);
        assert_eq!(round_to_boundary(0x2A4), 0x2C0);
        assert_eq!(round_to_boundary(576), 576);
    }
}
