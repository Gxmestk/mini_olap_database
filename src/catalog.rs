//! # Catalog Module
//!
//! This module provides the Catalog structure which serves as the central metadata
//! repository for the Mini Rust OLAP database. The Catalog manages all tables
//! in the database, providing methods to register, retrieve, list, and drop tables.
//!
//! The Catalog acts as the entry point for all database operations, maintaining
//! a collection of tables and their associated metadata.

use crate::error::{DatabaseError, Result};
use crate::Table;
use std::collections::HashMap;

/// Represents the database catalog containing all tables.
///
/// The Catalog is the central metadata repository that tracks all tables
/// in the database. It provides a simple interface for managing the
/// collection of tables, including registration, retrieval, and listing.
///
/// # Example
///
/// ```ignore
/// use mini_rust_olap::catalog::Catalog;
/// use mini_rust_olap::Table;
///
/// let mut catalog = Catalog::new();
///
/// // Create a new table
/// let table = Table::new("users".to_string());
///
/// // Register it in the catalog
/// catalog.register_table(table)?;
///
/// // Check if table exists
/// assert!(catalog.table_exists("users"));
///
/// // Retrieve the table
/// let retrieved = catalog.get_table("users")?;
/// assert_eq!(retrieved.name(), "users");
///
/// // List all tables
/// let tables = catalog.list_tables();
/// assert_eq!(tables, vec!["users"]);
/// ```
#[derive(Clone)]
pub struct Catalog {
    /// Mapping from table name to Table object
    tables: HashMap<String, Table>,
}

impl Catalog {
    /// Creates a new empty catalog.
    ///
    /// # Returns
    ///
    /// A new Catalog instance with no tables
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    /// Registers a table in the catalog.
    ///
    /// This method adds a table to the catalog, checking that no table
    /// with the same name already exists.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to register
    ///
    /// # Returns
    ///
    /// Returns an error if a table with the same name already exists
    pub fn register_table(&mut self, table: Table) -> Result<()> {
        let table_name = table.name().to_string();

        if self.tables.contains_key(&table_name) {
            return Err(DatabaseError::catalog_error(format!(
                "Table '{}' already exists in catalog",
                table_name
            )));
        }

        self.tables.insert(table_name, table);
        Ok(())
    }

    /// Retrieves a table by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to retrieve
    ///
    /// # Returns
    ///
    /// A reference to the table, or an error if not found
    pub fn get_table(&self, name: &str) -> Result<&Table> {
        self.tables.get(name).ok_or_else(|| {
            DatabaseError::catalog_error(format!("Table '{}' not found in catalog", name))
        })
    }

    /// Retrieves a mutable reference to a table by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to retrieve
    ///
    /// # Returns
    ///
    /// A mutable reference to the table, or an error if not found
    pub fn get_table_mut(&mut self, name: &str) -> Result<&mut Table> {
        self.tables.get_mut(name).ok_or_else(|| {
            DatabaseError::catalog_error(format!("Table '{}' not found in catalog", name))
        })
    }

    /// Checks if a table with the given name exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The table name to check
    ///
    /// # Returns
    ///
    /// true if the table exists, false otherwise
    pub fn table_exists(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }

    /// Returns a list of all table names in the catalog.
    ///
    /// The names are returned in arbitrary order (HashMap iteration order).
    ///
    /// # Returns
    ///
    /// A vector of table names
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    /// Returns a list of all table names sorted alphabetically.
    ///
    /// This is useful for displaying table names in a consistent order.
    ///
    /// # Returns
    ///
    /// A sorted vector of table names
    pub fn list_tables_sorted(&self) -> Vec<String> {
        let mut names = self.list_tables();
        names.sort();
        names
    }

    /// Drops (removes) a table from the catalog.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to drop
    ///
    /// # Returns
    ///
    /// An error if the table doesn't exist
    pub fn drop_table(&mut self, name: &str) -> Result<()> {
        self.tables.remove(name).ok_or_else(|| {
            DatabaseError::catalog_error(format!(
                "Cannot drop table '{}': table not found in catalog",
                name
            ))
        })?;
        Ok(())
    }

