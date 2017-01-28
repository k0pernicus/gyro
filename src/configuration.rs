use {ConfigurationContent, ConfigurationFile, GROUPS_ENTRY_NAME, IGNORED_ENTRY_NAME,
     WATCHED_ENTRY_NAME};
use chrono::offset::utc::UTC;
use rustc_serialize::Encodable;
use std::error::Error;
use std::fmt;
use std::result;
use toml;
use toml::Value;

///
/// An entry is corresponding to a git repository stored in the configuration file:
/// `name`: The name of git repository
/// `path`: The local path of the git repository parent
/// `updated`: The last time that informations have been updated from the given repository
///
#[derive(RustcDecodable, RustcEncodable)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub updated: String,
}

impl Entry {
    ///
    /// The function to instanciate a new Entry structure
    /// This function takes as parameters `name` et `path`
    /// It initialize the `updated` field automatically
    ///
    pub fn new(name: &str, path: &str) -> Self {
        Entry {
            name: String::from(name),
            path: String::from(path),
            updated: UTC::now().to_rfc2822(),
        }
    }

    ///
    /// A method to update the `updated` field
    ///
    pub fn update(&mut self) {
        self.updated = UTC::now().to_rfc2822();
    }
}

///
/// A custom type that return a T type, or a ConfigureContentError error
///
type Result<T> = result::Result<T, ConfigureContentError>;

///
/// The specific category to add or remove an entry from the configuration file:
/// `Groups` is corresponding to the `groups` array title
/// `Ignored` is corresponding to the `ignored` array title
/// `Watched` is corresponding to the `watched` array title
///
#[derive(Debug, PartialEq)]
pub enum EntryCategory {
    Groups,
    Ignored,
    Watched,
}

///
/// Enum that contain possible error flags for ConfigureContent:
/// `BadPosition` is corresponding to an error when transfering an entry between two categories
/// `DecodingError` is corresponding to an error when decoding a Toml value to a Rust data structure
/// `EncodingError` is corresponding to an error when encoding a Rust data structure to a Toml value
/// `InternalError` is corresponding to internal errors for Rust data structures (like insertion)
/// `KeyAlreadyExists` is corresponding to an error when inserting a key that already exists
/// `UnknownKey` is corresponding to an error when deleting a key that does not exists
///
#[derive(Debug)]
pub enum ConfigureContentError {
    BadPosition(String),
    DecodingError(String),
    EncodingError(String),
    InternalError(String),
    KeyAlreadyExists(String),
    UnknownKey(String),
}

impl fmt::Display for ConfigureContentError {
    ///
    /// Function that format the specific error to display
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigureContentError::BadPosition(ref e) => e.fmt(f),
            ConfigureContentError::DecodingError(ref e) => e.fmt(f),
            ConfigureContentError::EncodingError(ref e) => e.fmt(f),
            ConfigureContentError::InternalError(ref e) => e.fmt(f),
            ConfigureContentError::KeyAlreadyExists(ref e) => e.fmt(f),
            ConfigureContentError::UnknownKey(ref e) => e.fmt(f),
        }
    }
}

pub trait ConfigureContent {
    ///
    /// Method to get the entry path, from a key and a category
    ///
    fn get_entry_path(&self, key: &str, category: &EntryCategory) -> String;

    ///
    /// Method to add a single entry value, represented by a string key, in a given category
    ///
    fn add_entry(&mut self,
                 key: &str,
                 entry_value: &mut Entry,
                 category: &EntryCategory)
                 -> Result<()>;

    ///
    /// Method to remove a given string key, in a category
    ///
    fn remove_entry(&mut self, key: &str, category: &EntryCategory) -> Result<Value>;

    ///
    /// Method to transfer a given entry (represented by a string key), from an old category to a
    /// new one
    ///
    fn transfer_entry(&mut self,
                      key: &str,
                      old_category: &EntryCategory,
                      new_category: &EntryCategory)
                      -> Result<()>;
}

impl ConfigureContent for ConfigurationContent {
    ///
    /// This method returns a String type, that represents a complete path entry
    ///
    fn get_entry_path(&self, key: &str, category: &EntryCategory) -> String {
        match category {
            &EntryCategory::Groups => format!("{}.{}", GROUPS_ENTRY_NAME, key),
            &EntryCategory::Ignored => format!("{}.{}", IGNORED_ENTRY_NAME, key),
            &EntryCategory::Watched => format!("{}.{}", WATCHED_ENTRY_NAME, key),
        }
    }

    ///
    /// This method returns a Result type, that represents if the entry has been successfully added
    /// to the given entry, or an error
    ///
    fn add_entry(&mut self,
                 key: &str,
                 entry_value: &mut Entry,
                 category: &EntryCategory)
                 -> Result<()> {
        let entry_path_name = self.get_entry_path(key, category);
        if self.contains_key(&entry_path_name) {
            return Err(ConfigureContentError::KeyAlreadyExists(entry_path_name));
        }
        entry_value.update();
        let mut encoder = ConfigurationFile::new();
        match entry_value.encode(&mut encoder) {
            Ok(_) => {
                self.insert(entry_path_name, Value::Table(encoder.toml));
                Ok(())
            }
            Err(error) => {
                Err(ConfigureContentError::EncodingError(String::from(error.description())))
            }
        }
    }

    ///
    /// This method returns a Result type, that represents the value that been removes, or an error
    ///
    fn remove_entry(&mut self, key: &str, category: &EntryCategory) -> Result<Value> {
        let entry_path_name = self.get_entry_path(key, category);
        if !self.contains_key(&entry_path_name) {
            return Err(ConfigureContentError::UnknownKey(entry_path_name));
        }
        match self.remove(&entry_path_name) {
            Some(value) => Ok(value),
            None => {
                Err(ConfigureContentError::InternalError(format!("Can not remove the key '{}' \
                                                                  from the data structure",
                                                                 entry_path_name)))
            }
        }
    }

    ///
    /// This method returns a Result type, that represents if the entry has been successfully
    /// transfered, or an error
    ///
    fn transfer_entry(&mut self,
                      key: &str,
                      old_category: &EntryCategory,
                      new_category: &EntryCategory)
                      -> Result<()> {
        if old_category == new_category {
            return Err(ConfigureContentError::InternalError(format!("Entry categories are \
                                                                     equals")));
        }
        let entry_value: Value = self.remove_entry(key, old_category).unwrap();
        match toml::decode::<Entry>(entry_value) {
            Some(ref mut entry) => self.add_entry(key, entry, new_category),
            None => {
                Err(ConfigureContentError::DecodingError(format!("Can not decode the current \
                                                                  value from '{:?}' to '{:?}'",
                                                                 old_category,
                                                                 new_category)))
            }
        }
    }
}
