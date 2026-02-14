use std::fmt;

/// Database operation errors
#[derive(Debug, Clone)]
pub enum Error {
    NotFound(String),
    AlreadyExists(String),
    InvalidData(String),
    DatabaseError(String),
    InternalError,
    Timeout,
    ConnectionError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            Error::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Error::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Error::InternalError => write!(f, "Internal error"),
            Error::Timeout => write!(f, "Operation timeout"),
            Error::ConnectionError => write!(f, "Connection error"),
        }
    }
}

impl std::error::Error for Error {}

/// Trait for entities that have an ID
pub trait Identifiable {
    fn id(&self) -> &str;
}

/// Trait for entities that have tags
pub trait Tagged {
    /// Check if the item has a specific tag
    fn has_tag(&self, tag: &str) -> bool;
    
    /// Get all tags as a vector of string references
    fn get_tags(&self) -> Vec<&str>;
}

/// Database trait for CRUD operations
/// T must implement Clone and Identifiable
pub trait Database<T>
where
    T: Clone + Identifiable + Send + Sync,
{
    /// Insert a new item into the database
    /// Returns the inserted item or an error if it already exists
    fn insert(&mut self, data: T) -> Result<T, Error>;

    /// Update an existing item by ID
    /// Returns the updated item or an error if not found
    fn update(&mut self, id: &str, data: T) -> Result<T, Error>;

    /// Delete an item by ID
    /// Returns the deleted item or an error if not found
    fn delete(&mut self, id: &str) -> Result<T, Error>;

    /// Get a single item by ID
    /// Returns the item or an error if not found
    fn get(&self, id: &str) -> Result<T, Error>;

    /// Get all items with pagination
    /// Returns a vector of items or an error
    fn get_all(&self, limit: usize, offset: usize) -> Result<Vec<T>, Error>;

    /// Get all items by tag with pagination
    /// Returns a vector of items or an error
    /// Note: For better performance, implement this with a Tagged trait bound
    fn get_all_by_tag(&self, tag: &str, limit: usize, offset: usize) -> Result<Vec<T>, Error>;
}

/// Specialized database trait for tagged items
pub trait TaggedDatabase<T>
where
    T: Clone + Identifiable + Tagged + Send + Sync,
{
    /// Get all items by tag with pagination (optimized for tagged items)
    fn get_all_by_tag(&self, tag: &str, limit: usize, offset: usize) -> Result<Vec<T>, Error>;
}
