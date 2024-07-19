//! Contains the types that describe the usefull information in this filetype

use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u24be, u32be},
        read::{BinaryDeserialize, ReadAtExt, ReadError},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    test_eq,
};

use crate::round_to_boundary;

/// A file in a U8 archive
pub struct Node<'a> {
    /// The filename
    pub name: Cow<'a, str>,
    /// The data
    pub data: Cow<'a, [u8]>,
}

/// A unparsed node
#[derive(Debug, Clone, Copy)]
enum UnparsedNode {
    /// A unparsed directory node
    Directory(NewUnparsedDirectory),
    /// A unparsed file node
    File(NewUnparsedFile),
}

impl UnparsedNode {
    /// Magic number for files
    const MAGIC_FILE: u8 = 0x0;
    /// Magic number for directories
    const MAGIC_DIRECTORY: u8 = 0x1;
}

impl<'de> BinaryDeserialize<'de> for UnparsedNode {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let node_type = reader.read_at::<u8>(position)?;
            let name_offset = reader.read_at::<u24be>(position)?;
            let data_offset = reader.read_at::<u32be>(position)?;
            let size = reader.read_at::<u32be>(position)?;

            match node_type {
                Self::MAGIC_FILE => Ok(Self::File(NewUnparsedFile {
                    name_offset,
                    data_offset,
                    size,
                })),
                Self::MAGIC_DIRECTORY => Ok(Self::Directory(NewUnparsedDirectory {
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
#[derive(Debug, Clone, Copy)]
pub struct NewUnparsedDirectory {
    /// Offset to the name from the start of the string table
    pub name_offset: u32,
    /// The index of the last node included in this directory
    pub last_included_node_index: u32,
}

impl BinarySerialize for NewUnparsedDirectory {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u8>(position, UnparsedNode::MAGIC_DIRECTORY)?;
        writer.write_at::<u24be>(position, input.name_offset)?;
        writer.write_at::<u32be>(position, 0u32)?;
        writer.write_at::<u32be>(position, input.last_included_node_index)?;
        Ok(())
    }
}

/// A unparsed file node
#[derive(Debug, Clone, Copy)]
pub struct NewUnparsedFile {
    /// Offset to the name from the start of the string table
    pub name_offset: u32,
    /// Offset to the data from the start of the file
    pub data_offset: u32,
    /// The size of the data
    pub size: u32,
}
impl BinarySerialize for NewUnparsedFile {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u8>(position, UnparsedNode::MAGIC_FILE)?;
        writer.write_at::<u24be>(position, input.name_offset)?;
        writer.write_at::<u32be>(position, input.data_offset)?;
        writer.write_at::<u32be>(position, input.size)?;
        Ok(())
    }
}

/// The contents of a U8 archive
#[derive(Debug)]
pub struct U8Archive<'a> {
    /// The complete file tree of the archive
    pub file_tree: FileTree<'a>,
}

impl U8Archive<'_> {
    /// Magic of .u8 files
    const MAGIC: u32 = 0x55AA_382D;
    /// Offset to the root node
    const ROOTNODE_OFFSET: u32 = 0x20;
    /// Padding between the header and root node
    const PADDING: [u8; 16] = [0; 16];
}

impl<'de> BinaryDeserialize<'de> for U8Archive<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let begin_position = *position;
            // Check the magic value
            let magic = reader.read_at::<u32be>(position)?;
            test_eq!(magic, Self::MAGIC)?;
            // Check the rootnode offset
            let rootnode_offset = reader.read_at::<u32be>(position)?;
            test_eq!(rootnode_offset, Self::ROOTNODE_OFFSET)?;
            // Check that the data offset equals the header size plus the rootnode offset
            let header_size = reader.read_at::<u32be>(position)?;
            let data_offset = reader.read_at::<u32be>(position)?;
            test_eq!(
                round_to_boundary(Self::ROOTNODE_OFFSET.checked_add(header_size).unwrap()),
                data_offset,
            )?;
            // Check the padding
            let padding = reader.read_at::<[u8; 16]>(position)?;
            test_eq!(padding, Self::PADDING)?;

            let rootnode = reader.read_at::<UnparsedNode>(position)?;
            let UnparsedNode::Directory(rootnode) = rootnode else {
                Err(ReadError::custom("Rootnode is not a directory!".into()))?
            };

            let total_nodes = rootnode.last_included_node_index;
            let string_table_offset = total_nodes
                .checked_mul(12)
                .and_then(|c| c.checked_add(Self::ROOTNODE_OFFSET))
                .map(u64::from)
                .and_then(|c| c.checked_add(begin_position))
                .ok_or_else(ReadError::int_under_overflow)?;

            let file_tree = FileTree {
                directories: HashMap::new(),
                files: HashMap::new(),
            };

            let mut trees = vec![(Cow::from(""), file_tree)];
            let mut indexes = vec![total_nodes];

