# JDMod
Application for modding Just Dance games.
The goal is to be able to import every Just Dance game ever made and to be made, and be able to export to Just Dance 2022 Switch.

It can import and export songs, playlists, quests/objectives, avatars, aliases, portraitborders, gacha machine, and search labels.

Platform support matrix:

|        | 2014 | 2015 | 舞力全开2015 | 2016 | 2017 | 舞力全开2017 | 2018 | 2019 | 2020 | 舞力全开 | 2021 | 2022 |
| ------ | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| Switch |      |      |      |      |  ✅   |      |  ✅   |  ✅   |  ✅   |  ✅   |  ✅   |  ✅   |
| PS4    |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |      |  ❌   |  ❌   |
| Wii U  |  ❌   |  ⚠<sup>1</sup>   |      |  ✅   |  ✅         ||  ❌   |  ❌   |      |      |      |      |
| X360   |  ❌   |  ❌   |      |  ❌   |  ❌   |      |  ❌   |  ❌   |      |      |      |      |
| PS3    |  ❌   |  ❌   |      |  ❌   |  ❌   |      |  ❌   |      |      |      |      |      |
| Win    |      |      |      |      |  ✅   |      |      |      |      |      |      |      |
| Wii    |  ❌   |  ❌   |      |  ❌   |  ❌   |      |  ❌   |  ❌   |  ❌   |      |      |      |

<sup>1</sup> Only DLC is supported

Legend:
- Empty cell: Game not released on that platform
- ✅: Supported in JDMod
- ⚠: Partial support in JDMod, see footnotes
- ❌: Not yet supported in JDMod

## Usage
```
jdmod new path/to/switch/just/dance/2022/secure_fat.gf path/to/where/you/want/the/mod
jdmod import path/to/game/files/to/import path/to/where/you/placed/the/mod
```
Every game, dlc, and song you want to import should be in seperate directories.

## FAQ
### When I try to open it, nothing happens!
This is a CLI application and only works in the terminal.

### Just Dance X is not listed! Is it supported?
**No**, currently only games using the UbiArt engine are supported. Any game not listed here uses a different game engine.

### Xbox One/Series is not listed! Is it supported?
**No**, there are no known hacks of the Xbox One/Series. Therefore song data can not be extracted from these platforms.

### I want to create a mod for a platform that is not the Nintendo Switch, is this supported?
**No**, you can only export a mod created in this program for the Nintendo Switch. Any other platform is not supported.
If you have the programming skills, you're welcome to add support for it.

### I want to create a mod for a game that is not Just Dance 2022, is this supported?
**No**, you can only export a mod created in this program for Just Dance 2022. Any other game is not supported.
