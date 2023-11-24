mod ksy;
mod source;

use std::{path::{Path, PathBuf}, fs::File, io::Write};

use anyhow::Error;
use ksy::Ksy;

#[derive(Debug, Default)]
#[must_use]
pub struct Builder {
    ksy_paths: Vec<PathBuf>,
}

impl Builder {
    pub fn add_ksy_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.ksy_paths.push(path.as_ref().to_owned());
        self
    }

    pub fn generate(self) -> Result<Source, Error> {
        let mut string = String::new();
        for ksy_path in self.ksy_paths {
            let file = File::open(ksy_path)?;
            let ksy = Ksy::from_reader(file);
            string.push_str(&format!("{ksy:#?}"));
        }
        Ok(Source {
            string
        })
    }
}

pub struct Source {
    string: String
}

impl Source {
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.string.as_bytes())?;
        Ok(())
    }
}


