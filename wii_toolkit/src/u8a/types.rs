//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;
use std::collections::{hash_map::Entry, HashMap};

use anyhow::anyhow;
use dotstar_toolkit_utils::bytes_newer4::primitives::{u24be, u32be};
use dotstar_toolkit_utils::bytes_newer4::read::ZeroCopyReadAtExt;
use dotstar_toolkit_utils::{
    bytes_newer4::{
        read::{BinaryDeserialize, ReadError},
        write::{BinarySerialize, WriteError, ZeroCopyWriteAt},
    },
    testing::test,
};

use crate::round_to_boundary;

/// The decoded U8 archive
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

/// A file in a U8 archive
pub struct NewNode<'a> {
    /// The filename
    pub name: Cow<'a, str>,
    /// The data
    pub data: Cow<'a, [u8]>,
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

/// A unparsed node
enum NewUnparsedNode {
    /// A unparsed directory node
    Directory(NewUnparsedDirectory),
    /// A unparsed file node
    File(NewUnparsedFile),
}

impl NewUnparsedNode {
    const MAGIC_FILE: u8 = 0x0;
    const MAGIC_DIRECTORY: u8 = 0x1;
}

impl<'de> BinaryDeserialize<'de> for NewUnparsedNode {
    fn deserialize_at(
        reader: &'de impl ZeroCopyReadAtExt,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let node_type = reader.read_at::<u8>(position)?;
            let name_offset = reader.read_at::<u24be>(position)?.into();
            let data_offset = reader.read_at::<u32be>(position)?.into();
            let size = reader.read_at::<u32be>(position)?.into();

            match node_type {
                Self::MAGIC_FILE => Ok(NewUnparsedNode::File(NewUnparsedFile {
                    name_offset,
                    data_offset,
                    size,
                })),
                Self::MAGIC_DIRECTORY => Ok(NewUnparsedNode::Directory(NewUnparsedDirectory {
                    name_offset,
                    last_included_node_index: size,
                })),
                _ => Err(ReadError::custom("Node magic is unknown!".into())),
            }?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

/// A unparsed directory node
pub struct NewUnparsedDirectory {
    /// Offset to the name from the start of the string table
    pub name_offset: u24be,
    /// The index of the last node included in this directory
    pub last_included_node_index: u32be,
}

impl BinarySerialize for NewUnparsedDirectory {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_at(position, &NewUnparsedNode::MAGIC_DIRECTORY)?;
        writer.write_at(position, &self.name_offset)?;
        writer.write_at(position, &u32be::from(0))?;
        writer.write_at(position, &self.last_included_node_index)?;
        Ok(())
    }
}

/// A unparsed file node
pub struct NewUnparsedFile {
    /// Offset to the name from the start of the string table
    pub name_offset: u24be,
    /// Offset to the data from the start of the file
    pub data_offset: u32be,
    /// The size of the data
    pub size: u32be,
}
impl BinarySerialize for NewUnparsedFile {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_at(position, &NewUnparsedNode::MAGIC_FILE)?;
        writer.write_at(position, &self.name_offset)?;
        writer.write_at(position, &self.data_offset)?;
        writer.write_at(position, &self.size)?;
        Ok(())
    }
}

/// The contents of a U8 archive
pub struct NewU8Archive<'a> {
    /// The complete file tree of the archive
    pub file_tree: FileTree<'a>,
}

impl NewU8Archive<'_> {
    const MAGIC: u32be = u32be::new(0x55AA382D);
    const ROOTNODE_OFFSET: u32be = u32be::new(0x20);
    const PADDING: [u8; 16] = [0; 16];
}

