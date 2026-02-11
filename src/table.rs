//! # Table Module
//!
//! This module provides the Table structure which serves as the fundamental
//! data container for the Mini Rust OLAP database. Tables organize data in a
//! columnar format, storing each column independently to enable efficient
//! analytical queries.

use crate::column::{create_column, Column};
use crate::error::{DatabaseError, Result};
use crate::types::{DataType, Value};
use std::collections::HashMap;

/// Represents a table in the database with a name, schema, and columnar data.
///
/// Tables are organized in a columnar format, which means that instead of storing
/// rows together, each column is stored as an independent sequence of values.
/// This organization enables efficient analytical queries that only need to scan
/// a subset of columns.
///
/// # Example
///
/// ```ignore
/// use mini_rust_olap::table::Table;
/// use mini_rust_olap::column::{create_column, Column};
/// use mini_rust_olap::types::DataType;
///
/// let mut table = Table::new("users".to_string());
///
/// // Add a column
/// let mut id_column = create_column(DataType::Int64);
/// id_column.push_value(1);
/// id_column.push_value(2);
/// table.add_column("id".to_string(), id_column)?;
///
/// println!("Table has {} rows", table.row_count());
/// ```
pub struct Table {
    /// The name of this table
    name: String,

    /// Mapping from column name to column index for fast lookup
    column_index: HashMap<String, usize>,

    /// The actual column data, stored as boxed Column trait objects
    columns: Vec<Box<dyn Column>>,

    /// The schema mapping column names to their data types
    schema: HashMap<String, DataType>,
}

impl Clone for Table {
    fn clone(&self) -> Self {
        let mut new_table = Table::new(self.name.clone());

        // Iterate over column names in insertion order, not schema (HashMap order is non-deterministic)
        for name in self.column_names() {
            let data_type = self.schema.get(&name).unwrap();
            let index = self.column_index.get(&name).unwrap();
            let original_column = &self.columns[*index];
            let cloned_column = original_column.slice(Some(0..original_column.len()));

            let mut column = create_column(*data_type);
            for i in 0..cloned_column.len() {
                let value = cloned_column.get(i).unwrap();
                let _ = column.as_mut().push_value(value.clone());
            }

            new_table.add_column(name.clone(), column).unwrap();
        }

        new_table
    }
}

