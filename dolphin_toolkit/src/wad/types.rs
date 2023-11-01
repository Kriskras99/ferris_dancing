//! Contains the types that describe the usefull information in this filetype

use aes::Aes128;
use anyhow::{anyhow, Error};
use cipher::{block_padding::NoPadding, generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use stable_deref_trait::StableDeref;
use yoke::{Yoke, Yokeable};

/// Describes which variant of WAD this is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum WadType {
    /// Used for boot2
    Bootable = 0x4962,
    /// Used on the update partition of Wii discs
    Installable = 0x4973,
    /// Used for data stored on the SD card
    Backup = 0x426b,
}

impl TryFrom<u16> for WadType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x4962 => Ok(Self::Bootable),
            0x4973 => Ok(Self::Installable),
            0x426b => Ok(Self::Backup),
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
    #[allow(clippy::as_conversions)]
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
#[derive(Clone, Copy)]
pub struct ContentMetadata<'a> {
    /// Unique ID
    pub content_id: u32,
    /// See [`ContentType`]
    pub content_type: ContentType,
    /// Unique index
    pub index: u16,
    /// Size of data (excluding this header)
    pub size: u64,
    /// SHA1 hash of the decrypted data (excluding this header)
    pub sha1_hash: &'a [u8; 0x14],
}

/// Content with it's data, crypto, and metadata
pub struct Content<'a> {
    /// The encrypted data
    pub data: &'a [u8],
    /// Decryption key
    pub key: [u8; 0x10],
    /// Decryption IV
    pub iv: [u8; 0x10],
    /// The header
    pub metadata: ContentMetadata<'a>,
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
        Ok(decryptor.decrypt_padded_vec_mut::<NoPadding>(self.data)?)
    }
}

/// Header of the WAD itself
pub struct TitleMetadata<'a> {
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
    pub ratings: &'a [u8], // TODO: Actually parse this
    /// Allowed IPC calls, format unknown
    pub ipc_mask: &'a [u8], // TODO: Actually parse this
    /// See [`AccessRights`]
    pub access_rights: AccessRights,
    /// Version of the title
    pub title_version: u16,
    /// Content to index to start
    pub boot_index: u16,
    /// All contents
    pub contents: Vec<ContentMetadata<'a>>,
}

/// Ticket metadata
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
pub struct InstallableArchive<'a> {
    /// See [`WadType`]
    pub wad_type: WadType,
    /// See [`TicketMetadata`]
    pub ticket_metadata: TicketMetadata,
    /// See [`TitleMetadata`]
    pub title_metadata: TitleMetadata<'a>,
    /// All content in this WAD
    pub content: Vec<Content<'a>>,
}

/// Represents a backup WAD
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
    pub title_metadata: TitleMetadata<'a>,
    /// All content in this WAD
    pub content: Vec<Content<'a>>,
}

/// The decoded WAD archive
#[derive(Yokeable)]
pub enum WadArchive<'a> {
    /// See [`InstallableArchive`]
    Installable(InstallableArchive<'a>),
    /// See [`BackupArchive`]
    Backup(BackupArchive<'a>),
}

/// Owned version of the decoded WAD archive that can be easily moved around
pub struct WadArchiveOwned<C: StableDeref> {
    /// The yoke is used to store the backing with the reference
    yoke: Yoke<WadArchive<'static>, C>,
}

impl<C: StableDeref> From<Yoke<WadArchive<'static>, C>> for WadArchiveOwned<C> {
    fn from(yoke: Yoke<WadArchive<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> WadArchiveOwned<C> {
    /// Get the internal WAD as an installable archive
    ///
    /// # Errors
    /// Returns an error if the WAD is a backup archive
    pub fn as_installable(&'a self) -> Result<&'a InstallableArchive<'a>, Error> {
        if let WadArchive::Installable(installable) = self.yoke.get() {
            Ok(installable)
        } else {
            Err(anyhow!("Not a installable archive!"))
        }
    }

    /// Get the internal WAD as a backup archive
    ///
    /// # Errors
    /// Returns an error if the WAD is an installable archive
    pub fn as_backup(&'a self) -> Result<&'a BackupArchive<'a>, Error> {
        if let WadArchive::Backup(backup) = self.yoke.get() {
            Ok(backup)
        } else {
            Err(anyhow!("Not a backup archive!"))
        }
    }

    /// Get the WAD archive
    pub fn archive(&'a self) -> &'a WadArchive<'a> {
        self.yoke.get()
    }
}

/// MAGIC for bootable WAD
pub const MAGIC_IB: [u8; 6] = [0x0, 0x0, 0x0, 0x20, 0x49, 0x62];
/// MAGIC for installable WAD
pub const MAGIC_IS: [u8; 6] = [0x0, 0x0, 0x0, 0x20, 0x49, 0x73];
/// MAGIC for backup WAD
pub const MAGIC_BK: [u8; 6] = [0x0, 0x0, 0x0, 0x70, 0x42, 0x6b];