    /// Returns the number of tables in the catalog.
    ///
    /// # Returns
    ///
    /// The count of tables
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Returns a reference to the internal tables map.
    ///
    /// This provides direct access to all tables for advanced operations.
    ///
    /// # Returns
    ///
    /// A reference to the tables HashMap
    pub fn tables(&self) -> &HashMap<String, Table> {
        &self.tables
    }

    /// Clears all tables from the catalog.
    ///
    /// This removes all tables, effectively resetting the catalog to empty.
    pub fn clear(&mut self) {
        self.tables.clear();
    }

    /// Renames a table in the catalog.
    ///
    /// # Arguments
    ///
    /// * `old_name` - The current name of the table
    /// * `new_name` - The new name for the table
    ///
    /// # Returns
    ///
    /// An error if the old table doesn't exist or new name already exists
    pub fn rename_table(&mut self, old_name: &str, new_name: String) -> Result<()> {
        if !self.table_exists(old_name) {
            return Err(DatabaseError::catalog_error(format!(
                "Cannot rename table '{}': table not found",
                old_name
            )));
        }

        if self.table_exists(&new_name) {
            return Err(DatabaseError::catalog_error(format!(
                "Cannot rename to '{}': table already exists",
                new_name
            )));
        }

        // Remove the table with old name and reinsert with new name
        let table = self.tables.remove(old_name).unwrap();
        // Note: Table doesn't have a set_name method, so we'd need to clone
        // or implement one. For now, we'll just move it and it keeps its internal name.
        // In a real implementation, we'd want to update the table's internal name too.
        self.tables.insert(new_name, table);

        Ok(())
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Catalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Catalog")?;
        writeln!(f, "  Tables: {}", self.table_count())?;

        if self.table_count() > 0 {
            writeln!(f, "  Table names: {}", self.list_tables().join(", "))?;
        }

        Ok(())
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column::{Column, IntColumn};
    use crate::types::Value;

    /// Test creating a new catalog
    #[test]
    fn test_catalog_new() {
        let catalog = Catalog::new();
        assert_eq!(catalog.table_count(), 0);
        assert!(catalog.list_tables().is_empty());
    }

    /// Test creating a catalog with Default trait
    #[test]
    fn test_catalog_default() {
        let catalog = Catalog::default();
        assert_eq!(catalog.table_count(), 0);
    }

    /// Test registering a table
    #[test]
    fn test_register_table() {
        let mut catalog = Catalog::new();
        let table = Table::new("users".to_string());

        assert!(catalog.register_table(table).is_ok());
        assert_eq!(catalog.table_count(), 1);
        assert!(catalog.table_exists("users"));
    }

    /// Test registering a table with duplicate name
    #[test]
    fn test_register_duplicate_table() {
        let mut catalog = Catalog::new();
        let table1 = Table::new("users".to_string());
        let table2 = Table::new("users".to_string());

        assert!(catalog.register_table(table1).is_ok());
        let result = catalog.register_table(table2);

        assert!(result.is_err());
        let error_msg = match result {
            Err(e) => e.to_string(),
            Ok(_) => panic!("Expected error"),
        };
        assert!(error_msg.contains("already exists"));
    }

    /// Test getting an existing table
    #[test]
    fn test_get_table_exists() {
        let mut catalog = Catalog::new();
        let table = Table::new("users".to_string());

        catalog.register_table(table).unwrap();

        let retrieved = catalog.get_table("users");
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().name(), "users");
    }

    /// Test getting a non-existent table
    #[test]
    fn test_get_table_not_exists() {
        let catalog = Catalog::new();

        let result = catalog.get_table("nonexistent");
        assert!(result.is_err());
        let error_msg = match result {
            Err(e) => format!("{}", e),
            Ok(_) => panic!("Expected error"),
        };
        assert!(error_msg.contains("not found"));
    }

    /// Test getting a mutable reference to a table
    #[test]
    fn test_get_table_mut() {
        let mut catalog = Catalog::new();
        let table = Table::new("users".to_string());

        catalog.register_table(table).unwrap();

        let table_mut = catalog.get_table_mut("users");
        assert!(table_mut.is_ok());
        assert_eq!(table_mut.unwrap().name(), "users");
    }

