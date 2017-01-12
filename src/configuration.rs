use {ConfigurationContent, ConfigurationFile, IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};
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
    Watched,
    Ignored
}

#[derive(Debug)]
pub enum ConfigureContentError {
    BadPosition(String),
    EncodingError(String),
    InternalError(String),
    KeyAlreadyExists(String),
    UnknownKey(String),
}

impl fmt::Display for ConfigureContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureContentError::BadPosition(ref e) => e.fmt(f),
            ConfigureContentError::EncodingError(ref e) => e.fmt(f),
            ConfigureContentError::InternalError(ref e) => e.fmt(f),
            ConfigureContentError::KeyAlreadyExists(ref e) => e.fmt(f),
            ConfigureContentError::UnknownKey(ref e) => e.fmt(f),
        }
    }
}

pub trait ConfigureContent {
    fn add_entry(&mut self, key: &str, value: &Entry, category: &EntryCategory) -> Result<()>;
    fn remove_entry(&mut self, key: &str, category: &EntryCategory) -> Result<()>;
    fn transfer_entry(&mut self, key: &str, old_category: &EntryCategory, new_category: &EntryCategory) -> Result<()>;
}

impl ConfigureContent for ConfigurationContent {
    fn add_entry(&mut self, key: &str, value: &Entry, category: &EntryCategory) -> Result<()> {
        let entry_path_name: String = match category {
            &EntryCategory::Watched => format!("{}.{}", WATCHED_ENTRY_NAME, key),
            &EntryCategory::Ignored => format!("{}.{}", IGNORED_ENTRY_NAME, key),
        };
        if self.contains_key(&entry_path_name) {
            return Err(ConfigureContentError::KeyAlreadyExists(entry_path_name));
        }
        let mut encoder = ConfigurationFile::new();
        match value.encode(&mut encoder) {
            Ok(_) => {
                self.insert(entry_path_name, Value::Table(encoder.toml));
                Ok(())
            },
            Err(error) => Err(ConfigureContentError::EncodingError(String::from(error.description()))),
        }
    }

    fn remove_entry(&mut self, key: &str, category: &EntryCategory) -> Result<()> {
         let entry_path_name: String = match category {
            &EntryCategory::Watched => format!("{}.{}", WATCHED_ENTRY_NAME, key),
            &EntryCategory::Ignored => format!("{}.{}", IGNORED_ENTRY_NAME, key),
        };
        if !self.contains_key(&entry_path_name) {
            return Err(ConfigureContentError::UnknownKey(entry_path_name));
        }
        match self.remove(&entry_path_name) {
            Some(_) => Ok(()),
            None => Err(ConfigureContentError::InternalError(format!("Canno't remove the key {} from the data structure", entry_path_name))),
        }
    }

    fn transfer_entry(&mut self, key: &str, old_category: &EntryCategory, new_category: &EntryCategory) -> Result<()> {
        unimplemented!();
    }

}
