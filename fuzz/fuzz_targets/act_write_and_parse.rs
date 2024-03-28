#![no_main]

extern crate libfuzzer_sys;

use ubiart_toolkit::cooked::act::Actor;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|actor: Actor| {
    let mut vec = Vec::new();
    if actor.serialize(&mut vec).is_ok() {
        if let Ok(new_actor) = Actor::deserialize(&vec) {
            assert_eq!(actor, new_actor)
        }
    }
});
