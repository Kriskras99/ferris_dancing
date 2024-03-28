#![no_main]

extern crate libfuzzer_sys;

use ubiart_toolkit::cooked::act::Actor;
use libfuzzer_sys::fuzz_target;
use dotstar_toolkit_utils::bytes::write::BinarySerialize;
use ubiart_toolkit::utils::{Game, UniqueGameId, Platform};

fuzz_target!(|actor: Actor| {
    let mut vec = Vec::new();
    if actor.serialize(&mut vec).is_ok() {
        let new_actor = ubiart_toolkit::cooked::act::parse(&vec, &mut 0, UniqueGameId { game: Game::JustDance2022, platform: Platform::Nx, id: 0}).unwrap();
        assert_eq!(actor, new_actor);
    }
});
