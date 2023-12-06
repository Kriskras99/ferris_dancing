use std::collections::{HashSet, HashMap};

use crate::ksy::{Identifier, Attribute, Import, Ksy, Endiannes};


pub struct Root {
    pub modules: HashMap<Identifier, Type>,
}

impl Root {
    pub fn validate(ksys: HashMap<Identifier, Ksy>) -> Self {
        let mut modules = HashMap::with_capacity(ksys.capacity());
        for (id, ksy) in ksys {
            assert!(modules.insert(id, Type::validate(ksy, None)).is_none())
        }
        Self {
            modules
        }
    }
}

pub struct Type {
    pub identifier: Identifier,
    pub imports: Vec<Import>,
    pub endian: Endiannes,
    pub encoding: String,
    pub seq: Vec<Attribute>,
    pub instances: HashMap<Identifier, Attribute>,
    pub enums: Vec<String>,
    pub subtypes: Vec<Type>,
}

impl Type {
    pub fn validate(ksy: Ksy, identifier: Option<Identifier>) -> Self {
        let identifier = if let Some(identifier) = identifier {
            identifier
        } else {
            ksy.meta.and_then(|m| m.id).expect("Missing identifier!")
        };
        let mut ty = Self {
            identifier,
            imports: Vec::new(),
            endian: Endiannes::Little,
            encoding: String::from("UTF-8"),
            seq: Vec::new(),
            instances: HashMap::new(),
            enums: Vec::new(),
            subtypes: Vec::new(),
        };
        ty
    }
}
