# Ferris Dancing🦀
Tools and libraries for working with Just Dance games.

## [JDMod](jdmod/README.md)
Application for modding Just Dance games.
The goal is to be able to import every Just Dance game ever made and to be made, and be able to export to Just Dance 2022 Switch.

Currently supported are Just Dance 2017-2022 for the Switch.
It can import and export songs, playlists, quests/objectives, avatars, aliases, portraitborders, gacha machine, and search labels.

## [UbiArt Toolkit](ubiart_toolkit/README.md)
Tools and library for parsing and writing various UbiArt files.

Supports the following formats (Nintendo Switch games only!)
| File extension | Tool       | Remarks                          |
| -------------- | ---------- | -------------------------------- |
| .act.ckd       | acttool    |                                  |
| .alias8        | alias8tool |                                  |
| .ipk           | ipktool    | Supports extraction and creation |
| .isc.ckd       | isctool    |                                  |
| .sgs.ckd       | jsontool   |                                  |
| .isg.ckd       | jsontool   |                                  |
| .json.ckd      | jsontool   |                                  |
| .msm.ckd       | msmtool    | does nothing useful yet          |
| .png.ckd       | xtxtool    | Supports decoding to png         |
| .tga.ckd       | xtxtool    | Supports decoding to png         |
| secure_fat.gf  | sfattool   | Only lists files                 |
It also has the tools `stringidtool` and `ubicrctool` for easy creation of ids as used in the UbiArt engine

## [Dolphin Toolkit](dolphin_toolkit/README.md)
Tools and library for parsing Nintendo Wii file formats.

Supports the following formats
| File extension | Tool    |  Remarks                                |
| -------------- | ------- | --------------------------------------- | 
| .app           | u8tool  | Supports extraction                     |
| .wad           | wadtool | Supports extraction of Installable WADs |

## [.* Toolkit Utils](dotstar_toolkit_utils/README.md)
Library with various utilities for writing parsers.
It contains three sections:
1. `bytes`: contains Byteorder like functions for reading integers and strings from byte slices.
2. `testing`: contains alternatives to the assert! family that return Results instead of panicking.
3. `vfs`: contains traits for a virtual filesystem and some basic filesystems that allow for parsing without extracting
