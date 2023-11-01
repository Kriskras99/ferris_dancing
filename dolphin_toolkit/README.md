# Dolphin Toolkit
A library and CLI tools for parsing Nintendo Wii file formats.

Currently supported file formats are:
| File format | Extension | Supported                        |
| ----------- | --------- | -------------------------------- |
| U8          | .app      | yes                              |
| WAD         | .wad      | Partially, only Installable WADs |

## CLI tools
Two tools are provided, `u8tool` and `wadtool`.

### `u8tool`
```
Library and CLI tools for working with Nintendo Wii file formats

Usage: u8tool <SOURCE> [DESTINATION]

Arguments:
  <SOURCE>       The file to parse
  [DESTINATION]  The directory to extract all files too

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `wadtool`
```
Library and CLI tools for working with Nintendo Wii file formats

Usage: wadtool <SOURCE> [DESTINATION]

Arguments:
  <SOURCE>       The file to parse
  [DESTINATION]  The directory to extract all files too

Options:
  -h, --help     Print help
  -V, --version  Print version
```
