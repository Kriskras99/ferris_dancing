//! Contains the parser implementation

use anyhow::Error;
use byteorder::BigEndian;
use dotstar_toolkit_utils::{
    bytes::{read_null_terminated_string_at, read_slice_at, read_u24_at, read_u32_at, read_u8_at},
    testing::test,
};

use super::types::{Node, NodeType, U8Archive, UnparsedNode};

/// Check if these bytes match the magic for a U8 archive
#[must_use]
pub fn can_parse(first_bytes: &[u8; 0x8]) -> bool {
    first_bytes == &[0x2D, 0x38, 0x55, 0xAA, 0x0, 0x0, 0x0, 0x20]
}

/// Parse a Wii U8 file
///
/// # Errors
/// -  the file is not a WAD file or the parser encounters an unexpected value.
///
/// # Panics
/// - Will panic when additions/subtractions over/underflow
pub fn parse(src: &[u8]) -> Result<U8Archive<'_>, Error> {
    let mut position = 0;

    // Check the magic
    let magic = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&magic, &0x55AA_382D)?;

    // Parse the header
    let rootnode_offset = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&rootnode_offset, &0x20)?;
    let _header_size = read_u32_at::<BigEndian>(src, &mut position)?;
    let _data_offset = read_u32_at::<BigEndian>(src, &mut position)?;
    let reserved1: &[u8; 0x10] = read_slice_at(src, &mut position)?;
    test(reserved1, &[0; 0x10])?;

    // Parse the root node
    let node_type = NodeType::try_from(read_u8_at(src, &mut position)?)?;
    // Make sure the root node is a directory
    test(&node_type, &NodeType::Directory)?;
    // Read the rest of the metadata of the root node
    let name_offset = usize::try_from(read_u24_at::<BigEndian>(src, &mut position)?)?;
    let data_offset = usize::try_from(read_u32_at::<BigEndian>(src, &mut position)?)?;
    let size = usize::try_from(read_u32_at::<BigEndian>(src, &mut position)?)?;
    let rootnode = UnparsedNode {
        node_type,
        name_offset,
        data_offset,
        size,
    };

    // Setup the unparsed node list and initialize with the root node
    let number_of_nodes = rootnode.size;
    let mut unparsed_nodes = Vec::with_capacity(number_of_nodes);
    unparsed_nodes.push(rootnode);

    // Read all the node metadata
    for _ in 1..number_of_nodes {
        let node_type = NodeType::try_from(read_u8_at(src, &mut position)?)?;
        let name_offset = usize::try_from(read_u24_at::<BigEndian>(src, &mut position)?)?;
        let data_offset = usize::try_from(read_u32_at::<BigEndian>(src, &mut position)?)?;
        let size = usize::try_from(read_u32_at::<BigEndian>(src, &mut position)?)?;

        let node = UnparsedNode {
            node_type,
            name_offset,
            data_offset,
            size,
        };

        unparsed_nodes.push(node);
    }

    // Name offset in nodes is relative to the start of the string table
    let string_table_offset = position;

    // Prepare a place to save the parsed nodes
    let mut parsed_nodes = Vec::with_capacity(unparsed_nodes[0].size);
    // This keeps track of the current path, as we go parse more directory nodes
    let mut current_path = vec![read_null_terminated_string_at(
        src,
        &mut (string_table_offset
            .checked_add(unparsed_nodes[0].name_offset)
            .expect("Overflow occurred!")),
    )?];
    // This keeps track of the size of directories, if a node index matches the end of this vector it should be popped together with an element of the current path
    let mut max_indexes = vec![unparsed_nodes[0].size];

    for (index, node) in unparsed_nodes.into_iter().enumerate().skip(1) {
        // Account for the fact that the root node has index 0
        let index = index.checked_add(1).expect("Overflow occurred!");
        let node_type = node.node_type;
        let mut name_offset = string_table_offset
            .checked_add(node.name_offset)
            .expect("Overflow occurred!");
        let data_offset = node.data_offset;
        let size = node.size;
        let name = read_null_terminated_string_at(src, &mut name_offset)?;

        match node_type {
            NodeType::File => parsed_nodes.push(Node {
                path: current_path.clone(),
                name,
                data: &src[data_offset..data_offset.checked_add(size).expect("Overflow occurred!")],
            }),
            NodeType::Directory => {
                current_path.push(name);
                max_indexes.push(size);
            }
        }

        // Remove tracked directories if the index matches the end of max_indexes
        while max_indexes.last() == Some(&index) {
            current_path.pop();
        }
    }

    Ok(U8Archive {
        files: parsed_nodes,
    })
}
