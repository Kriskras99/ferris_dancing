meta:
  id: split_path
  title: Split path with CRC (UbiArt)
  endian: be
  encoding: UTF-8

seq:
  - id: len_filename
    type: u4
  - id: filename
    type: str
    size: len_filename
  - id: len_path
    type: u4
  - id: path
    type: str
    size: len_path
  - id: ubiart_crc
    type: u4
    doc: This is the UbiArt CRC of path+filename
  - id: padding
    contents: [0x00, 0x00, 0x00, 0x00]