            for index in 2..=total_nodes {
                let node = reader.read_at::<UnparsedNode>(position)?;
                match node {
                    UnparsedNode::Directory(node) => {
                        let mut string_offset = u64::from(node.name_offset)
                            .checked_add(string_table_offset)
                            .ok_or_else(ReadError::int_under_overflow)?;
                        let name = reader.read_null_terminated_string_at(&mut string_offset)?;
                        let tree = FileTree {
                            directories: HashMap::new(),
                            files: HashMap::new(),
                        };
                        trees.push((name, tree));
                        indexes.push(node.last_included_node_index);
                    }
                    UnparsedNode::File(node) => {
                        let mut data_offset = u64::from(node.data_offset)
                            .checked_add(begin_position)
                            .ok_or_else(ReadError::int_under_overflow)?;
                        let size = usize::try_from(node.size).unwrap();
                        let mut string_offset = u64::from(node.name_offset)
                            .checked_add(string_table_offset)
                            .ok_or_else(ReadError::int_under_overflow)?;
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
                            assert!(
                                indexes.pop().is_some(),
                                "There should be something left to pop!"
                            );
                        }
                    }
                }
            }

            U8Archive {
                file_tree: trees.pop().unwrap_or_else(|| unreachable!()).1,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl BinarySerialize for U8Archive<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        let count = input.file_tree.count()?;
        let (string_table_size, string_table) = input.file_tree.string_table()?;

        // Write the magic value
        writer.write_at::<u32be>(position, Self::MAGIC)?;
        // Write the rootnode offset
        writer.write_at::<u32be>(position, Self::ROOTNODE_OFFSET)?;
        // Calculate and write the header size and data offset
        let header_size = count
            .checked_mul(12)
            .and_then(|c| c.checked_add(string_table_size))
            .ok_or_else(WriteError::int_under_overflow)?;
        let mut data_offset = round_to_boundary(
            Self::ROOTNODE_OFFSET
                .checked_add(header_size)
                .ok_or_else(WriteError::int_under_overflow)?,
        );
        writer.write_at::<u32be>(position, header_size)?;
        writer.write_at::<u32be>(position, data_offset)?;
        // Write the padding
        writer.write_at::<[u8; 16]>(position, Self::PADDING)?;

        let mut current_idx = 1;
        write_filetree_rec(
            writer,
            position,
            &mut data_offset,
            &input.file_tree,
            &string_table,
            &mut current_idx,
            "",
        )?;

        Ok(())
    }
}

/// Write the tree metadata recursively
fn write_filetree_rec(
    writer: &mut (impl WriteAt + ?Sized),
    position: &mut u64,
    data_offset: &mut u32,
    file_tree: &FileTree,
    string_table: &HashMap<Cow<'_, str>, u32>,
    current_idx: &mut u32, // start with one
    name: &str,
) -> Result<(), WriteError> {
    // Create and write this directory node
    let count = file_tree.count()?;
    let node = NewUnparsedDirectory {
        name_offset: *string_table.get(name).unwrap_or_else(|| unreachable!()),
        last_included_node_index: current_idx
            .checked_add(count)
            .ok_or_else(WriteError::int_under_overflow)?,
    };
    writer.write_at::<NewUnparsedDirectory>(position, node)?;
    *current_idx = current_idx
        .checked_add(1)
        .ok_or_else(WriteError::int_under_overflow)?;
    // Write all files directly in this directory
    for (filename, data) in &file_tree.files {
        let size = u32::try_from(data.len())?;
        let node = NewUnparsedFile {
            name_offset: *string_table
                .get(filename.as_ref())
                .unwrap_or_else(|| unreachable!()),
            data_offset: *data_offset,
            size,
        };
        // Write the data
        let mut data_offset_u64 = u64::from(*data_offset);
        writer.write_slice_at(&mut data_offset_u64, data.as_ref())?;
        *data_offset = data_offset
            .checked_add(size)
            .ok_or_else(WriteError::int_under_overflow)?;
        // Write the file node
        writer.write_at::<NewUnparsedFile>(position, node)?;
        *current_idx = current_idx
            .checked_add(1)
            .ok_or_else(WriteError::int_under_overflow)?;
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

/// A recursive file tree
#[derive(Debug)]
pub struct FileTree<'a> {
    /// The directories at this level
    pub directories: HashMap<Cow<'a, str>, FileTree<'a>>,
    /// The files at this level
    pub files: HashMap<Cow<'a, str>, Cow<'a, [u8]>>,
}

impl FileTree<'_> {
    /// Count the amount of files and directories in this tree
    fn count(&self) -> Result<u32, WriteError> {
        let n = self
            .directories
            .len()
            .checked_add(self.files.len())
            .ok_or_else(WriteError::int_under_overflow)?;
        let mut n = u32::try_from(n)?;
        for directory in self.directories.values() {
            n = n
                .checked_add(directory.count()?)
                .ok_or_else(WriteError::int_under_overflow)?;
        }
        Ok(n)
    }
}

impl<'a> FileTree<'a> {
    /// Creates a string table from this tree
    fn string_table(&self) -> Result<(u32, HashMap<Cow<'a, str>, u32>), WriteError> {
        let mut string_map = HashMap::new();
        let mut offset = 1;
        self.string_table_rec(&mut string_map, &mut offset)?;
        Ok((offset, string_map))
    }

    /// Recursive part of `string_table`
    fn string_table_rec(
        &self,
        string_map: &mut HashMap<Cow<'a, str>, u32>,
        offset: &mut u32,
    ) -> Result<(), WriteError> {
        for file in &self.files {
            if let Entry::Vacant(entry) = string_map.entry(file.0.clone()) {
                let length = entry.key().len();
                entry.insert(*offset);
                *offset = offset
                    .checked_add(length.try_into()?)
                    .and_then(|i| i.checked_add(1))
                    .ok_or_else(WriteError::int_under_overflow)?;
            }
        }
        for directory in &self.directories {
            if let Entry::Vacant(entry) = string_map.entry(directory.0.clone()) {
                let length = entry.key().len();
                entry.insert(*offset);
                *offset = offset
                    .checked_add(length.try_into()?)
                    .and_then(|i| i.checked_add(1))
                    .ok_or_else(WriteError::int_under_overflow)?;
            }
            directory.1.string_table_rec(string_map, offset)?;
        }
        Ok(())
    }
}
