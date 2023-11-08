//! Contains the types that describe the usefull information in this filetype

use anyhow::anyhow;
use stable_deref_trait::StableDeref;
use yoke::{Yoke, Yokeable};

/// Owned version of the decoded U8 archive that can be easily moved around
pub struct U8ArchiveOwned<C: StableDeref> {
    /// The yoke is used to store the backing with the reference
    yoke: Yoke<U8Archive<'static>, C>,
}

impl<C: StableDeref> From<Yoke<U8Archive<'static>, C>> for U8ArchiveOwned<C> {
    fn from(yoke: Yoke<U8Archive<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> U8ArchiveOwned<C> {
    /// Get a reference to all files in this archive
    pub fn files(&'a self) -> &[Node<'a>] {
        &self.yoke.get().files
    }
}

/// The decoded U8 archive
#[derive(Yokeable)]
pub struct U8Archive<'a> {
    /// The files
    pub files: Vec<Node<'a>>,
}

/// The type of a U8 node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum NodeType {
    /// File
    File = 0,
    /// Directory
    Directory = 1,
}

impl TryFrom<u8> for NodeType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::File),
            1 => Ok(Self::Directory),
            _ => Err(anyhow!("Unknown node type! {value}")),
        }
    }
}

impl From<NodeType> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: NodeType) -> Self {
        value as Self
    }
}

/// A file in a U8 archive
pub struct Node<'a> {
    /// The path components
    pub path: Vec<&'a str>,
    /// The filename
    pub name: &'a str,
    /// The data
    pub data: &'a [u8],
}

/// A yet to be parsed node
#[derive(Clone, Copy)]
pub(crate) struct UnparsedNode {
    /// Typ of this node
    pub node_type: NodeType,
    /// Offset to the name of the file/directory
    pub name_offset: usize,
    /// Offset to the file data, ignored if a directory
    pub data_offset: usize,
    /// The size of the data if a file, the max index if a directory
    pub size: usize,
}
