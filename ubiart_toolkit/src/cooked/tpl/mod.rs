mod binary;
mod json;
pub mod types;

#[cfg(feature = "full_json_types")]
pub mod extra_types;

use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
pub use json::{create, create_vec, create_vec_with_capacity_hint};
use ubiart_toolkit_shared_types::errors::ParserError;

use crate::{
    cooked::tpl::types::Actor,
    utils::{Game, UniqueGameId},
};

/// Parse a .tpl.ckd file
pub fn parse(data: &[u8], ugi: UniqueGameId, lax: bool) -> Result<Actor<'_>, ParserError> {
    match ugi.game {
        game if game >= Game::JustDance2016 => {
            let actor = crate::cooked::json::parse(data, lax)?;
            Ok(actor)
        }
        Game::JustDance2015 => {
            let mut position = 0;
            let actor = Actor::deserialize_at_with(data, &mut position, ugi)?;
            #[cfg(test)]
            assert_eq!(position, data.len() as u64);
            Ok(actor)
        }
        _ => todo!("{ugi}"),
    }
}
