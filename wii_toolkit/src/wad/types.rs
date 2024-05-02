//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use aes::Aes128;
use anyhow::{anyhow, Error};
use cipher::{block_padding::NoPadding, generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use dotstar_toolkit_utils::bytes::{
    primitives::{u16be, u32be},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};

/// Describes which variant of WAD this is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum WadType {
    /// Used for boot2
    Bootable = 0x4962,
    /// Used on the update partition of Wii discs
    Installable = 0x4973,
    /// Used for data stored on the SD card
    Backup = 0x426B,
}

impl BinaryDeserialize<'_> for WadType {
    fn deserialize_at(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u16 = reader.read_at::<u16be>(position)?.into();
        match value {
            0x4962 => Ok(Self::Bootable),
            0x4973 => Ok(Self::Installable),
            0x426B => Ok(Self::Backup),
            _ => Err(ReadError::custom(format!(
                "Unknown value for WAD type: {value:x}"
            ))),
        }
    }
}

impl TryFrom<u16> for WadType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x4962 => Ok(Self::Bootable),
            0x4973 => Ok(Self::Installable),
            0x426B => Ok(Self::Backup),
            _ => Err(anyhow!("Unknown value for WAD type: {value:x}")),
        }
    }
}

/// The type of content
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum ContentType {
    /// Game/Program
    Normal = 0x0001,
    /// DLC
    DLC = 0x4001,
    /// Shared between various apps
    Shared = 0x8001,
}

impl BinaryDeserialize<'_> for ContentType {
    fn deserialize_at(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u16 = reader.read_at::<u16be>(position)?.into();
        match value {
            0x0001 => Ok(Self::Normal),
            0x4001 => Ok(Self::DLC),
            0x8001 => Ok(Self::Shared),
            _ => Err(ReadError::custom(format!(
                "Unknown value for content type: {value:x}"
            ))),
        }
    }
}

impl TryFrom<u16> for ContentType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(Self::Normal),
            0x4001 => Ok(Self::DLC),
            0x8001 => Ok(Self::Shared),
            _ => Err(anyhow!("Unknown value for content type: {value:x}")),
        }
    }
}

impl From<ContentType> for u16 {
    #[allow(
        clippy::as_conversions,
        reason = "ContentType is repr(u16) so this is safe"
    )]
    fn from(value: ContentType) -> Self {
        value as Self
    }
}

/// Unknown?
#[derive(Debug)]
#[repr(u32)]
pub enum TitleType {
    /// Unknown?
    Something = 0x19,
}

impl BinaryDeserialize<'_> for TitleType {
    fn deserialize_at(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u32 = reader.read_at::<u32be>(position)?.into();
        match value {
            0x19 => Ok(Self::Something),
            _ => Err(ReadError::custom(format!(
                "Unknown value for title type: {value:x}"
            ))),
        }
    }
}

impl TryFrom<u32> for TitleType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x19 => Ok(Self::Something),
            _ => Err(anyhow!("Unknown value for title type: {value:x}")),
        }
    }
}

/// The region lock for the entire WAD
#[derive(Debug)]
#[repr(u16)]
pub enum Region {
    /// Japan
    Japan = 0x0,
    /// North America
    UnitedStates = 0x1,
    /// Europe
    Europe = 0x2,
    /// No region lock
    RegionFree = 0x3,
    /// South Korea
    Korea = 0x4,
}

impl BinaryDeserialize<'_> for Region {
    fn deserialize_at(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u16 = reader.read_at::<u16be>(position)?.into();
        match value {
            0x0 => Ok(Self::Japan),
            0x1 => Ok(Self::UnitedStates),
            0x2 => Ok(Self::Europe),
            0x3 => Ok(Self::RegionFree),
            0x4 => Ok(Self::Korea),
            _ => Err(ReadError::custom(format!(
                "Unknown value for region: {value:x}"
            ))),
        }
    }
}

impl TryFrom<u16> for Region {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Japan),
            0x1 => Ok(Self::UnitedStates),
            0x2 => Ok(Self::Europe),
            0x3 => Ok(Self::RegionFree),
            0x4 => Ok(Self::Korea),
            _ => Err(anyhow!("Unknown value for region: {value:x}")),
        }
    }
}

/// Flags for DVD-video access and full PPC hardware access
#[derive(Debug)]
#[repr(u32)]
pub enum AccessRights {
    /// No special access rights
    None = 0x0,
}

impl BinaryDeserialize<'_> for AccessRights {
    fn deserialize_at(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u16 = reader.read_at::<u16be>(position)?.into();
        match value {
            0x0 => Ok(Self::None),
            _ => Err(ReadError::custom(format!(
                "Unknown value for access rights: {value:x}"
            ))),
        }
    }
}

impl TryFrom<u32> for AccessRights {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::None),
            _ => Err(anyhow!("Unknown value for access rights: {value:x}")),
        }
    }
}

