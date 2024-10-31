# UbiArt Toolkit
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

## Library
This library has one feature that can be enabled:
 - `full_json_types`: Enable all JSON types (will increase compile times)
