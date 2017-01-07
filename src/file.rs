use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use toml::{encode_str, Encoder, Parser, Table, Value};

static CONFIGURATION_FILE_NAME: &'static str = ".gpm";

/// The type of the content file is a Table type.
pub type ConfigurationContent = Table;

/// The configuration file is basically a TOML file that contain some informations about local git
/// projects.
pub type ConfigurationFile = Encoder;

pub trait ConfigurationFileExtension {
    ///
    /// This method will initialize the configuration file with some basic content.
    ///
    fn init() -> Self;
    fn save(&self, path: &Path) -> Result<(), Error>;
}

impl<'a> ConfigurationFileExtension for ConfigurationFile {
    ///
    /// This method returns a ConfigurationFile type.
    ///
    fn init() -> Self {
        let mut encoder = ConfigurationFile::new();
        let toml_content = r#"
            [watch]
            
            [ignored]

            [groups]
            "#;
        encoder.toml = Parser::new(toml_content).parse().unwrap();
        encoder
    }

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
    /// This method will load a file, get the content and parse it.
    ///
    fn parse_from_file(path: &Path) -> Option<Table>;
}

impl<'a> TomlExtension for Parser<'a> {
    ///
    /// This method returns an Option type that contains a toml::Table type.
    ///
    fn parse_from_file(path: &Path) -> Option<Table> {
        let mut configuration_file = File::open(path);
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
