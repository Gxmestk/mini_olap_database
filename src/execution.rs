//! # Query Execution Engine
//!
//! This module provides the foundation for vectorized query execution.
//! It defines the `Batch` struct for columnar data processing and the
//! `Operator` trait for implementing query operators like Scan, Filter,
//! Project, and GroupBy.

use crate::column::Column;
use crate::types::DataType;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Error type for execution operations
#[derive(Debug)]
pub enum ExecutionError {
    /// Operator is not open
    OperatorNotOpen,
    /// Operator is already open
    OperatorAlreadyOpen,
    /// Schema mismatch between operators
    SchemaMismatch(String),
    /// Schema not found
    SchemaNotFound,
    /// Invalid column index
    InvalidColumnIndex { index: usize, count: usize },
    /// Column not found
    ColumnNotFound(String),
    /// Invalid row index
    InvalidRowIndex { index: usize, count: usize },
    /// IO error during execution
    IoError(std::io::Error),
    /// Custom error message
    Custom(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::OperatorNotOpen => {
                write!(f, "Operator must be opened before calling this operation")
            }
            ExecutionError::OperatorAlreadyOpen => {
                write!(f, "Operator is already open")
            }
            ExecutionError::SchemaMismatch(msg) => {
                write!(f, "Schema mismatch: {}", msg)
            }
            ExecutionError::SchemaNotFound => {
                write!(f, "Schema not found")
            }
            ExecutionError::InvalidColumnIndex { index, count } => {
                write!(
                    f,
                    "Invalid column index {} (only {} columns available)",
                    index, count
                )
            }
            ExecutionError::ColumnNotFound(name) => {
                write!(f, "Column '{}' not found in batch", name)
            }
            ExecutionError::InvalidRowIndex { index, count } => {
                write!(
                    f,
                    "Invalid row index {} (only {} rows available)",
                    index, count
                )
            }
            ExecutionError::IoError(err) => {
                write!(f, "IO error during execution: {}", err)
            }
            ExecutionError::Custom(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

impl From<std::io::Error> for ExecutionError {
    fn from(err: std::io::Error) -> Self {
        ExecutionError::IoError(err)
    }
}

/// Result type for execution operations
pub type Result<T> = std::result::Result<T, ExecutionError>;

/// A batch of rows in columnar format for vectorized execution.
///
/// A Batch holds data in columnar format, which allows for efficient
/// vectorized operations. All columns in a batch must have the same number
/// of rows.
///
/// # Example
///
/// ```rust
/// use mini_rust_olap::execution::Batch;
/// use mini_rust_olap::column::{IntColumn, FloatColumn, Column};
/// use mini_rust_olap::types::Value;
/// use std::sync::Arc;
///
/// let mut col1 = IntColumn::new();
/// col1.push_value(Value::Int64(1)).unwrap();
/// col1.push_value(Value::Int64(2)).unwrap();
/// col1.push_value(Value::Int64(3)).unwrap();
///
/// let mut col2 = FloatColumn::new();
/// col2.push_value(Value::Float64(10.0)).unwrap();
/// col2.push_value(Value::Float64(20.0)).unwrap();
/// col2.push_value(Value::Float64(30.0)).unwrap();
///
/// let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);
/// assert_eq!(batch.row_count(), 3);
/// assert_eq!(batch.column_count(), 2);
/// ```
#[derive(Clone)]
pub struct Batch {
    columns: Vec<Arc<dyn Column>>,
}

impl Batch {
    /// Create a new Batch from a vector of columns.
    ///
    /// # Panics
    ///
    /// Panics if columns have different lengths or if the batch is empty.
    ///
    /// # Arguments
    ///
    /// * `columns` - Vector of columns with identical row counts
    pub fn new(columns: Vec<Arc<dyn Column>>) -> Self {
        if columns.is_empty() {
            panic!("Cannot create a batch with no columns");
        }

        let row_count = columns[0].len();

        for (i, col) in columns.iter().enumerate() {
            if col.len() != row_count {
                panic!(
                    "Column {} has {} rows, but column 0 has {} rows",
                    i,
                    col.len(),
                    row_count
                );
            }
        }

        Batch { columns }
    }

    /// Create an empty batch with the given schema.
    ///
    /// Useful for creating batches that will be populated later or for
    /// handling empty results.
    pub fn empty() -> Self {
        // Create a batch with no columns (special case for empty result)
        Batch {
            columns: Vec::new(),
        }
    }

    /// Returns the number of rows in the batch.
    pub fn row_count(&self) -> usize {
        if self.columns.is_empty() {
            0
        } else {
            self.columns[0].len()
        }
    }

    /// Returns the number of columns in the batch.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns true if the batch is empty (no rows).
    pub fn is_empty(&self) -> bool {
        self.row_count() == 0
    }

    /// Get a column by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The column index
    pub fn column(&self, index: usize) -> Result<Arc<dyn Column>> {
        if index >= self.columns.len() {
            return Err(ExecutionError::InvalidColumnIndex {
                index,
                count: self.columns.len(),
            });
        }
        Ok(self.columns[index].clone())
    }

    /// Get all columns in the batch.
    pub fn columns(&self) -> &[Arc<dyn Column>] {
        &self.columns
    }

    /// Get the value at a specific row and column.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The row index
    /// * `column_index` - The column index
    pub fn get(&self, row_index: usize, column_index: usize) -> Result<crate::types::Value> {
        if self.columns.is_empty() {
            return Err(ExecutionError::Custom("Batch is empty".to_string()));
        }
        if column_index >= self.columns.len() {
            return Err(ExecutionError::InvalidColumnIndex {
                index: column_index,
                count: self.columns.len(),
            });
        }

        let column = &self.columns[column_index];
        if row_index >= column.len() {
            return Err(ExecutionError::InvalidRowIndex {
                index: row_index,
                count: column.len(),
            });
        }

        column
            .get(row_index)
            .map_err(|e| ExecutionError::Custom(e.to_string()))
    }

    /// Get the value at a specific row and column as a string.
    ///
    /// This is a convenience method that always returns the value as a string.
    pub fn get_as_string(&self, row_index: usize, column_index: usize) -> Result<String> {
        let value = self.get(row_index, column_index)?;
        Ok(value.to_string())
    }

    /// Select specific columns to create a new batch.
    ///
    /// # Arguments
    ///
    /// * `column_indices` - Indices of columns to select
    pub fn select(&self, column_indices: &[usize]) -> Result<Batch> {
        let mut selected_columns = Vec::new();

        for &index in column_indices {
            if index >= self.columns.len() {
                return Err(ExecutionError::InvalidColumnIndex {
                    index,
                    count: self.columns.len(),
                });
            }
            selected_columns.push(self.columns[index].clone());
        }

        Ok(Batch::new(selected_columns))
    }

    /// Project columns to create a new batch with renamed columns.
    ///
    /// # Arguments
    ///
    /// * `column_indices` - Indices of columns to select
    /// * `_aliases` - New names for the selected columns (not yet implemented)
    pub fn project(&self, column_indices: &[usize], _aliases: &[String]) -> Result<Batch> {
        // For now, just select - renaming will be handled at the schema level
        self.select(column_indices)
    }
}

impl fmt::Debug for Batch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Batch")
            .field("row_count", &self.row_count())
            .field("column_count", &self.column_count())
            .finish()
    }
}

/// Base trait for all query operators.
///
/// Every query operator (Scan, Filter, Project, GroupBy, etc.) implements
/// this trait. Operators follow a pull-based execution model:
///
/// 1. `open()` - Initialize the operator and allocate resources
/// 2. `next_batch()` - Pull the next batch of data
/// 3. `close()` - Release resources and cleanup
///
/// Operators can be chained together to form a query execution plan.
/// For example: Scan → Filter → Project
///
/// # Example
///
/// ```rust
/// use mini_rust_olap::execution::Operator;
/// use mini_rust_olap::execution::ExecutionError;
/// use mini_rust_olap::execution::Result;
/// use mini_rust_olap::types::DataType;
/// use std::collections::HashMap;
/// use std::sync::Arc;
///
/// struct MyOperator {
///     // operator state
/// }
///
/// impl Operator for MyOperator {
///     fn open(&mut self) -> Result<()> {
///         // Initialize operator
///         Ok(())
///     }
///
///     fn next_batch(&mut self) -> Result<Option<mini_rust_olap::execution::Batch>> {
///         // Return next batch, None if done
///         Ok(None)
///     }
///
///     fn close(&mut self) -> Result<()> {
///         // Cleanup resources
///         Ok(())
///     }
///
///     fn schema(&self) -> Result<HashMap<String, DataType>> {
///         // Return output schema
///         Err(ExecutionError::Custom("Not implemented".into()))
///     }
/// }
/// ```
pub trait Operator {
    /// Initialize the operator and allocate any necessary resources.
    ///
    /// Must be called before `next_batch()`. This is where operators
    /// typically open file handles, allocate buffers, or initialize
    /// child operators.
    fn open(&mut self) -> Result<()>;

