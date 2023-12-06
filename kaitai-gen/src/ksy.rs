use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{anyhow, bail, Error};
use heck::ToPascalCase;
use serde_yaml::{Mapping, Value};

#[derive(Debug, Default)]
pub struct Ksy {
    pub meta: Option<Meta>,
    pub seq: Option<Attributes>,
    pub types: Option<HashMap<Identifier, Ksy>>,
}

impl Ksy {
    fn add_meta(&mut self, meta: Meta) -> Result<(), Error> {
        if self.meta.is_none() {
            self.meta = Some(meta);
            Ok(())
        } else {
            Err(anyhow!("meta specified more than once!"))
        }
    }

    fn add_seq(&mut self, seq: Attributes) -> Result<(), Error> {
        if self.seq.is_none() {
            self.seq = Some(seq);
            Ok(())
        } else {
            Err(anyhow!("seq specified more than once!"))
        }
    }

    fn add_types(&mut self, types: HashMap<Identifier, Self>) -> Result<(), Error> {
        if self.types.is_none() {
            self.types = Some(types);
            Ok(())
        } else {
            Err(anyhow!("types specified more than once!"))
        }
    }
}

impl Ksy {
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, Error> {
        let mapping: Mapping = serde_yaml::from_reader(reader)?;
        Self::from_mapping(mapping)
    }

    pub fn from_mapping(mapping: Mapping) -> Result<Self, Error> {
        let mut ksy = Self::default();
        for (key, value) in mapping {
            match (key.as_str(), value) {
                (Some("meta"), Value::Mapping(mapping)) => {
                    ksy.add_meta(Meta::from_mapping(mapping)?)?;
                }
                (Some("meta"), _) => panic!("meta is not a mapping!"),
                (Some("seq"), Value::Sequence(sequence)) => ksy.add_seq(
                    sequence
                        .into_iter()
                        .map(Attribute::from_value)
                        .collect::<Result<_, _>>()?,
                )?,
                (Some("seq"), _) => panic!("seq is not a sequence!"),
                (Some("types"), Value::Mapping(mapping)) => ksy.add_types(
                    mapping
                        .into_iter()
                        .map(|v| {
                            if let (Value::String(string), Value::Mapping(mapping)) = v {
                                Identifier::try_from(string)
                                    .and_then(|i| Self::from_mapping(mapping).map(|k| (i, k)))
                            } else {
                                Err(anyhow!("types has invalid types: {v:?}"))
                            }
                        })
                        .collect::<Result<_, _>>()?,
                )?,
                (Some("types"), _) => panic!("types is not a mapping!"),
                (Some(key), value) => {
                    panic!("Unrecognized ksy item, expected meta, seq, or types: {key:?} {value:?}")
                }
                (None, _) => panic!("ksy key is not a string!"),
            }
        }
        Ok(ksy)
    }
}

pub type Attributes = Vec<Attribute>;
pub type Doc = String;
pub type TypeRef = String;
pub type StringOrInteger = String;

#[derive(Debug, Default)]
pub struct Meta {
    pub id: Option<Identifier>,
    pub title: Option<String>,
    pub endian: Option<Endiannes>,
    pub encoding: Option<String>,
    pub file_extensions: Option<Vec<String>>,
    pub imports: Option<Vec<Import>>,
}

impl Meta {
    fn add_id(&mut self, id: Identifier) -> Result<(), Error> {
        if self.id.is_none() {
            self.id = Some(id);
            Ok(())
        } else {
            Err(anyhow!("id specified more than once!"))
        }
    }

    fn add_title(&mut self, title: String) -> Result<(), Error> {
        if self.title.is_none() {
            self.title = Some(title);
            Ok(())
        } else {
            Err(anyhow!("title specified more than once!"))
        }
    }

    fn add_endian(&mut self, endian: Endiannes) -> Result<(), Error> {
        if self.endian.is_none() {
            self.endian = Some(endian);
            Ok(())
        } else {
            Err(anyhow!("endian specified more than once!"))
        }
    }

