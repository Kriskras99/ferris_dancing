mod codegen;
mod ksy;

use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Error};
use codegen::codegen;
use ksy::{Identifier, Import, Ksy};

#[derive(Debug, Default)]
#[must_use]
pub struct Builder {
    identifier_to_ksy: HashMap<Identifier, Ksy>,
    path_to_identifier: HashMap<PathBuf, Identifier>,
}

impl Builder {
    pub fn add_ksy_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let path = path.canonicalize()?;
        // Make sure not to parse the same file more than once
        // Can happen if a user adds a file that is also imported
        if !self.path_to_identifier.contains_key(&path) {
            let file = File::open(&path)?;
            let mut ksy = Ksy::from_reader(file)?;
            let Some(meta) = &mut ksy.meta else {
                bail!("{path:?} does not have /meta")
            };
            let Some(id) = &meta.id else {
                bail!("{path:?} does not have /meta/id")
            };
            if self.identifier_to_ksy.contains_key(id) {
                bail!("{id} specified twice! Second time in {path:?}");
            }
            if let Some(imports) = &mut meta.imports {
                for import in imports {
                    let Import::RelativePath(relative_path) = import else {
                        unreachable!()
                    };
                    println!("Rel: {relative_path}");
                    let mut path = path.with_file_name(relative_path);
                    if !path.set_extension("ksy") {
                        bail!("Can't add .ksy extension to {path:?}");
                    }
                    println!("{path:?}");
                    path = path.canonicalize()?;
                    self = self.add_ksy_file(&path)?;
                    *import = Import::Identifier(self.path_to_identifier[&path].clone());
                }
            }
            self.path_to_identifier.insert(path, id.clone());
            self.identifier_to_ksy.insert(id.clone(), ksy);
        }
        Ok(self)
    }

    pub fn generate(self) -> Result<Source, Error> {
        let scope = codegen(self.identifier_to_ksy)?;
        Ok(Source {
            string: scope.to_string(),
        })
    }
}

pub struct Source {
    string: String,
}

impl Source {
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.string.as_bytes())?;
        Ok(())
    }
}
