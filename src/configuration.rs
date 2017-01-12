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
    pub created: String,
    pub updated: String,
}

impl Entry {
    pub fn new(name: &str, path: &str) -> Self {
        Entry {
            name: String::from(name),
            path: String::from(path),
            created: UTC::now().to_rfc2822(),
            updated: UTC::now().to_rfc2822(),
        }
    }
}

type Result<T> = result::Result<T, ConfigureContentError>;

pub enum EntryCategory {
    Body,
    Watched,
    Ignored
}

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
    fn add_entry(&mut self, key: &str, value: &Entry, category: &EntryCategory) -> Result<()>;
    fn remove_entry(&self, key: &str, category: &EntryCategory) -> Result<()>;
    fn transfer_entry(&self, key: &str, category: &EntryCategory) -> Result<()>;
}

impl ConfigureContent for ConfigurationContent {
    fn add_entry(&mut self, key: &str, value: &Entry, category: &EntryCategory) -> Result<()> {
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

    fn remove_entry(&self, key: &str, category: &EntryCategory) -> Result<()> {
        unimplemented!()
    }

    fn transfer_entry(&self, key: &str, category: &EntryCategory) -> Result<()> {
        unimplemented!()
    }

}