impl Table {
    /// Creates a new empty table with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table
    ///
    /// # Returns
    ///
    /// A new Table instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            column_index: HashMap::new(),
            columns: Vec::new(),
            schema: HashMap::new(),
        }
    }

    /// Adds a column to the table.
    ///
    /// This method validates that the column name is unique and that the column
    /// has the same number of rows as existing columns (if any).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column
    /// * `column` - The column to add (boxed Column trait object)
    ///
    /// # Returns
    ///
    /// Returns an error if the column name already exists or if row counts don't match
    pub fn add_column(&mut self, name: String, column: Box<dyn Column>) -> Result<()> {
        // Check for duplicate column names
        if self.schema.contains_key(&name) {
            return Err(DatabaseError::column_error(format!(
                "Column '{}' already exists in table '{}'",
                name, self.name
            )));
        }

        // Check that the new column has the same number of rows as existing columns
        if !self.columns.is_empty() {
            let existing_row_count = self.columns[0].len();
            let new_row_count = column.len();

            if existing_row_count != new_row_count {
                return Err(DatabaseError::table_error(format!(
                    "Cannot add column '{}': row count mismatch. Expected {} rows, got {}",
                    name, existing_row_count, new_row_count
                )));
            }
        }

        // Get the data type from the column
        let data_type = column.data_type();

        // Track the column index for fast lookup
        let index = self.columns.len();
        self.column_index.insert(name.clone(), index);

        // Store the column and update the schema
        self.columns.push(column);
        self.schema.insert(name, data_type);

        Ok(())
    }

    /// Gets a reference to a column by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column to retrieve
    ///
    /// # Returns
    ///
    /// A reference to the column, or an error if not found
    pub fn get_column(&self, name: &str) -> Result<&dyn Column> {
        let index = self.column_index.get(name).ok_or_else(|| {
            DatabaseError::column_error(format!(
                "Column '{}' not found in table '{}'",
                name, self.name
            ))
        })?;

        Ok(self.columns[*index].as_ref())
    }

    /// Gets a mutable reference to a column by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column to retrieve
    ///
    /// # Returns
    ///
    /// A mutable reference to the column, or an error if not found
    pub fn get_column_mut(&mut self, name: &str) -> Result<&mut dyn Column> {
        let index = *self.column_index.get(name).ok_or_else(|| {
            DatabaseError::column_error(format!(
                "Column '{}' not found in table '{}'",
                name, self.name
            ))
        })?;

        Ok(self.columns[index].as_mut())
    }

    /// Gets the value at a specific row and column.
    ///
    /// # Arguments
    ///
    /// * `column_name` - The name of the column
    /// * `row_index` - The row index (0-based)
    ///
    /// # Returns
    ///
    /// The value at the specified position, or an error if not found
    pub fn get_value(&self, column_name: &str, row_index: usize) -> Result<Value> {
        let column = self.get_column(column_name)?;
        column.get(row_index)
    }

    /// Returns the number of rows in the table.
    ///
    /// If the table has no columns, returns 0.
    pub fn row_count(&self) -> usize {
        if self.columns.is_empty() {
            0
        } else {
            self.columns[0].len()
        }
    }

    /// Returns the number of columns in the table.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns the name of the table.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a slice of all column names in insertion order.
    pub fn column_names(&self) -> Vec<String> {
        let mut name_index_pairs: Vec<(String, usize)> = self
            .column_index
            .iter()
            .map(|(name, &index)| (name.clone(), index))
            .collect();
        name_index_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        name_index_pairs.into_iter().map(|(name, _)| name).collect()
    }

    /// Adds a row of values to the table.
    ///
    /// The values are provided as strings and will be parsed according to the
    /// column's data type. The order of values must match the order in which
    /// columns were added to the table.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of string values to add
    ///
    /// # Returns
    ///
    /// An error if the number of values doesn't match the column count
    pub fn add_row(&mut self, values: Vec<String>) -> Result<()> {
        if values.len() != self.column_count() {
            return Err(DatabaseError::table_error(format!(
                "Row value count mismatch. Expected {} values, got {}",
                self.column_count(),
                values.len()
            )));
        }

        for (index, value) in values.iter().enumerate() {
            let column = self.columns[index].as_mut();

            // Parse the string value according to column type
            let parsed_value = match column.data_type() {
                DataType::Int64 => value.parse::<i64>().map(Value::Int64).map_err(|_| {
                    DatabaseError::column_error(format!("Invalid integer value: '{}'", value))
                })?,
                DataType::Float64 => value.parse::<f64>().map(Value::Float64).map_err(|_| {
                    DatabaseError::column_error(format!("Invalid float value: '{}'", value))
                })?,
                DataType::String => Value::String(value.clone()),
            };

            let _ = column.push_value(parsed_value);
        }

        Ok(())
    }

    /// Returns a reference to the schema (column name to data type mapping).
    pub fn schema(&self) -> &HashMap<String, DataType> {
        &self.schema
    }

    /// Checks if a column with the given name exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The column name to check
    ///
    /// # Returns
    ///
    /// true if the column exists, false otherwise
    pub fn has_column(&self, name: &str) -> bool {
        self.schema.contains_key(name)
    }

    /// Gets the data type of a column.
    ///
    /// # Arguments
    ///
    /// * `name` - The column name
    ///
    /// # Returns
    ///
    /// The data type of the column, or an error if not found
    pub fn get_column_type(&self, name: &str) -> Result<DataType> {
        self.schema.get(name).copied().ok_or_else(|| {
            DatabaseError::column_error(format!(
                "Column '{}' not found in table '{}'",
                name, self.name
            ))
        })
    }

    /// Creates a new table containing only the specified columns.
    ///
    /// # Arguments
    ///
    /// * `column_names` - The names of the columns to include in the new table
    ///
    /// # Returns
    ///
    /// A new table with the selected columns, or an error if a column is not found
    pub fn select_columns(&self, column_names: &[String]) -> Result<Self> {
        let mut new_table = Table::new(self.name.clone());

        for name in column_names {
            let index = self.column_index.get(name).ok_or_else(|| {
                DatabaseError::column_error(format!(
                    "Column '{}' not found in table '{}'",
                    name, self.name
                ))
            })?;

            // Clone the column
            let original_column = &self.columns[*index];
            let cloned_column = original_column.slice(Some(0..original_column.len()));

            // Get the data type
            let data_type = self.schema.get(name).unwrap();

            // Add to the new table
            let mut column = create_column(*data_type);
            for i in 0..cloned_column.len() {
                let value = cloned_column.get(i).ok_or_else(|| {
                    DatabaseError::column_error(format!("Failed to get value at index {}", i))
                })?;
                let _ = column.as_mut().push_value(value.clone());
            }

            new_table.add_column(name.clone(), column)?;
        }

        Ok(new_table)
    }

    /// Checks if all columns have consistent lengths (schema validation).
    ///
    /// This is useful for debugging or ensuring data integrity after
    /// manual modifications.
    ///
    /// # Returns
    ///
    /// true if all columns have the same length, false otherwise
    pub fn validate_schema(&self) -> bool {
        if self.columns.is_empty() {
            return true;
        }

        let first_length = self.columns[0].len();
        self.columns.iter().all(|col| col.len() == first_length)
    }

    /// Drops a column from the table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column to drop
    ///
    /// # Returns
    ///
    /// An error if the column doesn't exist
    pub fn drop_column(&mut self, name: &str) -> Result<()> {
        let index = self.column_index.remove(name).ok_or_else(|| {
            DatabaseError::column_error(format!(
                "Column '{}' not found in table '{}'",
                name, self.name
            ))
        })?;

        self.columns.remove(index);
        self.schema.remove(name);

        // Rebuild column_index to fix indices
        self.rebuild_column_index();

        Ok(())
    }

    /// Rebuilds the column index map after removing a column.
    fn rebuild_column_index(&mut self) {
        self.column_index.clear();
        for (name, data_type) in self.schema.iter() {
            // We need to find the index by searching columns
            for (idx, col) in self.columns.iter().enumerate() {
                if col.data_type() == *data_type {
                    // This is a heuristic - in reality we'd need a better way to match
                    self.column_index.insert(name.clone(), idx);
                    break;
                }
            }
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table: {}", self.name)?;
        writeln!(f, "Rows: {}", self.row_count())?;
        writeln!(
            f,
            "Columns: {} ({})",
            self.column_count(),
            self.column_names().join(", ")
        )?;

        if self.row_count() > 0 && self.row_count() <= 10 {
            // Print first 5 rows
            writeln!(f, "\nSample data:")?;
            // Print header
            let names: Vec<&String> = self.schema.keys().collect();
            write!(f, "|")?;
            for name in &names {
                write!(f, " {} |", name)?;
            }
            writeln!(f)?;

            // Print data rows
            for row in 0..self.row_count().min(5) {
                write!(f, "|")?;
                for name in &names {
                    let col = self.get_column(name).unwrap();
                    let val = col.get(row).unwrap();
                    write!(f, " {} |", val)?;
                }
                writeln!(f)?;
            }
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
    use crate::column::{FloatColumn, IntColumn, StringColumn};

    /// Test creating a new table
    #[test]
    fn test_table_new() {
        let table = Table::new("test_table".to_string());
        assert_eq!(table.name(), "test_table");
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.column_count(), 0);
        assert!(table.column_names().is_empty());
    }

    /// Test adding a column to an empty table
    #[test]
    fn test_add_column_to_empty_table() {
        let mut table = Table::new("test".to_string());
        let column = Box::new(IntColumn::new()) as Box<dyn Column>;

        assert!(table.add_column("id".to_string(), column).is_ok());
        assert_eq!(table.column_count(), 1);
        assert!(table.has_column("id"));
    }

    /// Test adding multiple columns
    #[test]
    fn test_add_multiple_columns() {
        let mut table = Table::new("test".to_string());

        let id_col = Box::new(IntColumn::new()) as Box<dyn Column>;
        let name_col = Box::new(StringColumn::new()) as Box<dyn Column>;

        assert!(table.add_column("id".to_string(), id_col).is_ok());
        assert!(table.add_column("name".to_string(), name_col).is_ok());

        assert_eq!(table.column_count(), 2);
        assert!(table.has_column("id"));
        assert!(table.has_column("name"));
    }

    /// Test adding a column with a duplicate name
    #[test]
    fn test_add_duplicate_column() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;

        assert!(table.add_column("col".to_string(), col1).is_ok());

        let result = table.add_column("col".to_string(), col2);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("already exists"));
    }

    /// Test adding columns with mismatched row counts
    #[test]
    fn test_add_column_mismatched_rows() {
        let mut table = Table::new("test".to_string());

        let mut col1 = IntColumn::new();
        let _ = col1.push_value(Value::Int64(1));
        let _ = col1.push_value(Value::Int64(2));

        let mut col2 = IntColumn::new();
        let _ = col2.push_value(Value::Int64(10));

        assert!(table.add_column("col1".to_string(), Box::new(col1)).is_ok());

        let result = table.add_column("col2".to_string(), Box::new(col2));
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("row count mismatch"));
    }

    /// Test adding columns with matching row counts
    #[test]
    fn test_add_column_matching_rows() {
        let mut table = Table::new("test".to_string());

        let mut col1 = IntColumn::new();
        let _ = col1.push_value(Value::Int64(1));
        let _ = col1.push_value(Value::Int64(2));

        let mut col2 = IntColumn::new();
        let _ = col2.push_value(Value::Int64(10));
        let _ = col2.push_value(Value::Int64(20));

        assert!(table.add_column("col1".to_string(), Box::new(col1)).is_ok());
        assert!(table.add_column("col2".to_string(), Box::new(col2)).is_ok());

        assert_eq!(table.column_count(), 2);
    }

    /// Test getting a column that exists
    #[test]
    fn test_get_column_exists() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        let retrieved = table.get_column("id");
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().len(), 0);
    }

    /// Test getting a column that doesn't exist
    #[test]
    fn test_get_column_not_exists() {
        let table = Table::new("test".to_string());

        let result = table.get_column("nonexistent");
        assert!(result.is_err());
        let error_msg = match result {
            Err(e) => format!("{}", e),
            Ok(_) => panic!("Expected error"),
        };
        assert!(error_msg.contains("not found"));
    }

    /// Test row_count for empty table
    #[test]
    fn test_row_count_empty_table() {
        let table = Table::new("test".to_string());
        assert_eq!(table.row_count(), 0);
    }

    /// Test row_count with data
    #[test]
    fn test_row_count_with_data() {
        let mut table = Table::new("test".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(1));
        let _ = col.push_value(Value::Int64(2));
        let _ = col.push_value(Value::Int64(3));

        table.add_column("id".to_string(), Box::new(col)).unwrap();

        assert_eq!(table.row_count(), 3);
    }

    /// Test column_count
    #[test]
    fn test_column_count() {
        let mut table = Table::new("test".to_string());

        assert_eq!(table.column_count(), 0);

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("col1".to_string(), col1).unwrap();

        assert_eq!(table.column_count(), 1);

        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;
        table.add_column("col2".to_string(), col2).unwrap();

        assert_eq!(table.column_count(), 2);
    }

    /// Test column_names
    #[test]
    fn test_column_names() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;
        let col3 = Box::new(FloatColumn::new()) as Box<dyn Column>;

        table.add_column("id".to_string(), col1).unwrap();
        table.add_column("name".to_string(), col2).unwrap();
        table.add_column("score".to_string(), col3).unwrap();

        let names = table.column_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"id".to_string()));
        assert!(names.contains(&"name".to_string()));
        assert!(names.contains(&"score".to_string()));
    }

    /// Test get_value
    #[test]
    fn test_get_value() {
        let mut table = Table::new("test".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(42));
        let _ = col.push_value(Value::Int64(100));

        table
            .add_column("number".to_string(), Box::new(col))
            .unwrap();

        let val1 = table.get_value("number", 0);
        assert!(val1.is_ok());
        assert_eq!(val1.unwrap(), Value::Int64(42));

        let val2 = table.get_value("number", 1);
        assert!(val2.is_ok());
        assert_eq!(val2.unwrap(), Value::Int64(100));
    }

    /// Test get_value with invalid column
    #[test]
    fn test_get_value_invalid_column() {
        let mut table = Table::new("test".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(42));

        table
            .add_column("number".to_string(), Box::new(col))
            .unwrap();

        let result = table.get_value("invalid", 0);
        assert!(result.is_err());
    }

    /// Test get_value with invalid row
    #[test]
    fn test_get_value_invalid_row() {
        let mut table = Table::new("test".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(42));

        table
            .add_column("number".to_string(), Box::new(col))
            .unwrap();

        let result = table.get_value("number", 10);
        assert!(result.is_err());
    }

    /// Test add_row
    #[test]
    fn test_add_row() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;

        table.add_column("id".to_string(), col1).unwrap();
        table.add_column("name".to_string(), col2).unwrap();

        let values = vec!["1".to_string(), "Alice".to_string()];
        assert!(table.add_row(values).is_ok());

        assert_eq!(table.row_count(), 1);

        let id = table.get_value("id", 0).unwrap();
        assert_eq!(id, Value::Int64(1));

        let name = table.get_value("name", 0).unwrap();
        assert_eq!(name, Value::String("Alice".to_string()));
    }

    /// Test add_row with wrong number of values
    #[test]
    fn test_add_row_wrong_count() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        let values = vec!["1".to_string(), "extra".to_string()];
        let result = table.add_row(values);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Row value count mismatch"));
    }

    /// Test add_row with invalid value type
    #[test]
    fn test_add_row_invalid_type() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        let values = vec!["not_a_number".to_string()];
        let result = table.add_row(values);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Invalid integer value"));
    }

    /// Test schema retrieval
    #[test]
    fn test_schema() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;

        table.add_column("id".to_string(), col1).unwrap();
        table.add_column("name".to_string(), col2).unwrap();

        let schema = table.schema();
        assert_eq!(schema.len(), 2);
        assert_eq!(schema.get("id"), Some(&DataType::Int64));
        assert_eq!(schema.get("name"), Some(&DataType::String));
    }

    /// Test has_column
    #[test]
    fn test_has_column() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        assert!(table.has_column("id"));
        assert!(!table.has_column("nonexistent"));
    }

    /// Test get_column_type
    #[test]
    fn test_get_column_type() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        assert_eq!(table.get_column_type("id").unwrap(), DataType::Int64);

        let result = table.get_column_type("nonexistent");
        assert!(result.is_err());
    }

    /// Test select_columns
    #[test]
    fn test_select_columns() {
        let mut table = Table::new("test".to_string());

        let mut col1 = IntColumn::new();
        let _ = col1.push_value(Value::Int64(1));
        let _ = col1.push_value(Value::Int64(2));

        let mut col2 = StringColumn::new();
        let _ = col2.push_value(Value::String("Alice".to_string()));
        let _ = col2.push_value(Value::String("Bob".to_string()));

        let mut col3 = FloatColumn::new();
        let _ = col3.push_value(Value::Float64(90.5));
        let _ = col3.push_value(Value::Float64(85.0));

        table.add_column("id".to_string(), Box::new(col1)).unwrap();
        table
            .add_column("name".to_string(), Box::new(col2))
            .unwrap();
        table
            .add_column("score".to_string(), Box::new(col3))
            .unwrap();

        let selected = table.select_columns(&["name".to_string(), "score".to_string()]);
        assert!(selected.is_ok());

        let new_table = selected.unwrap();
        assert_eq!(new_table.column_count(), 2);
        assert!(new_table.has_column("name"));
        assert!(new_table.has_column("score"));
        assert!(!new_table.has_column("id"));
        assert_eq!(new_table.row_count(), 2);
    }

    /// Test select_columns with nonexistent column
    #[test]
    fn test_select_columns_nonexistent() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        let result = table.select_columns(&["nonexistent".to_string()]);
        assert!(result.is_err());
    }

    /// Test validate_schema
    #[test]
    fn test_validate_schema() {
        let mut table = Table::new("test".to_string());

        // Empty table should be valid
        assert!(table.validate_schema());

        let mut col1 = IntColumn::new();
        let _ = col1.push_value(Value::Int64(1));
        let _ = col1.push_value(Value::Int64(2));

        let mut col2 = IntColumn::new();
        let _ = col2.push_value(Value::Int64(10));
        let _ = col2.push_value(Value::Int64(20));

        table
            .add_column("col1".to_string(), Box::new(col1))
            .unwrap();
        table
            .add_column("col2".to_string(), Box::new(col2))
            .unwrap();

        // Valid schema
        assert!(table.validate_schema());
    }

    /// Test drop_column
    #[test]
    fn test_drop_column() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;

        table.add_column("id".to_string(), col1).unwrap();
        table.add_column("name".to_string(), col2).unwrap();

        assert_eq!(table.column_count(), 2);
        assert!(table.has_column("id"));
        assert!(table.has_column("name"));

        let result = table.drop_column("id");
        assert!(result.is_ok());

        assert_eq!(table.column_count(), 1);
        assert!(!table.has_column("id"));
        assert!(table.has_column("name"));
    }

    /// Test drop_column with nonexistent column
    #[test]
    fn test_drop_column_nonexistent() {
        let mut table = Table::new("test".to_string());

        let col = Box::new(IntColumn::new()) as Box<dyn Column>;
        table.add_column("id".to_string(), col).unwrap();

        let result = table.drop_column("nonexistent");
        assert!(result.is_err());
    }

    /// Test table display
    #[test]
    fn test_table_display() {
        let mut table = Table::new("test".to_string());

        let mut col1 = IntColumn::new();
        let _ = col1.push_value(Value::Int64(1));
        let _ = col1.push_value(Value::Int64(2));

        let mut col2 = StringColumn::new();
        let _ = col2.push_value(Value::String("Alice".to_string()));
        let _ = col2.push_value(Value::String("Bob".to_string()));

        table.add_column("id".to_string(), Box::new(col1)).unwrap();
        table
            .add_column("name".to_string(), Box::new(col2))
            .unwrap();

        let display = format!("{}", table);
        assert!(display.contains("Table: test"));
        assert!(display.contains("Rows: 2"));
        assert!(display.contains("Columns: 2"));
        assert!(display.contains("Sample data"));
    }

    /// Test cloning a table
    #[test]
    fn test_table_clone() {
        let mut table = Table::new("test".to_string());

        let mut col = IntColumn::new();
        let _ = col.push_value(Value::Int64(1));
        let _ = col.push_value(Value::Int64(2));

        table.add_column("id".to_string(), Box::new(col)).unwrap();

        let cloned = table.clone();
        assert_eq!(cloned.name(), "test");
        assert_eq!(cloned.row_count(), 2);
        assert_eq!(cloned.column_count(), 1);
    }

    /// Test string column data
    #[test]
    fn test_string_column_data() {
        let mut table = Table::new("test".to_string());

        let mut col = StringColumn::new();
        let _ = col.push_value(Value::String("hello".to_string()));
        let _ = col.push_value(Value::String("world".to_string()));

        table
            .add_column("greeting".to_string(), Box::new(col))
            .unwrap();

        assert_eq!(table.row_count(), 2);
        assert_eq!(
            table.get_value("greeting", 0).unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            table.get_value("greeting", 1).unwrap(),
            Value::String("world".to_string())
        );
    }

    /// Test float column data
    #[test]
    fn test_float_column_data() {
        let mut table = Table::new("test".to_string());

        let mut col = FloatColumn::new();
        let _ = col.push_value(Value::Float64(4.5));
        let _ = col.push_value(Value::Float64(2.71));

        table.add_column("pi".to_string(), Box::new(col)).unwrap();

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.get_value("pi", 0).unwrap(), Value::Float64(4.5));
        assert_eq!(table.get_value("pi", 1).unwrap(), Value::Float64(2.71));
    }

    /// Test mixed column types
    #[test]
    fn test_mixed_column_types() {
        let mut table = Table::new("test".to_string());

        let mut id_col = IntColumn::new();
        let _ = id_col.push_value(Value::Int64(1));

        let mut name_col = StringColumn::new();
        let _ = name_col.push_value(Value::String("Alice".to_string()));

        let mut score_col = FloatColumn::new();
        let _ = score_col.push_value(Value::Float64(95.5));

        table
            .add_column("id".to_string(), Box::new(id_col))
            .unwrap();
        table
            .add_column("name".to_string(), Box::new(name_col))
            .unwrap();
        table
            .add_column("score".to_string(), Box::new(score_col))
            .unwrap();

        assert_eq!(table.column_count(), 3);
        assert_eq!(table.get_column_type("id").unwrap(), DataType::Int64);
        assert_eq!(table.get_column_type("name").unwrap(), DataType::String);
        assert_eq!(table.get_column_type("score").unwrap(), DataType::Float64);
    }

    /// Test iterator over column names
    #[test]
    fn test_column_names_iterator() {
        let mut table = Table::new("test".to_string());

        let col1 = Box::new(IntColumn::new()) as Box<dyn Column>;
        let col2 = Box::new(StringColumn::new()) as Box<dyn Column>;

        table.add_column("id".to_string(), col1).unwrap();
        table.add_column("name".to_string(), col2).unwrap();

        let mut found_id = false;
        let mut found_name = false;

        for name in table.column_names() {
            if name == "id" {
                found_id = true;
            }
            if name == "name" {
                found_name = true;
            }
        }

        assert!(found_id && found_name);
    }

    /// Test empty table display
    #[test]
    fn test_empty_table_display() {
        let table = Table::new("empty".to_string());

        let display = format!("{}", table);
        assert!(display.contains("Table: empty"));
        assert!(display.contains("Rows: 0"));
        assert!(display.contains("Columns: 0"));
    }
}