    /// Test table_exists for existing table
    #[test]
    fn test_table_exists_true() {
        let mut catalog = Catalog::new();
        let table = Table::new("users".to_string());

        catalog.register_table(table).unwrap();

        assert!(catalog.table_exists("users"));
    }

    /// Test table_exists for non-existent table
    #[test]
    fn test_table_exists_false() {
        let catalog = Catalog::new();

        assert!(!catalog.table_exists("users"));
    }

    /// Test list_tables
    #[test]
    fn test_list_tables() {
        let mut catalog = Catalog::new();

        let table1 = Table::new("users".to_string());
        let table2 = Table::new("products".to_string());
        let table3 = Table::new("orders".to_string());

        catalog.register_table(table1).unwrap();
        catalog.register_table(table2).unwrap();
        catalog.register_table(table3).unwrap();

        let tables = catalog.list_tables();
        assert_eq!(tables.len(), 3);
        assert!(tables.contains(&"users".to_string()));
        assert!(tables.contains(&"products".to_string()));
        assert!(tables.contains(&"orders".to_string()));
    }

    /// Test list_tables for empty catalog
    #[test]
    fn test_list_tables_empty() {
        let catalog = Catalog::new();

        let tables = catalog.list_tables();
        assert!(tables.is_empty());
    }

    /// Test list_tables_sorted
    #[test]
    fn test_list_tables_sorted() {
        let mut catalog = Catalog::new();

        let table1 = Table::new("zebra".to_string());
        let table2 = Table::new("apple".to_string());
        let table3 = Table::new("middle".to_string());

        catalog.register_table(table1).unwrap();
        catalog.register_table(table2).unwrap();
        catalog.register_table(table3).unwrap();

        let tables = catalog.list_tables_sorted();
        assert_eq!(tables.len(), 3);
        assert_eq!(tables[0], "apple");
        assert_eq!(tables[1], "middle");
        assert_eq!(tables[2], "zebra");
    }

    /// Test dropping an existing table
    #[test]
    fn test_drop_table() {
        let mut catalog = Catalog::new();
        let table = Table::new("users".to_string());

        catalog.register_table(table).unwrap();
        assert_eq!(catalog.table_count(), 1);

        assert!(catalog.drop_table("users").is_ok());
        assert_eq!(catalog.table_count(), 0);
        assert!(!catalog.table_exists("users"));
    }

    /// Test dropping a non-existent table
    #[test]
    fn test_drop_table_not_exists() {
        let mut catalog = Catalog::new();

        let result = catalog.drop_table("nonexistent");
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not found"));
    }

    /// Test table_count
    #[test]
    fn test_table_count() {
        let mut catalog = Catalog::new();

        assert_eq!(catalog.table_count(), 0);

        catalog
            .register_table(Table::new("t1".to_string()))
            .unwrap();
        assert_eq!(catalog.table_count(), 1);

        catalog
            .register_table(Table::new("t2".to_string()))
            .unwrap();
        assert_eq!(catalog.table_count(), 2);

        catalog.drop_table("t1").unwrap();
        assert_eq!(catalog.table_count(), 1);
    }