/// Header for content
#[derive(Debug, Clone, Copy)]
pub struct ContentMetadata {
    /// Unique ID
    pub content_id: u32,
    /// See [`ContentType`]
    pub content_type: ContentType,
    /// Unique index
    pub index: u16,
    /// Size of data (excluding this header)
    pub size: u64,
    /// SHA1 hash of the decrypted data (excluding this header)
    pub sha1_hash: [u8; 0x14],
}

/// Content with it's data, crypto, and metadata
#[derive(Debug)]
pub struct Content<'a> {
    /// The encrypted data
    pub data: Cow<'a, [u8]>,
    /// Decryption key
    pub key: [u8; 0x10],
    /// Decryption IV
    pub iv: [u8; 0x10],
    /// The header
    pub metadata: ContentMetadata,
}

/// Convenience alias to make it clear which decryption is being done
pub type Aes128CbcDec = cbc::Decryptor<Aes128>;

impl Content<'_> {
    /// Decrypt the content into a newly allocated `Vec`
    ///
    /// # Errors
    /// Returns an error if the content is malformed
    pub fn decrypt(&self) -> Result<Vec<u8>, Error> {
        let decryptor =
            Aes128CbcDec::new(&GenericArray::from(self.key), &GenericArray::from(self.iv));
        Ok(decryptor.decrypt_padded_vec_mut::<NoPadding>(self.data.as_ref())?)
    }
}

/// Header of the WAD itself
#[derive(Debug)]
pub struct TitleMetadata {
    /// Unknown
    pub ca_crl_version: u8,
    /// Unknown
    pub signer_crl_version: u8,
    /// Is this a vWii title
    pub is_vwii: bool,
    /// Minimum IOS version
    pub system_version: u64,
    /// Title id
    pub title_id: u64,
    /// See [`TitleType`]
    pub title_type: TitleType,
    /// Unknown
    pub group_id: u16,
    /// See [`Region`]
    pub region: Region,
    /// Ratings, format unknown
    pub ratings: [u8; 0x10], // TODO: Actually parse this
    /// Allowed IPC calls, format unknown
    pub ipc_mask: [u8; 0xC], // TODO: Actually parse this
    /// See [`AccessRights`]
    pub access_rights: AccessRights,
    /// Version of the title
    pub title_version: u16,
    /// Content to index to start
    pub boot_index: u16,
    /// All contents
    pub contents: Vec<ContentMetadata>,
}

/// Ticket metadata
#[derive(Debug)]
pub struct TicketMetadata {
    /// Title key (already decrypted)
    pub title_key: [u8; 0x10],
    /// Used as IV for title key decryption of console specific titles
    pub ticket_id: u64,
    /// Unknown
    pub console_id: u32,
    /// IV for content decryption
    pub title_id: u64,
    /// Unknown
    pub title_version: u16,
    /// Unknown
    pub permitted_titles_mask: u64,
    /// Unknown
    pub permit_mask: u64,
    /// Title Export allowed using PRNG key
    pub title_export_allowed: bool,
}

/// Represents an installable/bootable WAD
#[derive(Debug)]
pub struct InstallableArchive<'a> {
    /// See [`WadType`]
    pub wad_type: WadType,
    /// See [`TicketMetadata`]
    pub ticket_metadata: TicketMetadata,
    /// See [`TitleMetadata`]
    pub title_metadata: TitleMetadata,
    /// All content in this WAD
    pub content: Vec<Content<'a>>,
}

/// Represents a backup WAD
#[derive(Debug)]
pub struct BackupArchive<'a> {
    /// See [`WadType`]
    pub wad_type: WadType,
    /// Console id
    pub console_id: u32,
    /// Unknown
    pub included_contents: &'a [u8],
    /// Title id
    pub title_id: u64,
    /// See [`TicketMetadata`]
    pub ticket_metadata: TicketMetadata,
    /// See [`TitleMetadata`]
    pub title_metadata: TitleMetadata,
    /// All content in this WAD
    pub content: Vec<Content<'a>>,
}

/// The decoded WAD archive
#[derive(Debug)]
pub enum WadArchive<'a> {
    /// See [`InstallableArchive`]
    Installable(InstallableArchive<'a>),
    /// See [`BackupArchive`]
    Backup(BackupArchive<'a>),
}

/// MAGIC for bootable WAD
pub const MAGIC_IB: [u8; 6] = [0x0, 0x0, 0x0, 0x20, 0x49, 0x62];
/// MAGIC for installable WAD
pub const MAGIC_IS: [u8; 6] = [0x0, 0x0, 0x0, 0x20, 0x49, 0x73];
/// MAGIC for backup WAD
pub const MAGIC_BK: [u8; 6] = [0x0, 0x0, 0x0, 0x70, 0x42, 0x6B];
