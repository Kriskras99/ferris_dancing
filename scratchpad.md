WiiU: 
 - https://github.com/VitaSmith/cdecrypt (can decrypt dlc and update)
 - https://gbatemp.net/threads/wii-u-image-wud-compression-tool.397901/ (wux to wud)
 - https://github.com/Maschell/JWUDTool/releases/tag/0.4 (wux to decrypted)

Kpop dance practice to video?
https://www.youtube.com/watch?v=G_Lhkhxl8BU

Between PS4 2022 Chandelier and NX 2022 Chandelier
- Shared moves under world/maps/chandelier/timeline/moves/wiiu/*.msm
- Shared world/maps/chandelier/autodance/chandelier.ogg
- Shared extensions:
    - *.tpl.ckd (JSON)
    - *.stape.ckd (JSON)
    - *.act.ckd (Binary with strings)
    - *.sgs.ckd (JSON)
    - *.tape.ckd (JSON)
    - *.ktape.ckd (JSON)
    - *.mpd.ckd (ps4 is chandelier.hd.mpd.ckd, nx is chandelier.mpd.ckd but is same file) (Binary)
    - *.msm (Binary)
- Not shared:
    - *.wav.ckd (Binary)
    - *.isc.ckd (XML)
    - *.dtape.ckd (JSON)
    - *.webm (ps4 is 1080p VP8, nx is 720p VP9) (Binary)
- Unique files for nx:
    - nx/cache/itf_cooked/nx/world/maps/chandelier/menuart/actors/chandelier_cover_albumbkg.act.ckd
    - nx/cache/itf_cooked/nx/world/maps/chandelier/menuart/textures/chandelier_cover_albumbkg.tga.ckd
- Unique files for ps4:
    - world/maps/{song}/timeline/moves/orbis/*.gesture (Binary)

TODO:
- Songs with 3 rewards (2020/2021) only show 2 rewards
- Alts which require 5 dances to unlock (2020) only show 3 bars (but do show correct text)
- Add more coaches for the loading screen
    - coaches are in cache/../world/ui/textures/coaches/bootloading_{}.tga.ckd
    - scene is in cache/../world/ui/screens/boot_loading/
- Add sticker mode back if possible
- Having an objective with type OpenAnthologyMode with crash 2022 on start
- Fix that not all kids songs appear (gc_carousel_rules.json:songItemLists:Kids)
- Have catalog show games per edition
- Figure out .msm files and convert between them