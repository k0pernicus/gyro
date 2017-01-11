use {ConfigurationContent, ConfigurationFile};
use chrono::DateTime;
use chrono::offset::utc::UTC;
use rustc_serialize::Encodable;
use std::error::Error;
use std::fmt;
use std::result;
use toml::Value;

#[derive(RustcEncodable)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub created: DateTime<UTC>,
    pub updated: DateTime<UTC>,
}

type Result<T> = result::Result<T, ConfigureContentError>;

#[derive(Debug)]
pub enum ConfigureContentError {
    BadPosition(String),
    EncodingError(String),
    KeyAlreadyExists(String),
    UnknownKey(String),
}

impl fmt::Display for ConfigureContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureContentError::BadPosition(ref e) => e.fmt(f),
            ConfigureContentError::EncodingError(ref e) => e.fmt(f),
            ConfigureContentError::KeyAlreadyExists(ref e) => e.fmt(f),
            ConfigureContentError::UnknownKey(ref e) => e.fmt(f),
        }
    }
}


pub trait ConfigureContent {
    fn add_watch_entry(&mut self, key: &str, value: &Entry) -> Result<()>;
    fn add_ignored_entry(&self, key: &str, value: &Entry) -> Result<()>;
    fn remove_watch_entry(&self, key: &str) -> Result<()>;
    fn remove_ignored_entry(&self, key: &str) -> Result<()>;
    fn watch_to_ignored_entry(&self, key: &str) -> Result<()>;
    fn ignored_to_watch_entry(&self, key: &str) -> Result<()>;
}

impl ConfigureContent for ConfigurationContent {
    fn add_watch_entry(&mut self, key: &str, value: &Entry) -> Result<()> {
        if self.contains_key(key) {
            return Err(ConfigureContentError::KeyAlreadyExists(String::from(key)));
        }
        let mut encoder = ConfigurationFile::new();
        match value.encode(&mut encoder) {
            Ok(_) => {
                self.insert(String::from(key), Value::Table(encoder.toml));
                Ok(())
            },
            Err(error) => Err(ConfigureContentError::EncodingError(String::from(error.description()))),
        }
    }

    fn add_ignored_entry(&self, key: &str, value: &Entry) -> Result<()> {
        unimplemented!()
    }

    fn remove_watch_entry(&self, key: &str) -> Result<()> {
        unimplemented!()
    }

    fn remove_ignored_entry(&self, key: &str) -> Result<()> {
        unimplemented!()
    }

    fn watch_to_ignored_entry(&self, key: &str) -> Result<()> {
        unimplemented!()
    }

    fn ignored_to_watch_entry(&self, key: &str) -> Result<()> {
        unimplemented!()
    }

}
