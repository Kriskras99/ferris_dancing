meta:
  id: alias8
  title: UbiArt .alias8
  file-extension: alias8
  endian: be
  encoding: UTF-8
  imports:
    - split_path

seq:
  - id: unk1
    contents: [0x00, 0x00, 0x00, 0x02]
  - id: num_aliases
    type: u4
    doc: Number of aliases in the file
  - id: aliases
    type: alias
    repeat: expr
    repeat-expr: num_aliases

types:
  alias:
    seq:
      - id: len_alias1
        type: u4
        doc: The length of the first alias
      - id: first_alias
        type: str
        doc: The first alias
        size: len_alias1
      - id: len_alias2
        type: u4
        doc: The length of the second alias
      - id: second_alias
        type: str
        doc: The second alias
        size: len_alias2
      - id: path
        type: split_path
        doc: The path the aliases point to
      - id: unk2
        contents: [0xFF, 0xFF]
      - id: unk3
        doc: Unknown, possible values in [`Alias::UNK3`]
        type: u2