    /// Get the next batch of data from the operator.
    ///
    /// Returns `Ok(None)` when there are no more batches.
    /// Batches can be of varying sizes, but should be reasonably large
    /// for efficient vectorized processing (typically 1024 rows or more).
    ///
    /// # Arguments
    ///
    /// * `self` - Mutable reference to the operator
    ///
    /// # Returns
    ///
    /// * `Ok(Some(batch))` - The next batch of data
    /// * `Ok(None)` - No more data available
    /// * `Err(ExecutionError)` - An error occurred during execution
    fn next_batch(&mut self) -> Result<Option<Batch>>;

    /// Release resources and cleanup.
    ///
    /// Must be called after processing is complete. Operators should
    /// close file handles, free memory, and close child operators.
    fn close(&mut self) -> Result<()>;

    /// Get the schema of the output data.
    ///
    /// Returns the schema that will be produced by this operator.
    /// The schema should be valid after `open()` is called.
    fn schema(&self) -> Result<HashMap<String, DataType>>;

    /// Check if the operator is currently open.
    fn is_open(&self) -> bool {
        false
    }
}

/// State tracking for operator lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorState {
    NotOpen,
    Open,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column::{FloatColumn, IntColumn, StringColumn};
    use crate::types::Value;

    #[test]
    fn test_batch_creation() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let mut col2 = FloatColumn::new();
        col2.push_value(Value::Float64(1.0)).unwrap();
        col2.push_value(Value::Float64(2.0)).unwrap();
        col2.push_value(Value::Float64(3.0)).unwrap();

