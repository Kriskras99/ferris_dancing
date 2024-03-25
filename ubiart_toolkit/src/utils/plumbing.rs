use self::sealed::Sealed;
use super::{Game, Platform};

mod sealed {
    pub trait Sealed {}
}

pub trait GamePlatform: Sealed + Clone + Copy + std::fmt::Debug + PartialEq {
    fn version() -> Game;
    fn platform() -> Platform;
    fn id() -> u32;
}

macro_rules! create_gameplatform {
    ( $name:ident, $game:expr, $platform:expr, $id:literal ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {}

        impl Sealed for $name {}
        impl GamePlatform for $name {
            fn version() -> super::Game {
                $game
            }

            fn platform() -> super::Platform {
                $platform
            }

            fn id() -> u32 {
                $id
            }
        }
    };
}

create_gameplatform!(Wii2014, Game::JustDance2014, Platform::Wii, 0x1C24_B91A);
create_gameplatform!(WiiU2015, Game::JustDance2015, Platform::WiiU, 0xC563_9F58);
create_gameplatform!(Nx2017, Game::JustDance2017, Platform::Nx, 0x32F3_512A);
create_gameplatform!(Nx2018, Game::JustDance2018, Platform::Nx, 0x032E_71C5);
create_gameplatform!(Nx2019, Game::JustDance2019, Platform::Nx, 0x57A7_053C);
create_gameplatform!(Nx2020, Game::JustDance2020, Platform::Nx, 0x217A_94CE);
create_gameplatform!(Wii2020, Game::JustDance2020, Platform::Wii, 0x4C8E_C5C5);
create_gameplatform!(NxChina, Game::JustDanceChina, Platform::Nx, 0xA155_8F87);
create_gameplatform!(Nx2021, Game::JustDance2021, Platform::Nx, 0xA4F0_18EE);
create_gameplatform!(Nx2022, Game::JustDance2022, Platform::Nx, 0x1DDB_2268);
