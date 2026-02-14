use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::database::defs::{Database, Error, Identifiable, Tagged, TaggedDatabase};

/// In-memory reference database implementation
/// Uses HashMap for storage with `thread-safe` access via `RwLock`
pub struct ReferenceDatabase<T>
where
    T: Clone + Identifiable + Send + Sync,
{
    storage: Arc<RwLock<HashMap<String, T>>>,
}

impl<T> ReferenceDatabase<T>
where
    T: Clone + Identifiable + Send + Sync,
{
    /// Create a new empty reference database
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new reference database with initial data
    pub fn with_data(data: Vec<T>) -> Self {
        let mut map = HashMap::new();
        for item in data {
            map.insert(item.id().to_string(), item);
        }
        Self {
            storage: Arc::new(RwLock::new(map)),
        }
    }

    /// Get the number of items in the database
    pub fn len(&self) -> usize {
        self.storage.read().unwrap().len()
    }

    /// Check if the database is empty
    pub fn is_empty(&self) -> bool {
        self.storage.read().unwrap().is_empty()
    }
}

impl<T> Default for ReferenceDatabase<T>
where
    T: Clone + Identifiable + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Database<T> for ReferenceDatabase<T>
where
    T: Clone + Identifiable + Send + Sync,
{
    fn insert(&mut self, data: T) -> Result<T, Error> {
        let id = data.id().to_string();
        let mut storage = self.storage.write().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire write lock: {}", e))
        })?;

        if storage.contains_key(&id) {
            return Err(Error::AlreadyExists(format!("Item with id '{}' already exists", id)));
        }

        let cloned = data.clone();
        storage.insert(id, data);
        Ok(cloned)
    }

    fn update(&mut self, id: &str, data: T) -> Result<T, Error> {
        let mut storage = self.storage.write().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire write lock: {}", e))
        })?;

        if !storage.contains_key(id) {
            return Err(Error::NotFound(format!("Item with id '{}' not found", id)));
        }

        // Ensure the ID matches
        if data.id() != id {
            return Err(Error::InvalidData(format!(
                "ID mismatch: provided id '{}' does not match data id '{}'",
                id,
                data.id()
            )));
        }

        let cloned = data.clone();
        storage.insert(id.to_string(), data);
        Ok(cloned)
    }

    fn delete(&mut self, id: &str) -> Result<T, Error> {
        let mut storage = self.storage.write().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire write lock: {}", e))
        })?;

        storage.remove(id)
            .ok_or_else(|| Error::NotFound(format!("Item with id '{}' not found", id)))
    }

    fn get(&self, id: &str) -> Result<T, Error> {
        let storage = self.storage.read().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire read lock: {}", e))
        })?;

        storage.get(id)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("Item with id '{}' not found", id)))
    }

    fn get_all(&self, limit: usize, offset: usize) -> Result<Vec<T>, Error> {
        let storage = self.storage.read().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut items: Vec<T> = storage.values().cloned().collect();
        
        // Sort by ID for consistent ordering
        items.sort_by(|a, b| a.id().cmp(b.id()));
        
        // Apply pagination
        let start = offset.min(items.len());
        let end = (start + limit).min(items.len());
        
        Ok(items[start..end].to_vec())
    }

    fn get_all_by_tag(&self, _tag: &str, _limit: usize, _offset: usize) -> Result<Vec<T>, Error> {
        // Generic implementation - returns empty for non-tagged items
        // Use TaggedDatabase implementation for better performance with tagged items
        // This is a fallback that returns empty - prefer using TaggedDatabase for tagged items
        Ok(Vec::new())
    }
}

/// Implementation of TaggedDatabase for better tag-based queries
impl<T> TaggedDatabase<T> for ReferenceDatabase<T>
where
    T: Clone + Identifiable + Tagged + Send + Sync,
{
    fn get_all_by_tag(&self, tag: &str, limit: usize, offset: usize) -> Result<Vec<T>, Error> {
        let storage = self.storage.read().map_err(|e| {
            Error::DatabaseError(format!("Failed to acquire read lock: {}", e))
        })?;

        let mut items: Vec<T> = storage
            .values()
            .filter(|item| item.has_tag(tag))
            .cloned()
            .collect();
        
        // Sort by ID for consistent ordering
        items.sort_by(|a, b| a.id().cmp(b.id()));
        
        // Apply pagination
        let start = offset.min(items.len());
        let end = (start + limit).min(items.len());
        
        Ok(items[start..end].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct TestItem {
        id: String,
        name: String,
    }

    impl Identifiable for TestItem {
        fn id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn test_insert_and_get() {
        let mut db = ReferenceDatabase::new();
        let item = TestItem {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        let inserted = db.insert(item.clone()).unwrap();
        assert_eq!(inserted, item);

        let retrieved = db.get("1").unwrap();
        assert_eq!(retrieved, item);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut db = ReferenceDatabase::new();
        let item = TestItem {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        db.insert(item.clone()).unwrap();
        let result = db.insert(item);
        assert!(matches!(result, Err(Error::AlreadyExists(_))));
    }

    #[test]
    fn test_update() {
        let mut db = ReferenceDatabase::new();
        let item = TestItem {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        db.insert(item).unwrap();

        let updated = TestItem {
            id: "1".to_string(),
            name: "Updated".to_string(),
        };

        let result = db.update("1", updated.clone()).unwrap();
        assert_eq!(result.name, "Updated");

        let retrieved = db.get("1").unwrap();
        assert_eq!(retrieved, updated);
    }

    #[test]
    fn test_delete() {
        let mut db = ReferenceDatabase::new();
        let item = TestItem {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        db.insert(item.clone()).unwrap();
        let deleted = db.delete("1").unwrap();
        assert_eq!(deleted, item);

        assert!(matches!(db.get("1"), Err(Error::NotFound(_))));
    }

    #[test]
    fn test_get_all() {
        let mut db = ReferenceDatabase::new();
        
        for i in 0..5 {
            let item = TestItem {
                id: i.to_string(),
                name: format!("Item {}", i),
            };
            db.insert(item).unwrap();
        }

        let all = db.get_all(10, 0).unwrap();
        assert_eq!(all.len(), 5);

        let paginated = db.get_all(2, 1).unwrap();
        assert_eq!(paginated.len(), 2);
    }
}
