use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use toml::{encode_str, Parser, Value};

use {ConfigurationContent, ConfigurationFile, WATCHED_ENTRY_NAME, IGNORED_ENTRY_NAME,
     GROUPS_ENTRY_NAME};

pub trait ConfigurationFileExtension {
    ///
    /// This method initializes the configuration file with some basic content.
    ///
    fn init() -> Self;

    ///
    /// This method saves, into a file (given by it path), the content of the object.
    ///
    fn save(&self, path: &Path) -> Result<(), Error>;
}

impl<'a> ConfigurationFileExtension for ConfigurationFile {
    ///
    /// This method returns a ConfigurationFile type.
    ///
    fn init() -> Self {
        let mut encoder = ConfigurationFile::new();
        let toml_content = format!(r#"
            [{}]
            
            [{}]

            [{}]
            "#,
                                   WATCHED_ENTRY_NAME,
                                   IGNORED_ENTRY_NAME,
                                   GROUPS_ENTRY_NAME);
        encoder.toml = Parser::new(&toml_content).parse().unwrap();
        encoder
    }

    ///
    /// This method returns a Result type.
    ///
    fn save(&self, path: &Path) -> Result<(), Error> {
        let content_string = encode_str(&Value::Table(self.toml.clone()));
        let mut configuration_file = File::create(path)
            .expect("Could not open the configuration file!");
        configuration_file.write_all(content_string.as_bytes())
            .expect("Could not write to configuration file!");
        Ok(())
    }
}

/// Extension of the Toml crate
pub trait TomlExtension {
    ///
    /// This method loads a file, get the content and parse it.
    ///
    fn parse_from_file(path: &Path) -> Option<ConfigurationContent>;
}

impl<'a> TomlExtension for Parser<'a> {
    ///
    /// This method returns an Option type that contains the configuration file content.
    ///
    fn parse_from_file(path: &Path) -> Option<ConfigurationContent> {
        let configuration_file = File::open(path);
        // Return None if the path is not ok
        if configuration_file.is_err() {
            return None;
        }
        let mut content_file = String::new();
        match configuration_file.unwrap().read_to_string(&mut content_file) {
            // Return the result of the Toml parser
            Ok(_) => Parser::new(&content_file).parse(),
            // Return None if there is an error reading the file
            Err(_) => None,
        }
    }
}