    fn add_encoding(&mut self, encoding: String) -> Result<(), Error> {
        if self.encoding.is_none() {
            self.encoding = Some(encoding);
            Ok(())
        } else {
            Err(anyhow!("encoding specified more than once!"))
        }
    }

    fn add_file_extensions(&mut self, file_extensions: Vec<String>) -> Result<(), Error> {
        if self.file_extensions.is_none() {
            self.file_extensions = Some(file_extensions);
            Ok(())
        } else {
            Err(anyhow!("file_extensions specified more than once!"))
        }
    }

    fn add_imports(&mut self, imports: Vec<Import>) -> Result<(), Error> {
        if self.imports.is_none() {
            self.imports = Some(imports);
            Ok(())
        } else {
            Err(anyhow!("imports specified more than once!"))
        }
    }
}

impl Meta {
    pub fn from_mapping(mapping: Mapping) -> Result<Self, Error> {
        let mut meta = Self::default();
        for (key, value) in mapping {
            match (key.as_str(), value) {
                (Some("id"), Value::String(string)) => meta.add_id(Identifier::try_from(string)?)?,
                (Some("id"), _) => panic!("id is not a string!"),
                (Some("title"), Value::String(string)) => meta.add_title(string)?,
                (Some("title"), _) => panic!("title is not a string!"),
                (Some("endian"), Value::String(string)) => meta.add_endian(Endiannes::from_str(&string)?)?,
                (Some("endian"), _) => panic!("endian is not a string!"),
                (Some("encoding"), Value::String(string)) => meta.add_encoding(string)?,
                (Some("encoding"), _) => panic!("encoding is not a string!"),
                (Some("file-extension"), Value::String(string)) => meta.add_file_extensions(vec![string])?,
                (Some("file-extension"), Value::Sequence(sequence)) => meta.add_file_extensions(sequence.into_iter().map(|v| {
                    if let Value::String(string) = v {
                        Ok(string)
                    } else {
                        Err(anyhow!("file-extension is not a string"))
                    }
                }).collect::<Result<_,_>>()?)?,
                (Some("file-extension"), _) => panic!("file-extension is not a string or a sequence!"),
                (Some("imports"), Value::Sequence(sequence)) => meta.add_imports(sequence.into_iter().map(|v| {
                    if let Value::String(string) = v {
                        Import::try_from(string).map_err(Into::into)
                    } else {
                        Err(anyhow!("imports is not a string"))
                    }
                }).collect::<Result<_,_>>()?)?,
                (Some("imports"), _) => panic!("imports is not a sequence!"),
                (Some(key), value) => panic!("Unrecognized meta item, expected id, title, endian, or encoding: {key} {value:?}"),
                (None, _) => panic!("meta key is not a string!"),
            };
        }
        Ok(meta)
    }
}

#[derive(Debug, Default)]
pub struct Attribute {
    pub id: Option<Identifier>,
    pub r#type: Option<TypeRef>,
    pub size: Option<StringOrInteger>,
    pub doc: Option<Doc>,
    pub contents: Option<Contents>,
    pub repeat: Option<Repeat>,
    pub repeat_expr: Option<StringOrInteger>,
}