impl<'de> BinaryDeserialize<'de> for NewU8Archive<'de> {
    fn deserialize_at(
        reader: &'de impl ZeroCopyReadAtExt,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let begin_position = *position;
            // Check the magic value
            let magic = reader.read_at(position)?;
            test(&magic, &Self::MAGIC)?;
            // Check the rootnode offset
            let rootnode_offset = reader.read_at(position)?;
            test(&rootnode_offset, &Self::ROOTNODE_OFFSET)?;
            // Check that the data offset equals the header size plus the rootnode offset
            let header_size = reader.read_at::<u32be>(position)?;
            let data_offset = reader.read_at::<u32be>(position)?;
            test(
                &round_to_boundary(Self::ROOTNODE_OFFSET.checked_add(header_size).unwrap()),
                &data_offset,
            )?;
            // Check the padding
            let padding = reader.read_fixed_slice_at::<16>(position)?;
            test(&padding, &Self::PADDING)?;

            let rootnode = reader.read_at(position)?;
            let NewUnparsedNode::Directory(rootnode) = rootnode else {
                Err(ReadError::custom("Rootnode is not a directory!".into()))?
            };

            let total_nodes = u32::from(rootnode.last_included_node_index);
            let string_table_offset =
                u64::from(u32::from(Self::ROOTNODE_OFFSET) + total_nodes * 12) + begin_position;

            let file_tree = FileTree {
                directories: HashMap::new(),
                files: HashMap::new(),
            };

            let mut trees = vec![(Cow::from(""), file_tree)];
            let mut indexes = vec![total_nodes];

            for index in 2..=total_nodes {
                let node = reader.read_at(position)?;
                match node {
                    NewUnparsedNode::Directory(node) => {
                        let mut string_offset = u64::from(node.name_offset) + string_table_offset;
                        let name = reader.read_null_terminated_string_at(&mut string_offset)?;
                        let tree = FileTree {
                            directories: HashMap::new(),
                            files: HashMap::new(),
                        };
                        trees.push((name, tree));
                        indexes.push(u32::from(node.last_included_node_index));
                    }
                    NewUnparsedNode::File(node) => {
                        let mut data_offset = u64::from(node.data_offset) + begin_position;
                        let size = usize::try_from(node.size).unwrap();
                        let mut string_offset = u64::from(node.name_offset) + string_table_offset;
                        let name = reader.read_null_terminated_string_at(&mut string_offset)?;
                        let data = reader.read_slice_at(&mut data_offset, size)?;

                        trees
                            .last_mut()
                            .unwrap_or_else(|| unreachable!())
                            .1
                            .files
                            .insert(name, data);
                        while indexes.last() != indexes.first()
                            && index == *indexes.last().unwrap_or_else(|| unreachable!())
                        {
                            let tree = trees.pop().unwrap_or_else(|| unreachable!());
                            trees
                                .last_mut()
                                .unwrap_or_else(|| unreachable!())
                                .1
                                .directories
                                .insert(tree.0, tree.1);
                            assert!(!indexes.pop().is_none());
                        }
                    }
                }
            }

            NewU8Archive {
                file_tree: trees.pop().unwrap_or_else(|| unreachable!()).1,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl BinarySerialize for NewU8Archive<'_> {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        let count = self.file_tree.count();
        let (string_table_size, string_table) = self.file_tree.string_table();

        // Write the magic value
        writer.write_at(position, &Self::MAGIC)?;
        // Write the rootnode offset
        writer.write_at(position, &Self::ROOTNODE_OFFSET)?;
        // Calculate and write the header size and data offset
        let header_size = u32be::from(count * 12 + u32::try_from(string_table_size).expect("UGH"));
        let data_offset = round_to_boundary(Self::ROOTNODE_OFFSET.checked_add(header_size).unwrap());
        writer.write_at(position, &header_size)?;
        writer.write_at(position, &data_offset)?;
        // Write the padding
        writer.write_at(position, &Self::PADDING)?;

        fn write_filetree_rec(
            writer: &mut (impl ZeroCopyWriteAt + ?Sized),
            position: &mut u64,
            data_offset: &mut u32,
            file_tree: &FileTree,
            string_table: &HashMap<Cow<'_, str>, u32>,
            current_idx: &mut u32, // start with one
            name: &str,
        ) -> Result<(), WriteError> {
            // Create and write this directory node
            let count = file_tree.count();
            let node = NewUnparsedDirectory {
                name_offset: u24be::try_from(*string_table.get(name).unwrap()).unwrap(),
                last_included_node_index: u32be::from(*current_idx + count),
            };
            writer.write_at(position, &node)?;
            *current_idx += 1;
            // Write all files directly in this directory
            for (filename, data) in &file_tree.files {
                let size = u32::try_from(data.len()).unwrap();
                let node = NewUnparsedFile {
                    name_offset: u24be::try_from(*string_table
                        .get(filename.as_ref())
                        .unwrap_or_else(|| unreachable!())).unwrap(),
                    data_offset: u32be::from(*data_offset),
                    size: u32be::try_from(data.len()).unwrap(),
                };
                // Write the data
                let mut data_offset_u64 = u64::from(*data_offset);
                writer.write_slice_at(&mut data_offset_u64, data.as_ref())?;
                *data_offset += size;
                // Write the file node
                writer.write_at(position, &node)?;
                *current_idx += 1;
            }

            // Write all subdirectories and files
            for (name, tree) in &file_tree.directories {
                write_filetree_rec(
                    writer,
                    position,
                    data_offset,
                    tree,
                    string_table,
                    current_idx,
                    name.as_ref(),
                )?;
            }

            Ok(())
        }

        let mut current_idx = 1;
        write_filetree_rec(
            writer,
            position,
            &mut u32::from(data_offset),
            &self.file_tree,
            &string_table,
            &mut current_idx,
            "",
        )?;

        Ok(())
    }
}

/// A recursive file tree
pub struct FileTree<'a> {
    /// The directories at this level
    pub directories: HashMap<Cow<'a, str>, FileTree<'a>>,
    /// The files at this level
    pub files: HashMap<Cow<'a, str>, Cow<'a, [u8]>>,
}

impl FileTree<'_> {
    fn count(&self) -> u32 {
        let n = self.directories.len() + self.files.len();
        let mut n = u32::try_from(n).expect("Too many nodes!");
        for directory in self.directories.values() {
            n = n.checked_add(directory.count()).expect("Too many nodes");
        }
        n
    }
}

impl<'a> FileTree<'a> {
    fn string_table(&self) -> (u32, HashMap<Cow<'a, str>, u32>) {
        let mut string_map = HashMap::new();
        let mut offset = 1;
        self.string_table_rec(&mut string_map, &mut offset);
        (offset, string_map)
    }

    fn string_table_rec(&self, string_map: &mut HashMap<Cow<'a, str>, u32>, offset: &mut u32) {
        for file in &self.files {
            if let Entry::Vacant(entry) = string_map.entry(file.0.clone()) {
                let length = entry.key().len();
                entry.insert(*offset);
                offset.checked_add(length.try_into().unwrap()).unwrap().checked_add(1).unwrap();
            }
        }
        for directory in &self.directories {
            if let Entry::Vacant(entry) = string_map.entry(directory.0.clone()) {
                let length = entry.key().len();
                entry.insert(*offset);
                offset.checked_add(length.try_into().unwrap()).unwrap().checked_add(1).unwrap();
            }
        }
    }
}
