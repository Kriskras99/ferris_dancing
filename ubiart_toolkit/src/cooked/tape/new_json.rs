use simd_json::Node;
use dotstar_toolkit_utils::bytes::read::{BinaryDeserialize, BinaryDeserializeExt};
use crate::cooked::tape::Tape;
use crate::utils::{Game, UniqueGameId};

pub trait JsonDeserialize<'de> {
    type Ctx: Sized;
    type Output: Sized;

    fn from_tape_with(tape: &[Node<'de>], ctx: Self::Ctx) -> Result<Self::Output, String>;
}

pub trait JsonDeserializeExt<'de>: JsonDeserialize<'de>
where
    Self::Ctx: Default
{
    #[inline]
    fn from_tape(tape: &[Node<'de>]) -> Result<Self::Output, String> {
        Self::from_tape_with(tape, Self::Ctx::default())
    }
}

impl<'de, T: JsonDeserialize<'de>> JsonDeserializeExt<'de> for T where T::Ctx: Default {}

impl<'de> JsonDeserialize<'de> for Tape<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn from_tape_with(tape: &[Node<'de>], ctx: Self::Ctx) -> Result<Self::Output, String> {
        let Node::Object { len, count } = tape[0] else {
            return Err(format!("Expected object, found: {:?}", tape[0]));
        };
        let (min_len, max_len) = match ctx.game {
            Game::JustDance2017 => (5, 6),
            _ => (6, 7),
        };
        if (min_len..=max_len).contains(&len) {
            return Err(format!("Object has {len} keys, expected at least {min_len} and at most {max_len}"));
        }
        let mut class = None;
        let mut clips = None;
        let mut tape_clock = None;
        let mut tape_bar_count = None;
        let mut free_resources_after_play: None;
        let mut map_name = None;
        let mut soundwich_event = None;

        for key_idx in 0..len {
            let Node::String(key) = tape[1 + key_idx] else {
                return Err(format!("Expected key, found: {:?}", tape[1 + key_idx]));
            };
            match key {
                "__class" => todo!(),
                "Clips" => todo!(),
                "TapeClock" => todo!(),
                "TapeBarCount" => todo!(),
                "FreeResourcesAfterPlay" => todo!(),
                "MapName" => todo!(),
                "SoundwichEvent" => todo!(),
                _ => return Err(format!("Unknown key: {}", key)),
            }
        }


        todo!()
    }
}
