use std::{path::Path, fs::File};

use toml::Table;


pub struct Toml {
    pub table: Table,
    file_name: String
}

impl Toml {
    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }

    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {

        use std::io::prelude::*;

        let file_name = if let Some(s) = Path::file_name(&path.as_ref()) {
            if let Some(s) = s.to_str() {
                String::from(s)
            } else {
                String::from("<invalid string>")
            }
        } else {
            String::from("<invalid string>")
        };

        let mut file = File::open(path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let table = buffer.parse::<Table>()?;

        Ok(Self {
            table,
            file_name
        })
    }
}