impl Attribute {
    pub fn from_value(value: Value) -> Result<Self, Error> {
        let Value::Mapping(mapping) = value else {
            bail!("Attribute is not a mapping!")
        };
        let mut attribute = Self::default();
        for (key, value) in mapping {
            match (key.as_str(), value) {
                (Some("id"), Value::String(id)) => attribute.id = Some(Identifier::try_from(id)?),
                (Some("type"), Value::String(r#type)) => attribute.r#type = Some(TypeRef::try_from(r#type)?),
                (Some("size"), Value::String(size)) => attribute.size = Some(StringOrInteger::from(size)),
                (Some("doc"), Value::String(doc)) => attribute.doc = Some(doc),
                (Some("contents"), value) => attribute.contents = Some(Contents::from_value(value)?),
                (Some("repeat"), Value::String(repeat)) => attribute.repeat = Some(Repeat::from_str(&repeat)?),
                (Some("repeat-expr"), Value::String(repeat_expr)) => attribute.repeat_expr = Some(StringOrInteger::from(repeat_expr)),
                (Some(key), value) => panic!("Unrecognized attribute item, expected id, type, size, doc, contents, repeat, or repeat-expr: {key} {value:?}"),
                (None, _) => panic!("attribute key is not a string!"),
            }
        }
        Ok(attribute)
    }
}

#[derive(Debug, Default)]
pub enum Endiannes {
    Big,
    #[default]
    Little,
}

impl FromStr for Endiannes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "be" => Ok(Self::Big),
            "le" => Ok(Self::Little),
            _ => Err(anyhow!("switch-on not supported! Expecting 'be' or 'le'")),
        }
    }
}

#[derive(Debug)]
pub enum Repeat {
    Expression,
    EndOfStream,
}

impl FromStr for Repeat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eos" => Ok(Self::EndOfStream),
            "expr" => Ok(Self::Expression),
            _ => Err(anyhow!("until not supported! Expecting 'eos' or 'expr'")),
        }
    }
}

#[derive(Debug)]
pub struct Contents(Vec<u8>);

impl Contents {
    pub fn from_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::Number(number) => {
                let n = number
                    .as_u64()
                    .and_then(|n| u8::try_from(n).ok())
                    .ok_or_else(|| anyhow!("Number is too big!"))?;
                Ok(Self(vec![n]))
            }
            Value::String(string) => Ok(Self(string.as_bytes().to_vec())),
            Value::Sequence(sequence) => {
                let mut vec = Vec::new();
                for value in sequence {
                    match value {
                        Value::Number(number) => {
                            let n = number
                                .as_u64()
                                .and_then(|n| u8::try_from(n).ok())
                                .ok_or_else(|| anyhow!("Number is too big!"))?;
                            vec.push(n);
                        }
                        Value::String(string) => {
                            vec.extend_from_slice(string.as_bytes());
                        }
                        _ => panic!("Expected a number or string in seqeunce for contents"),
                    }
                }
                Ok(Self(vec))
            }
            _ => Err(anyhow!(
                "Expected a number, string, or sequence of numbers and/or strings for contents"
            )),
        }
    }
}

/// With this macro you can create a Regex that is only compiled once.
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn to_pascal_case(&self) -> String {
        self.0.to_pascal_case()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Identifier {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex = regex!(r"^[a-z][a-z0-9_]*$");
        if regex.is_match(&value) {
            Ok(Self(value))
        } else {
            Err(anyhow!(
                "Identifier does not match the regex '^[a-z][a-z0-9_]*$'"
            ))
        }
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Import {
    RelativePath(String),
    Identifier(Identifier),
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RelativePath(path) => f.write_str(path),
            Self::Identifier(id) => f.write_str(&id.0),
        }
    }
}

impl TryFrom<String> for Import {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex = regex!(r"^(.*/)?[a-z][a-z0-9_]*$");
        if regex.is_match(&value) {
            Ok(Self::RelativePath(value))
        } else {
            Err(anyhow!(
                "Identifier does not match the regex '^[a-z][a-z0-9_]*$'"
            ))
        }
    }
}

impl From<Identifier> for Import {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn split_path() {
        let file =
            File::open("/home/kriskras99/Source/ferris_dancing/ubiart-ksy/split_path.ksy").unwrap();
        let _ksy = Ksy::from_reader(file).unwrap();
    }

    #[test]
    fn alias8() {
        let file =
            File::open("/home/kriskras99/Source/ferris_dancing/ubiart-ksy/alias8.ksy").unwrap();
        let _ksy = Ksy::from_reader(file).unwrap();
    }
}
