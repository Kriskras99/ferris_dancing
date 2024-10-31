use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit_shared_types::errors::ParserError;

use super::Root;
use crate::utils::{Game, UniqueGameId};

/// Parse a isc file
pub fn parse(data: &[u8], ugi: UniqueGameId) -> Result<Root<'_>, ParserError> {
    let root = match ugi.game {
        game if game >= Game::JustDance2016 => {
            let string = std::str::from_utf8(data)?;
            let root: Root = quick_xml::de::from_str(string)?;
            root
        }
        Game::JustDance2015 => {
            let mut position = 0;
            let root = Root::deserialize_at_with(data, &mut position, ugi)?;
            #[cfg(test)]
            assert_eq!(position, data.len() as u64);
            root
        }
        _ => todo!(),
    };
    Ok(root)
}
