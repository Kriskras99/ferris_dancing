//! Contains the parser implementation

use cipher::{generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u16be, u32be, u64be},
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::test,
};

use super::types::{
    AccessRights, Aes128CbcDec, Content, ContentMetadata, ContentType, InstallableArchive, Region,
    TicketMetadata, TitleMetadata, TitleType, WadArchive, WadType,
};
use crate::round_to_boundary;

/// Decrypt the data inplace
fn aes_128_cbc_decrypt_inplace(data: &mut [u8], iv: &[u8], key: &[u8]) {
    let mut decrypter = Aes128CbcDec::new(key.into(), iv.into());
    for chunk in data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        decrypter.decrypt_block_mut(block);
    }
}

impl<'de> BinaryDeserialize<'de> for WadArchive<'de> {
    fn deserialize_at(
        reader: &'de impl ZeroCopyReadAtExt,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let size: u32 = reader.read_at::<u32be>(position)?.into();
        let wad_type = reader.read_at::<WadType>(position)?;
        match (size, wad_type) {
            (0x20, WadType::Bootable | WadType::Installable) => parse_installable(reader, position, wad_type),
            (0x70, WadType::Backup) => Err(ReadError::custom(format!("Backup WAD parsing not yet implemented!"))),
            _ => Err(ReadError::custom(format!("Unknown WAD type found or file is not a WAD file! Metadata size: {size}, WAD type: {wad_type:?}"))),
        }
    }
}