    /// Test tables() method
    #[test]
    fn test_tables_method() {
        let mut catalog = Catalog::new();

        catalog
            .register_table(Table::new("users".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("products".to_string()))
            .unwrap();

        let tables = catalog.tables();
        assert_eq!(tables.len(), 2);
        assert!(tables.contains_key("users"));
        assert!(tables.contains_key("products"));
    }

    /// Test clear method
    #[test]
    fn test_clear() {
        let mut catalog = Catalog::new();

        catalog
            .register_table(Table::new("t1".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("t2".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("t3".to_string()))
            .unwrap();

        assert_eq!(catalog.table_count(), 3);

        catalog.clear();

        assert_eq!(catalog.table_count(), 0);
        assert!(catalog.list_tables().is_empty());
    }

    /// Test rename_table
    #[test]
    fn test_rename_table() {
        let mut catalog = Catalog::new();
        let table = Table::new("old_name".to_string());

        catalog.register_table(table).unwrap();
        assert!(catalog.table_exists("old_name"));
        assert!(!catalog.table_exists("new_name"));

        let result = catalog.rename_table("old_name", "new_name".to_string());
        assert!(result.is_ok());
        assert!(!catalog.table_exists("old_name"));
        assert!(catalog.table_exists("new_name"));
    }

    /// Test rename_table with non-existent old name
    #[test]
    fn test_rename_table_old_not_exists() {
        let mut catalog = Catalog::new();

        let result = catalog.rename_table("nonexistent", "new_name".to_string());
        assert!(result.is_err());
        let error_msg = match result {
            Err(e) => format!("{}", e),
            Ok(_) => panic!("Expected error"),
        };
        assert!(error_msg.contains("not found"));
    }

    /// Test rename_table with existing new name
    #[test]
    fn test_rename_table_new_exists() {
        let mut catalog = Catalog::new();

        catalog
            .register_table(Table::new("table1".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("table2".to_string()))
            .unwrap();

        let result = catalog.rename_table("table1", "table2".to_string());
        assert!(result.is_err());
        let error_msg = match result {
            Err(e) => format!("{}", e),
            Ok(_) => panic!("Expected error"),
        };
        assert!(error_msg.contains("already exists"));
    }

    /// Test catalog display
    #[test]
    fn test_catalog_display() {
        let mut catalog = Catalog::new();

        catalog
            .register_table(Table::new("users".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("products".to_string()))
            .unwrap();

        let display = format!("{}", catalog);
        assert!(display.contains("Catalog"));
        assert!(display.contains("Tables: 2"));
        assert!(display.contains("Table names"));
    }

    /// Test catalog display for empty catalog
    #[test]
    fn test_catalog_display_empty() {
        let catalog = Catalog::new();

        let display = format!("{}", catalog);
        assert!(display.contains("Catalog"));
        assert!(display.contains("Tables: 0"));
    }

    /// Test integration with Table operations
    #[test]
    fn test_catalog_with_table_operations() {
        let mut catalog = Catalog::new();

        // Create a table with data
        let mut table = Table::new("test_table".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(1));
        let _ = col.push_value(Value::Int64(2));
        let _ = col.push_value(Value::Int64(3));

        table.add_column("id".to_string(), Box::new(col)).unwrap();

        // Register in catalog
        catalog.register_table(table).unwrap();

        // Retrieve and verify
        let retrieved = catalog.get_table("test_table").unwrap();
        assert_eq!(retrieved.name(), "test_table");
        assert_eq!(retrieved.row_count(), 3);
        assert_eq!(retrieved.column_count(), 1);

        // Get mutable reference and modify
        let table_mut = catalog.get_table_mut("test_table").unwrap();
        let _ = table_mut.add_row(vec!["4".to_string()]);

        // Verify modification
        let retrieved = catalog.get_table("test_table").unwrap();
        assert_eq!(retrieved.row_count(), 4);
    }

    /// Test multiple operations sequence
    #[test]
    fn test_catalog_operations_sequence() {
        let mut catalog = Catalog::new();

        // Add tables
        catalog
            .register_table(Table::new("t1".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("t2".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("t3".to_string()))
            .unwrap();

        assert_eq!(catalog.table_count(), 3);

        // Rename
        catalog
            .rename_table("t2", "renamed_t2".to_string())
            .unwrap();
        assert!(catalog.table_exists("renamed_t2"));
        assert!(!catalog.table_exists("t2"));

        // Drop
        catalog.drop_table("t1").unwrap();
        assert_eq!(catalog.table_count(), 2);

        // List
        let tables = catalog.list_tables();
        assert_eq!(tables.len(), 2);

        // Clear
        catalog.clear();
        assert_eq!(catalog.table_count(), 0);
    }

    /// Test clone catalog
    #[test]
    fn test_catalog_clone() {
        let mut catalog = Catalog::new();

        catalog
            .register_table(Table::new("users".to_string()))
            .unwrap();
        catalog
            .register_table(Table::new("products".to_string()))
            .unwrap();

        let cloned = catalog.clone();
        assert_eq!(cloned.table_count(), 2);
        assert!(cloned.table_exists("users"));
        assert!(cloned.table_exists("products"));
    }
}
