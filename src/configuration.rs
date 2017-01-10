use chrono::DateTime;
use chrono::offset::utc::UTC;
use ConfigurationContent;

struct Entry {
    name: String,
    path: String,
    created: DateTime<UTC>,
    updated: DateTime<UTC>,
}

enum ConfigureContentError {
    BadPosition(String),
    KeyAlreadyExists(String),
    UnknownKey(String),
}

trait ConfigureContent {
    fn add_watch_entry(&self, key: String, value: &Entry) -> Result<(), ConfigureContentError>;
    fn add_ignored_entry(&self, key: String, value: &Entry) -> Result<(), ConfigureContentError>;
    fn remove_watch_entry(&self, key: String) -> Result<(), ConfigureContentError>;
    fn remove_ignored_entry(&self, key: String) -> Result<(), ConfigureContentError>;
    fn watch_to_ignored_entry(&self, key: String) -> Result<(), ConfigureContentError>;
    fn ignored_to_watch_entry(&self, key: String) -> Result<(), ConfigureContentError>;
}