        let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);

        assert_eq!(batch.row_count(), 3);
        assert_eq!(batch.column_count(), 2);
    }

    #[test]
    #[should_panic(expected = "Cannot create a batch with no columns")]
    fn test_batch_empty_columns() {
        let _batch = Batch::new(vec![]);
    }

    #[test]
    #[should_panic(expected = "Column 1 has 5 rows, but column 0 has 3 rows")]
    fn test_batch_mismatched_lengths() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let mut col2 = IntColumn::new();
        col2.push_value(Value::Int64(1)).unwrap();
        col2.push_value(Value::Int64(2)).unwrap();
        col2.push_value(Value::Int64(3)).unwrap();
        col2.push_value(Value::Int64(4)).unwrap();
        col2.push_value(Value::Int64(5)).unwrap();

        let _batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);
    }

    #[test]
    fn test_batch_column_access() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let mut col2 = StringColumn::new();
        col2.push_value(Value::String("a".to_string())).unwrap();
        col2.push_value(Value::String("b".to_string())).unwrap();
        col2.push_value(Value::String("c".to_string())).unwrap();

        let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);

        let result = batch.column(0);
        assert!(result.is_ok());

        let result = batch.column(2);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ExecutionError::InvalidColumnIndex { index: 2, count: 2 })
        ));
    }

    #[test]
    fn test_batch_get_value() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(10)).unwrap();
        col1.push_value(Value::Int64(20)).unwrap();
        col1.push_value(Value::Int64(30)).unwrap();

        let mut col2 = StringColumn::new();
        col2.push_value(Value::String("x".to_string())).unwrap();
        col2.push_value(Value::String("y".to_string())).unwrap();
        col2.push_value(Value::String("z".to_string())).unwrap();

        let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);

        // Valid access - returns Value enum
        let result = batch.get(1, 0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int64(20));

        let result = batch.get(2, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::String("z".to_string()));

        // Test get_as_string
        let result = batch.get_as_string(1, 0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "20");

        // Invalid row index
        let result = batch.get(5, 0);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ExecutionError::InvalidRowIndex { index: 5, count: 3 })
        ));

        // Invalid column index
        let result = batch.get(0, 5);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ExecutionError::InvalidColumnIndex { index: 5, count: 2 })
        ));
    }

    #[test]
    fn test_batch_get_as_string() {
        let mut col1 = StringColumn::new();
        col1.push_value(Value::String("hello".to_string())).unwrap();
        col1.push_value(Value::String("world".to_string())).unwrap();
        col1.push_value(Value::String("test".to_string())).unwrap();

        let batch = Batch::new(vec![Arc::new(col1)]);

        let result = batch.get_as_string(1, 0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "world");
    }

    #[test]
    fn test_batch_select() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let mut col2 = StringColumn::new();
        col2.push_value(Value::String("a".to_string())).unwrap();
        col2.push_value(Value::String("b".to_string())).unwrap();
        col2.push_value(Value::String("c".to_string())).unwrap();

        let mut col3 = FloatColumn::new();
        col3.push_value(Value::Float64(1.0)).unwrap();
        col3.push_value(Value::Float64(2.0)).unwrap();
        col3.push_value(Value::Float64(3.0)).unwrap();

        let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2), Arc::new(col3)]);

        let selected = batch.select(&[0, 2]).unwrap();

        assert_eq!(selected.row_count(), 3);
        assert_eq!(selected.column_count(), 2);

        let val = selected.get(0, 0).unwrap();
        assert_eq!(val, Value::Int64(1));

        let val = selected.get(1, 1).unwrap();
        assert_eq!(val, Value::Float64(2.0));
    }

    #[test]
    fn test_batch_select_invalid_index() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let batch = Batch::new(vec![Arc::new(col1)]);

        let result = batch.select(&[0, 5]);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ExecutionError::InvalidColumnIndex { index: 5, count: 1 })
        ));
    }

    #[test]
    fn test_batch_empty() {
        let batch = Batch::empty();

        assert!(batch.is_empty());
        assert_eq!(batch.row_count(), 0);
    }

    #[test]
    fn test_execution_error_display() {
        let err = ExecutionError::OperatorNotOpen;
        assert_eq!(
            err.to_string(),
            "Operator must be opened before calling this operation"
        );

        let err = ExecutionError::InvalidColumnIndex { index: 5, count: 3 };
        assert_eq!(
            err.to_string(),
            "Invalid column index 5 (only 3 columns available)"
        );

        let err = ExecutionError::ColumnNotFound("age".to_string());
        assert_eq!(err.to_string(), "Column 'age' not found in batch");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let exec_err: ExecutionError = io_err.into();

        assert!(matches!(exec_err, ExecutionError::IoError(_)));
        assert_eq!(
            exec_err.to_string(),
            "IO error during execution: file not found"
        );
    }

    #[test]
    fn test_batch_debug() {
        let mut col1 = IntColumn::new();
        col1.push_value(Value::Int64(1)).unwrap();
        col1.push_value(Value::Int64(2)).unwrap();
        col1.push_value(Value::Int64(3)).unwrap();

        let mut col2 = StringColumn::new();
        col2.push_value(Value::String("a".to_string())).unwrap();
        col2.push_value(Value::String("b".to_string())).unwrap();
        col2.push_value(Value::String("c".to_string())).unwrap();

        let batch = Batch::new(vec![Arc::new(col1), Arc::new(col2)]);

        let debug_str = format!("{:?}", batch);
        assert!(debug_str.contains("Batch"));
        assert!(debug_str.contains("row_count: 3"));
        assert!(debug_str.contains("column_count: 2"));
    }

    // Simple mock operator for testing
    struct MockOperator {
        state: OperatorState,
    }

    impl Operator for MockOperator {
        fn open(&mut self) -> Result<()> {
            self.state = OperatorState::Open;
            Ok(())
        }

        fn next_batch(&mut self) -> Result<Option<Batch>> {
            if self.state != OperatorState::Open {
                return Err(ExecutionError::OperatorNotOpen);
            }
            Ok(None)
        }

        fn close(&mut self) -> Result<()> {
            self.state = OperatorState::Closed;
            Ok(())
        }

        fn schema(&self) -> Result<HashMap<String, DataType>> {
            Err(ExecutionError::Custom(
                "Mock operator has no schema".to_string(),
            ))
        }

        fn is_open(&self) -> bool {
            self.state == OperatorState::Open
        }
    }

    #[test]
    fn test_operator_lifecycle() {
        let mut op = MockOperator {
            state: OperatorState::NotOpen,
        };

        assert!(!op.is_open());

        // Open
        assert!(op.open().is_ok());
        assert!(op.is_open());

        // Get batch
        assert!(op.next_batch().is_ok());

        // Close
        assert!(op.close().is_ok());
        assert!(!op.is_open());
    }

    #[test]
    fn test_operator_not_open_error() {
        let mut op = MockOperator {
            state: OperatorState::NotOpen,
        };

        let result = op.next_batch();
        assert!(result.is_err());
        assert!(matches!(result, Err(ExecutionError::OperatorNotOpen)));
    }
}