/// Parse the `src` as an installable WAD archive
///
/// # Errors
/// Will error if `src` is not large enough
/// Will error if there is incorrect data
fn parse_installable<'de>(
    reader: &'de impl ZeroCopyReadAtExt,
    position: &mut u64,
    wad_type: WadType,
) -> Result<WadArchive<'de>, ReadError> {
    // parse header
    let version = reader.read_at::<u16be>(position)?.into();
    let cert_chain_size = reader.read_at::<u32be>(position)?;
    let unk1 = reader.read_at::<u32be>(position)?.into();
    let ticket_size = u64::from(reader.read_at::<u32be>(position)?);
    let tmd_size = u64::from(reader.read_at::<u32be>(position)?);
    let _content_size = reader.read_at::<u32be>(position)?;
    let _footer_size = reader.read_at::<u32be>(position)?;

    // verify known constant values`
    test(&version, &0x0u16)?;
    test(&unk1, &0x0u32)?;

    // skip cert chain
    // TODO: implement verification of cert chain
    *position = position
        .checked_add(u64::from(round_to_boundary(cert_chain_size)))
        .unwrap();

    // parse ticket data
    let ticket_start = *position;
    let ticket_metadata = parse_ticket(reader, position)?;
    *position = round_to_boundary_u64(*position);
    let ticket_end = *position;
    test(&ticket_end.checked_sub(ticket_start), &Some(ticket_size))?;

    // parse title_metadata
    let tmd_start = *position;
    let title_metadata = parse_tmd(reader, position)?;
    *position = round_to_boundary_u64(*position);
    let tmd_end = *position;
    test(&tmd_end.checked_sub(tmd_start), &Some(tmd_size))?;

    let content = parse_content(reader, position, &ticket_metadata, &title_metadata)?;

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
fn parse_ticket(
    reader: &impl ZeroCopyReadAtExt,
    position: &mut u64,
) -> Result<TicketMetadata, ReadError> {
    // skip signature, padding, issuer and ECDH data
    // TODO: Verify signature
    *position = position.checked_add(0x1BC).unwrap();
    // Read the metadata
    let format_version = reader.read_at::<u8>(position)?;
    let unk1 = reader.read_at::<u16be>(position)?.into();
    test(&unk1, &0x0)?;
    let mut title_key: [u8; 0x10] = reader.read_fixed_slice_at(position)?;
    let unk2 = reader.read_at::<u8>(position)?; // TODO: Figure out this value
    test(&unk2, &0x0)?;
    let ticket_id = reader.read_at::<u64be>(position)?.into();
    let console_id = reader.read_at::<u32be>(position)?.into();
    let title_id: u64 = reader.read_at::<u64be>(position)?.into();
    let unk3 = reader.read_at::<u16be>(position)?.into();
    test(&unk3, &0xFFFF)?;
    let title_version = reader.read_at::<u16be>(position)?.into();
    let permitted_titles_mask = reader.read_at::<u64be>(position)?.into();
    let permit_mask = reader.read_at::<u64be>(position)?.into();
    let tea = reader.read_at::<u8>(position)?;
    let title_export_allowed = if tea == 1 {
        true
    } else if tea == 0 {
        false
    } else {
        return Err(ReadError::custom(format!(
            "Title export allowed is not a boolean: {tea:x}"
        )));
    };
    let common_key_index = reader.read_at::<u8>(position)?;
    test(&common_key_index, &0x0)?;
    // Skip remainder of the header
    // TODO: Parse this?
    *position = position.checked_add(0xB2).unwrap();

    if format_version > 0 {
        return Err(ReadError::custom(format!(
            "Ticket: V1 header not yet supported!"
        )));
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
fn parse_tmd(
    reader: &impl ZeroCopyReadAtExt,
    position: &mut u64,
) -> Result<TitleMetadata, ReadError> {
    let signature_type = reader.read_at::<u32be>(position)?.into();
    test(&signature_type, &0x10001u32)?;
    // skip signature, padding and issuer
    // TODO: Verify signature
    *position = position.checked_add(0x17C).unwrap();
    let version = reader.read_at::<u8>(position)?;
    test(&version, &0)?;
    let ca_crl_version = reader.read_at::<u8>(position)?;
    let signer_crl_version = reader.read_at::<u8>(position)?;
    let iw = reader.read_at::<u8>(position)?;
    let is_vwii = if iw == 1 {
        true
    } else if iw == 0 {
        false
    } else {
        return Err(ReadError::custom(format!(
            "Is VWii is not a boolean: {iw:x}"
        )));
    };
    let system_version = reader.read_at::<u64be>(position)?.into();
    let title_id = reader.read_at::<u64be>(position)?.into();
    let title_type = reader.read_at::<TitleType>(position)?;
    let group_id = reader.read_at::<u16be>(position)?.into();
    let unk1 = reader.read_at::<u16be>(position)?.into();
    test(&unk1, &0x0)?;
    let region = reader.read_at::<Region>(position)?;
    let ratings: [u8; 0x10] = reader.read_fixed_slice_at(position)?;
    let reserved1: [u8; 0xC] = reader.read_fixed_slice_at(position)?;
    test(&reserved1, &[0; 0xC])?;
    let ipc_mask: [u8; 0xC] = reader.read_fixed_slice_at(position)?;
    let reserved2: [u8; 0x12] = reader.read_fixed_slice_at(position)?;
    test(&reserved2, &[0; 0x12])?;
    let access_rights = reader.read_at::<AccessRights>(position)?;
    let title_version = reader.read_at::<u16be>(position)?.into();
    let number_of_contents: u16 = reader.read_at::<u16be>(position)?.into();
    let boot_index = reader.read_at::<u16be>(position)?.into();
    let minor_version = reader.read_at::<u16be>(position)?.into();
    test(&minor_version, &0x0)?;

    let mut contents = Vec::with_capacity(number_of_contents.into());

    for _ in 0..number_of_contents {
        let content_id = reader.read_at::<u32be>(position)?.into();
        let index = reader.read_at::<u16be>(position)?.into();
        let content_type = reader.read_at::<ContentType>(position)?;
        let size = reader.read_at::<u64be>(position)?.into();
        let sha1_hash: [u8; 0x14] = reader.read_fixed_slice_at(position)?;

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
fn parse_content<'de>(
    reader: &'de impl ZeroCopyReadAtExt,
    position: &mut u64,
    ticket: &TicketMetadata,
    title: &TitleMetadata,
) -> Result<Vec<Content<'de>>, ReadError> {
    let mut contents = Vec::with_capacity(title.contents.len());
    let key = ticket.title_key;

    for metadata in &title.contents {
        let size = usize::try_from(metadata.size)?;
        let mut iv = [0; 0x10];
        iv[..2].copy_from_slice(&metadata.index.to_be_bytes());
        let data = reader.read_slice_at(position, size)?;
        let new_content = Content {
            data,
            key,
            iv,
            metadata: *metadata,
        };
        *position = round_to_boundary_u64(*position);
        contents.push(new_content);
    }

    Ok(contents)
}

/// Round address to the next boundary
///
/// # Panics
/// Will panic if the rounding would overflow
fn round_to_boundary_u64(n: u64) -> u64 {
    n.checked_add(0x3F)
        .map(|n| n & (!0x3F))
        .expect("Overflow occurred!")
}

#[cfg(test)]
mod tests {
    use super::round_to_boundary_u64;

    #[test]
    fn test_rounding() {
        assert_eq!(round_to_boundary_u64(0x0), 0x0);
        assert_eq!(round_to_boundary_u64(0x1), 0x40);
        assert_eq!(round_to_boundary_u64(0x40), 0x40);
        assert_eq!(round_to_boundary_u64(0x41), 0x80);
        assert_eq!(round_to_boundary_u64(0xA00), 0xA00);
        assert_eq!(round_to_boundary_u64(0x2A4), 0x2C0);
        assert_eq!(round_to_boundary_u64(576), 576);
    }
}
