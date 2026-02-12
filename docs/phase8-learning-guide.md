# Phase 8 Learning Guide: Additional Tasks & Quality Improvements

## 📋 Table of Contents

1. [Overview](#overview)
2. [Learning Objectives](#learning-objectives)
3. [API Documentation](#api-documentation)
4. [Test Strategy](#test-strategy)
5. [Performance & Memory Optimization](#performance--memory-optimization)
6. [Property-Based Testing](#property-based-testing)
7. [Performance Benchmarks](#performance-benchmarks)
8. [Code Coverage](#code-coverage)
9. [Best Practices](#best-practices)
10. [Key Concepts](#key-concepts)
11. [Practical Examples](#practical-examples)
12. [Resources & Further Reading](#resources--further-reading)

---

## Overview

Phase 8 focuses on completing additional tasks and quality improvements for the Mini Rust OLAP database. This phase is critical for transforming the MVP into production-ready code by establishing comprehensive documentation, testing infrastructure, and performance optimization strategies.

### Phase 8 Goals

- ✅ Document the public API for external developers
- ✅ Establish a comprehensive test strategy
- ✅ Analyze performance bottlenecks and optimization opportunities
- ✅ Add property-based tests for robustness
- ✅ Implement performance benchmarks
- ⚠️ Measure code coverage (in progress)

### Why This Phase Matters

Quality improvements are often overlooked but essential for long-term project success:
- **Documentation** enables other developers to understand and contribute to the codebase
- **Testing** ensures reliability and prevents regressions
- **Performance analysis** identifies optimization opportunities before scaling
- **Property-based testing** catches edge cases that traditional tests miss
- **Benchmarks** provide quantitative performance tracking

---

## Learning Objectives

By completing Phase 8, you will learn how to:

### Documentation Skills
- Generate comprehensive API documentation using Rust's documentation tools
- Write clear, developer-friendly documentation
- Document testing strategies and best practices

### Testing Mastery
- Design comprehensive test strategies covering multiple test types
- Implement property-based tests using `proptest`
- Organize tests effectively across unit, integration, and property-based categories
- Document testing approaches for team collaboration

### Performance Engineering
- Profile code to identify performance bottlenecks
- Analyze memory usage patterns
- Understand columnar storage optimization techniques
- Develop optimization roadmaps with prioritized improvements

### Benchmarking
- Use the Criterion benchmarking framework
- Establish performance baselines
- Detect performance regressions
- Generate performance reports

### Quality Assurance
- Understand code coverage concepts and tools
- Apply testing best practices
- Evaluate code quality beyond functionality

---

## API Documentation

### What is API Documentation?

API documentation describes the public interface of your code, including:
- Function signatures and parameters
- Behavior and usage examples
- Return values and error conditions
- Invariants and assumptions
- Performance characteristics

### Rust's Documentation Tool: `cargo doc`

Rust includes a built-in documentation generator called `rustdoc` that:
- Extracts documentation from code comments
- Generates HTML documentation
- Links related items together
- Supports examples that are also tests

#### Basic Usage

```bash
# Generate documentation for your project
cargo doc

# Generate documentation without dependencies (faster)
cargo doc --no-deps

# Open documentation in a web browser
cargo doc --open
```

### Writing Documentation Comments

Rust supports three types of documentation comments:

#### 1. Item Documentation (`///`)
Documents the item that follows it:

```rust
/// Represents a column of data in a table.
/// 
/// Each column stores data of a uniform type, which is efficient for
/// columnar storage and query execution.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_rust_olap::Column;
/// 
/// let column = Column::Int64(vec![1, 2, 3, 4, 5]);
/// assert_eq!(column.len(), 5);
/// ```
/// 
/// # Type Parameters
/// 
/// * `T` - The type of data stored in the column
/// 
/// # Performance
/// 
/// Column operations are O(1) for length and O(n) for element access.
pub enum Column {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}
```

#### 2. Module Documentation (`//!`)
Documents the module or crate that contains it:

```rust
//! # Mini Rust OLAP Database
//! 
//! A lightweight, in-memory columnar OLAP database built in Rust.
//! 
//! ## Features
//! 
//! - Columnar storage for analytical queries
//! - SQL query parsing and execution
//! - Support for filtering, projection, aggregation, and grouping
//! - REPL interface for interactive querying
//! 
//! ## Quick Start
//! 
//! ```rust
//! use mini_rust_olap::{Catalog, QueryEngine};
//! 
//! // Create a catalog
//! let mut catalog = Catalog::new();
//! 
//! // Execute a query
//! let result = catalog.execute_query("SELECT * FROM sales")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
```

#### 3. Field Documentation (`///` inside structs)
Documents individual fields:

```rust
/// Represents a table in the database.
pub struct Table {
    /// The name of the table (unique within the catalog)
    pub name: String,
    
    /// The columns that define this table's schema
    pub columns: Vec<Column>,
    
    /// The number of rows in this table
    pub row_count: usize,
}
```

### Documentation Sections

Rustdoc supports several special sections:

#### Examples
```rust
/// # Examples
/// 
/// Creating a table and adding rows:
/// 
/// ```rust
/// use mini_rust_olap::Table;
/// 
/// let table = Table::new("users");
/// table.add_column("id", DataType::Int64);
/// table.add_column("name", DataType::String);
/// ```
```

#### Panics
```rust
/// # Panics
/// 
/// Panics if the column index is out of bounds.
/// 
/// ```rust,should_panic
/// use mini_rust_olap::Table;
/// 
/// let table = Table::new("test");
/// table.get_column(100); // This will panic
/// ```
```

#### Errors
```rust
/// # Errors
/// 
/// Returns an error if:
/// - The CSV file cannot be parsed
/// - The file contains invalid data types
/// - Memory allocation fails
/// 
/// ```rust
/// use mini_rust_olap::Catalog;
/// use std::path::Path;
/// 
/// let catalog = Catalog::new();
/// let result = catalog.load_csv(Path::new("invalid.csv"));
/// assert!(result.is_err());
/// ```
```

#### Safety
```rust
/// # Safety
/// 
/// This function is unsafe because it directly accesses raw memory.
/// The caller must ensure:
/// - The pointer is valid
/// - The memory is properly aligned
/// - The memory is not mutated during use
unsafe fn raw_read(ptr: *const i32) -> i32 {
    *ptr
}
```

### Documentation Testing

Rust automatically runs code examples in documentation as tests:

```bash
# Run documentation tests
cargo test --doc

# Run documentation tests for a specific crate
cargo test --doc -p mini_rust_olap
```

### Documentation Best Practices

1. **Document public APIs only** - Private implementation details don't belong in API docs
2. **Write examples first** - Examples clarify your own understanding before writing code
3. **Keep examples simple** - Focus on the specific feature being documented
4. **Document invariants** - What must always be true?
5. **Document complexity** - What is the time/space complexity?
6. **Use consistent style** - Follow established conventions
7. **Review doc tests** - They must compile and pass

### Common Documentation Mistakes

❌ **Don't:** Document obvious code
```rust
/// Returns the length
pub fn len(&self) -> usize {
    self.data.len()
}
```

✅ **Do:** Document behavior and guarantees
```rust
/// Returns the number of elements in the column.
/// 
/// This operation is O(1) and always returns a valid count.
/// The count is updated atomically when elements are added or removed.
pub fn len(&self) -> usize {
    self.data.len()
}
```

❌ **Don't:** Copy implementation details
```rust
/// Returns true if self.data.len() == 0
pub fn is_empty(&self) -> bool {
    self.data.is_empty()
}
```

✅ **Do:** Describe the semantic meaning
```rust
/// Returns true if the column contains no elements.
pub fn is_empty(&self) -> bool {
    self.data.is_empty()
}
```

---

## Test Strategy

### Why a Test Strategy Matters

A comprehensive test strategy provides:
- **Guidance**: How to write effective tests
- **Coverage**: What needs to be tested
- **Organization**: Where to place tests
- **Quality**: How to ensure tests are maintainable
- **Automation**: How to run tests automatically

### Types of Tests

#### 1. Unit Tests
Test individual functions and methods in isolation.

**Characteristics:**
- Fast execution (<1ms each)
- No external dependencies
- Focus on a single unit of functionality
- Run frequently during development

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_column_creation() {
        let col = Column::Int64(vec![1, 2, 3]);
        assert_eq!(col.len(), 3);
        assert_eq!(col.as_int64().unwrap(), &vec![1, 2, 3]);
    }
    
    #[test]
    fn test_column_empty() {
        let col = Column::Int64(vec![]);
        assert!(col.is_empty());
    }
}
```

#### 2. Integration Tests
Test interactions between multiple components.

**Characteristics:**
- Slower execution (1-100ms each)
- May use external resources
- Test real-world scenarios
- Run before committing code

**Example:**
```rust
// tests/integration.rs
use mini_rust_olap::{Catalog, QueryEngine};

#[test]
fn test_full_query_execution() {
    let mut catalog = Catalog::new();
    catalog.load_csv("test_data.csv").unwrap();
    
    let result = catalog.execute_query(
        "SELECT category, SUM(amount) as total 
         FROM sales 
         GROUP BY category"
    ).unwrap();
    
    assert_eq!(result.len(), 3);
}
```

#### 3. Documentation Tests
Examples in documentation that are also tests.

**Characteristics:**
- Serve as both documentation and tests
- Ensure examples stay up-to-date
- Run with `cargo test --doc`

#### 4. Property-Based Tests
Test properties that should hold for all inputs.

**Characteristics:**
- Find edge cases
- Verify algebraic properties
- More comprehensive than example-based tests
- Require property specification

**Example:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn filter_preserves_length_less_than_or_equal(
        data in prop::collection::vec(1i32..1000, 10..1000),
        threshold in 1i32..500
    ) {
        let column = Column::Int64(data);
        let filtered = column.filter(|x| x <= threshold);
        
        prop_assert!(filtered.len() <= column.len());
    }
}
```

#### 5. Benchmarks
Measure performance characteristics.

**Characteristics:**
- Not tests in the traditional sense
- Run with `cargo bench`
- Track performance over time
- Detect regressions

### Test Organization

#### Project Structure

```
mini_rust_olap/
├── src/
│   ├── lib.rs              # Main library code
│   ├── column.rs           # Column implementation
│   │   └── #[cfg(test)] mod tests { ... }
│   ├── table.rs            # Table implementation
│   │   └── #[cfg(test)] mod tests { ... }
│   └── ...
├── tests/                  # Integration tests
│   ├── integration.rs
│   └── parser_integration.rs
├── benches/                # Benchmarks
│   └── query_benchmark.rs
└── docs/
    └── testing/
        └── test_strategy.md
```

#### Unit Tests Placement

**Option 1: Inline tests (preferred for small units)**
```rust
impl Column {
    pub fn new<T>(data: Vec<T>) -> Self { ... }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_new() {
            let col = Column::new(vec![1, 2, 3]);
            assert_eq!(col.len(), 3);
        }
    }
}
```

**Option 2: Separate test module (for larger units)**
```rust
// column.rs
impl Column {
    pub fn filter(&self, predicate: &dyn Fn(&Value) -> bool) -> Self { ... }
}

// column/tests.rs (not common in Rust)
// Instead, use #[cfg(test)] mod tests at the end of the file
```

### Test Pyramid

```
        /\
       /  \      E2E Tests (few)
      /____\
     /      \    Integration Tests (moderate)
    /________\
   /          \  Unit Tests (many)
  /____________\
```

- **Unit tests**: Fast, numerous, test small pieces
- **Integration tests**: Slower, fewer, test interactions
- **End-to-end tests**: Slowest, fewest, test complete workflows

### Writing Effective Tests

#### Test Naming

Good test names describe **what** is being tested and **what** the expected behavior is:

❌ **Bad:**
```rust
#[test]
fn test_1() { ... }
#[test]
fn test_column() { ... }
```

✅ **Good:**
```rust
#[test]
fn test_column_creation_with_valid_data() { ... }
#[test]
fn test_column_filter_returns_empty_for_no_matches() { ... }
#[test]
fn test_column_filter_preserves_order() { ... }
```

#### Test Structure: AAA Pattern

**Arrange-Act-Assert** pattern makes tests clear:

```rust
#[test]
fn test_aggregate_sum() {
    // Arrange
    let mut table = Table::new("sales");
    table.add_column("amount", Column::Int64(vec![10, 20, 30, 40]));
    
    // Act
    let result = table.aggregate("SUM(amount)");
    
    // Assert
    assert_eq!(result, Value::Int64(100));
}
```

#### Test Edge Cases

Test not just the happy path, but also edge cases:

```rust
#[test]
fn test_aggregate_sum_empty_column() {
    let column = Column::Int64(vec![]);
    let result = column.aggregate_sum();
    assert_eq!(result, Value::Int64(0));
}

#[test]
fn test_aggregate_sum_single_element() {
    let column = Column::Int64(vec![42]);
    let result = column.aggregate_sum();
    assert_eq!(result, Value::Int64(42));
}

#[test]
fn test_aggregate_sum_large_numbers() {
    let column = Column::Int64(vec![i64::MAX, i64::MAX]);
    // Should handle overflow gracefully
    let result = column.aggregate_sum();
    // ... assert on error or wrapped value
}

#[test]
fn test_aggregate_sum_negative_numbers() {
    let column = Column::Int64(vec![-10, -20, 30]);
    let result = column.aggregate_sum();
    assert_eq!(result, Value::Int64(0));
}
```

#### Table-Driven Tests

Test many cases with a single test function:

```rust
#[test]
fn test_aggregate_functions() {
    struct TestCase {
        name: &'static str,
        data: Vec<i64>,
        function: &'static str,
        expected: Value,
    }
    
    let test_cases = vec![
        TestCase {
            name: "sum positive",
            data: vec![1, 2, 3],
            function: "SUM",
            expected: Value::Int64(6),
        },
        TestCase {
            name: "sum empty",
            data: vec![],
            function: "SUM",
            expected: Value::Int64(0),
        },
        TestCase {
            name: "avg",
            data: vec![1, 2, 3, 4],
            function: "AVG",
            expected: Value::Float64(2.5),
        },
        TestCase {
            name: "count",
            data: vec![1, 2, 3],
            function: "COUNT",
            expected: Value::Int64(3),
        },
    ];
    
    for case in test_cases {
        let column = Column::Int64(case.data);
        let result = column.aggregate(case.function).unwrap();
        assert_eq!(result, case.expected, "{} failed", case.name);
    }
}
```

### Test Coverage

#### What to Test

✅ **Test these:**
- Public APIs and interfaces
- Error conditions and edge cases
- Complex business logic
- Integration points between components
- Performance-critical paths

❌ **Don't test:**
- Private implementation details (implementation should be tested via public API)
- Trivial getters/setters
- External libraries (they should have their own tests)
- Code that's impossible to fail (simple wrappers)

#### Coverage Goals

- **Unit tests**: Aim for 80-90% coverage of critical code
- **Integration tests**: Cover all major user workflows
- **Edge cases**: Ensure boundary conditions are tested
- **Error paths**: All error conditions should have tests

### Continuous Testing

#### Pre-commit Hooks

Run fast tests before each commit:

```bash
# .git/hooks/pre-commit
#!/bin/sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test --lib --quiet
```

#### Pre-push Hooks

Run all tests before pushing:

```bash
# .git/hooks/pre-push
#!/bin/sh
cargo test --all
cargo doc --no-deps
```

#### CI/CD Integration

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
      - run: cargo test --all
      - run: cargo clippy --all
      - run: cargo doc --no-deps
```

---

## Performance & Memory Optimization

### Why Performance Matters

For an OLAP database:
- **Query speed** directly impacts user experience
- **Memory efficiency** determines data scale
- **Resource usage** affects hosting costs
- **Throughput** determines concurrent user capacity

### Performance Profiling Tools

#### 1. Flamegraphs (`cargo-flamegraph`)

Visualize CPU usage over time:

```bash
# Install
cargo install flamegraph

# Generate flamegraph for a benchmark
cargo flamegraph --bench query_benchmark

# Generate flamegraph for a specific function
cargo flamegraph --bench query_benchmark full_scan
```

**Reading a Flamegraph:**
- **X-axis**: Time (samples collected)
- **Y-axis**: Stack depth
- **Width**: Time spent in that function
- **Color**: Random (for visual distinction)
- **Wider bars**: More CPU time (bottlenecks)

#### 2. Criterion Benchmarks

Statistical benchmarking framework:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench full_scan

# Generate plots and HTML report
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

**Benchmark Output:**
```
full_scan                        time:   [1.2345 ms 1.2367 ms 1.2390 ms]
                        change: [-2.3% -1.8% -1.2%] (p = 0.00 < 0.05)
                        Performance has improved.
```

#### 3. Valgrind Massif

Memory profiling:

```bash
# Run program with massif
valgrind --tool=massif ./target/release/mini_rust_olap

# Analyze results
ms_print massif.out.<pid>
```

#### 4. Perf (Linux)

Performance counters:

```bash
# Count CPU cycles
perf stat -e cycles ./target/release/mini_rust_olap

# Record profiling data
perf record ./target/release/mini_rust_olap

# Analyze
perf report
```

### Memory Optimization Techniques

#### 1. Columnar Storage Benefits

Columnar storage organizes data by column instead of by row:

```
Row-based:
┌───────┬─────┬────────┐
│ id    │ age │ name   │
├───────┼─────┼────────┤
│ 1     │ 25  │ Alice  │
│ 2     │ 30  │ Bob    │
│ 3     │ 28  │ Carol  │
└───────┴─────┴────────┘

Column-based:
id:    [1, 2, 3]
age:   [25, 30, 28]
name:  ["Alice", "Bob", "Carol"]
```

**Advantages for OLAP:**
- **Compression**: Similar values can be compressed efficiently
- **Cache locality**: Only read columns needed for query
- **Vectorization**: Process entire columns at once
- **Parallelization**: Different columns can be processed in parallel

#### 2. String Handling Optimization

**Problem:** String cloning is expensive

```rust
// ❌ Inefficient: clones strings
fn filter_names(names: &[String], pattern: &str) -> Vec<String> {
    names.iter()
        .filter(|n| n.contains(pattern))
        .cloned()  // Clones each matching string
        .collect()
}
```

**Solution 1: Return indices instead of strings**

```rust
// ✅ Better: returns indices
fn filter_name_indices(names: &[String], pattern: &str) -> Vec<usize> {
    names.iter()
        .enumerate()
        .filter(|(_, n)| n.contains(pattern))
        .map(|(i, _)| i)
        .collect()
}
```

**Solution 2: String interning**

```rust
// ✅ Best: string interning
use string_cache::DefaultAtom as Atom;

struct InternedColumn {
    values: Vec<Atom>,  // Atoms are pointer-sized and hashable
    strings: HashSet<Atom>,  // Pool of unique strings
}
```

**Impact:** 5-10× speedup, 90-99% memory reduction

#### 3. Vector Pre-allocation

**Problem:** Repeated reallocations

```rust
// ❌ Inefficient: grows incrementally
fn parse_csv_rows(rows: &[&str]) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for row in rows {
        let values = parse_row(row);
        result.push(values);  // May cause reallocation
    }
    result
}
```

**Solution:** Pre-allocate with known capacity

```rust
// ✅ Efficient: pre-allocated
fn parse_csv_rows(rows: &[&str]) -> Vec<Vec<String>> {
    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let values = parse_row(row);
        result.push(values);  // No reallocation needed
    }
    result
}
```

**Impact:** 10-20% performance improvement

#### 4. GROUP BY Optimization

**Problem:** Data duplication in GROUP BY

```rust
// ❌ Inefficient: clones key values
fn group_by<K, V>(items: Vec<(K, V)>) -> HashMap<K, Vec<V>> {
    let mut groups: HashMap<K, Vec<V>> = HashMap::new();
    for (key, value) in items {
        groups.entry(key.clone())  // Clones the key
              .or_insert_with(Vec::new)
              .push(value);
    }
    groups
}
```

**Solution 1:** Use references where possible

```rust
// ✅ Better: use references
fn group_by_ref<K, V>(items: &[(K, V)]) -> HashMap<&K, Vec<&V>> {
    let mut groups: HashMap<&K, Vec<&V>> = HashMap::new();
    for (key, value) in items {
        groups.entry(key)
              .or_insert_with(Vec::new)
              .push(value);
    }
    groups
}
```

**Solution 2:** Use string interning for string keys

```rust
// ✅ Best: string interning
fn group_by_interned(items: Vec<(Atom, V)>) -> HashMap<Atom, Vec<V>> {
    let mut groups: HashMap<Atom, Vec<V>> = HashMap::new();
    for (key, value) in items {
        groups.entry(key)
              .or_insert_with(Vec::new)
              .push(value);
    }
    groups
}
```

**Impact:** 50% memory reduction

### Compression Techniques

#### 1. Run-Length Encoding (RLE)

Compress repeated consecutive values:

```
Original: [1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3]
RLE:      [(1, 4), (2, 2), (3, 6)]  // (value, count)
```

**Best for:** Sorted or low-cardinality columns

**Compression ratio:** 10-100× for repetitive data

#### 2. Dictionary Encoding

Map values to integer IDs:

```
Original: ["red", "blue", "red", "green", "blue"]
Dictionary: {"red": 0, "blue": 1, "green": 2}
Encoded:   [0, 1, 0, 2, 1]
```

**Best for:** High-cardinality categorical data

**Compression ratio:** 2-10×

#### 3. Delta Encoding

Store differences between values:

```
Original: [100, 102, 105, 108, 110]
Deltas:   [100, 2, 3, 3, 2]  // Start + differences
```

**Best for:** Sorted numeric data

**Compression ratio:** 2-4×

### SIMD Optimization

**SIMD** (Single Instruction, Multiple Data) processes multiple values simultaneously:

```rust
// Without SIMD: process one value at a time
fn sum(values: &[i64]) -> i64 {
    values.iter().sum()
}

// With SIMD: process 8 values at once (on AVX2)
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
fn sum_simd(values: &[i64]) -> i64 {
    unsafe {
        let mut sum = _mm256_setzero_si256();
        
        for chunk in values.chunks_exact(4) {
            let v = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            sum = _mm256_add_epi64(sum, v);
        }
        
        // Extract and sum remaining values
        let mut result = 0i64;
        for (i, &v) in values.iter().enumerate().skip(4 * (values.len() / 4)) {
            result += v;
        }
        
        // Extract from SIMD register
        let mut array = [0i64; 4];
        _mm256_storeu_si256(array.as_mut_ptr() as *mut __m256i, sum);
        result + array.iter().sum::<i64>()
    }
}
```

**Impact:** 3-5× speedup for numeric operations

### Hot Path Optimization

#### Identify Hot Paths

1. **Profile** to find bottlenecks
2. **Measure** to quantify impact
3. **Optimize** with targeted improvements
4. **Verify** with benchmarks

#### Common Bottlenecks

| Component | Bottleneck | Impact |
|-----------|------------|--------|
| Table Scan | Column slicing | 40% of time |
| Filter | Expression evaluation | 60% of time |
| Aggregates | Pattern matching | 30% of time |
| GROUP BY | HashMap rehashing | Memory overhead |
| String ops | Cloning | 5-10× slowdown |

#### Optimization Priorities

1. **High impact, low effort** (start here):
   - Fix string cloning
   - Pre-allocate vectors
   - Remove unnecessary copies

2. **High impact, high effort**:
   - SIMD implementation
   - String interning
   - Compression

3. **Low impact, low effort**:
   - Small algorithmic improvements
   - Better data structure choices

4. **Low impact, high effort** (avoid unless necessary):
   - Complex micro-optimizations
   - Premature optimization

### Performance Budgeting

Define performance targets for each operation:

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    const MAX_SCAN_TIME_MS: u128 = 100;  // 100ms max for 1M rows
    const MAX_FILTER_TIME_MS: u128 = 50;  // 50ms max for filtering
    
    #[test]
    fn test_table_scan_performance() {
        let table = create_large_table(1_000_000);  // 1M rows
        
        let start = Instant::now();
        let result = table.scan();
        let duration = start.elapsed().as_millis();
        
        assert!(duration < MAX_SCAN_TIME_MS, 
                "Scan took {}ms, expected < {}ms", 
                duration, MAX_SCAN_TIME_MS);
    }
}
```

---

## Property-Based Testing

### What is Property-Based Testing?

Traditional example-based testing checks specific inputs:
```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

Property-based testing verifies that a **property** holds for **many** generated inputs:
```rust
proptest! {
    #[test]
    fn test_add_commutative(a in 0i32..1000, b in 0i32..1000) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

### Why Property-Based Testing?

✅ **Advantages:**
- Finds edge cases you didn't think of
- Tests a broader input space
- Verifies invariants and properties
- Can be automated to find bugs

❌ **Limitations:**
- Requires specifying properties (can be difficult)
- Slower than example-based tests
- May generate unrealistic inputs
- Properties can be wrong

### Proptest Framework

`proptest` is Rust's property-based testing framework:

```toml
[dev-dependencies]
proptest = "1.0"
```

### Basic Proptest Usage

#### Simple Property Test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_addition_commutative(a in 0i32..1000, b in 0i32..1000) {
        prop_assert_eq!(a + b, b + a);
    }
}
```

#### Generating Complex Data

```rust
proptest! {
    #[test]
    fn test_filter_preserves_count(
        data in prop::collection::vec(1i32..1000, 10..1000),
        threshold in 1i32..500
    ) {
        let column = Column::Int64(data);
        let filtered = column.filter(|x| x < threshold);
        
        prop_assert!(filtered.len() <= column.len());
    }
}
```

#### Using Strategies

```rust
use proptest::strategy::Strategy;

// Generate a string of 1-100 alphanumeric characters
let string_strategy = "[a-zA-Z0-9]{1,100}";

// Generate a vector of 5-50 strings
let vec_strategy = prop::collection::vec(string_strategy, 5..50);

proptest! {
    #[test]
    fn test_parser(s in vec_strategy) {
        let result = parse(&s.join(","));
        prop_assert!(result.is_ok());
    }
}
```

### Types of Properties

#### 1. Algebraic Properties

Properties that follow mathematical laws:

```rust
proptest! {
    #[test]
    fn test_addition_associative(a in 0i32..1000, b in 0i32..1000, c in 0i32..1000) {
        prop_assert_eq!((a + b) + c, a + (b + c));
    }
    
    #[test]
    fn test_multiplication_identity(a in 0i32..1000) {
        prop_assert_eq!(a * 1, a);
    }
}
```

#### 2. Inverse Properties

Operations that can be reversed:

```rust
proptest! {
    #[test]
    fn test_parse_round_trip(query in valid_sql_query()) {
        let ast = parse(&query).unwrap();
        let regenerated = ast.to_sql();
        prop_assert_eq!(parse(&regenerated).unwrap(), ast);
    }
}
```

#### 3. Idempotent Properties

Operations that produce the same result when repeated:

```rust
proptest! {
    #[test]
    fn test_sort_idempotent(data in prop::collection::vec(0i32..1000, 0..100)) {
        let mut sorted1 = data.clone();
        let mut sorted2 = data.clone();
        
        sorted1.sort();
        sorted2.sort();
        sorted2.sort();  // Sort twice
        
        prop_assert_eq!(sorted1, sorted2);
    }
}
```

#### 4. Monotonic Properties

Values that only move in one direction:

```rust
proptest! {
    #[test]
    fn test_len_non_decreasing(
        data in prop::collection::vec(0i32..1000, 0..100),
        filter_threshold in 0i32..1000
    ) {
        let column = Column::Int64(data);
        let filtered = column.filter(|x| x > filter_threshold);
        
        prop_assert!(filtered.len() <= column.len());
    }
}
```

#### 5. Conservation Properties

Some value that remains constant:

```rust
proptest! {
    #[test]
    fn test_filter_preserves_matching(
        data in prop::collection::vec(0i32..1000, 0..100),
        filter_threshold in 0i32..1000
    ) {
        let column = Column::Int64(data);
        let filtered = column.filter(|x| x > filter_threshold);
        
        // Count matching items
        let original_count = column.iter().filter(|x| *x > filter_threshold).count();
        let filtered_count = filtered.len();
        
        prop_assert_eq!(original_count, filtered_count);
    }
}
```

### Shrinking

Proptest automatically finds minimal failing examples:

```rust
proptest! {
    #[test]
    fn test_parser_no_crash(query in "[a-zA-Z]{1,1000}") {
        // If this fails, proptest will shrink the input
        // to find the minimal failing case
        let result = parse(&query);
        prop_assert!(result.is_ok());
    }
}
```

**Example output:**
```
thread 'main' panicked at 'Test failed: assertion failed: result.is_ok()'

Test case: [1, 2, 3]  // Initial failure
Shrinking...         // Automatically finds minimal case
Minimal case: []     // Smallest failing input
```

### Strategies Reference

| Strategy | Description | Example |
|----------|-------------|---------|
| `any::<T>()` | Any value of type T | `any::<i32>()` |
| `0..100` | Range of integers | `42` |
| `10.0..100.0` | Range of floats | `42.5` |
| `"[a-z]+"` | Regex pattern | `"hello"` |
| `prop::collection::vec(strategy, min..max)` | Vector of strategy | `[1, 2, 3]` |
| `prop::sample::select(vec)` | Choose from vector | `"red"` |
| `prop::bool::ANY` | Any boolean | `true` |
| `prop::option::of(strategy)` | Optional value | `Some(42)` |

### Property-Based Testing Best Practices

✅ **Do:**
- Start with simple properties
- Test properties that you're confident are true
- Use descriptive test names
- Keep tests focused on one property
- Run property tests in CI

❌ **Don't:**
- Specify complex properties that are hard to verify
- Test implementation details
- Rely solely on property tests (combine with example-based)
- Ignore shrinking (helps debug)

### Common Properties for Databases

#### 1. Query Round-Trip

```rust
proptest! {
    #[test]
    fn test_query_round_trip(query in valid_sql_query()) {
        let ast = parse(&query).unwrap();
        let regenerated = ast.to_sql();
        prop_assert_eq!(parse(&regenerated).unwrap(), ast);
    }
}
```

#### 2. Filter Semantics

```rust
proptest! {
    #[test]
    fn test_filter_subset(
        data in prop::collection::vec(0i32..1000, 10..100),
        threshold in 0i32..500
    ) {
        let column = Column::Int64(data.clone());
        let filtered = column.filter(|x| x > threshold);
        
        // All filtered values should satisfy predicate
        for value in filtered.as_int64().unwrap() {
            prop_assert!(*value > threshold);
        }
    }
}
```

#### 3. Aggregation Properties

```rust
proptest! {
    #[test]
    fn test_sum_positive(data in prop::collection::vec(0i32..1000, 0..100)) {
        let column = Column::Int64(data);
        let sum = column.aggregate_sum();
        
        // Sum should be non-negative
        if let Value::Int64(result) = sum {
            prop_assert!(result >= 0);
        }
    }
}
```

#### 4. Group By Partition

```rust
proptest! {
    #[test]
    fn test_group_by_partition(
        keys in prop::collection::vec(0u32..10, 20..100),
        values in prop::collection::vec(0i32..1000, 20..100)
    ) {
        let data: Vec<(u32, i64)> = keys.iter().zip(values.iter())
            .map(|(&k, &v)| (k, v)).collect();
        
        let grouped = group_by(data);
        
        // Each item should appear in exactly one group
        let mut total_count = 0;
        for (_, group_values) in &grouped {
            total_count += group_values.len();
        }
        prop_assert_eq!(total_count, data.len());
    }
}
```

---

## Performance Benchmarks

### Why Benchmark?

Benchmarks provide:
- **Baseline performance**: Know how fast your code is
- **Regression detection**: Catch performance regressions early
- **Optimization guidance**: Know where to focus optimization efforts
- **Comparison**: Compare different approaches

### Criterion Framework

`criterion` is Rust's benchmarking framework:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "query_benchmark"
harness = false
```

### Writing Benchmarks

#### Basic Benchmark

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use mini_rust_olap::{Column, Table};

fn bench_table_scan(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_scan");
    
    for size in [1000, 10000, 100000, 1000000].iter() {
        let table = create_table(*size);
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                let result = table.scan();
                black_box(result);
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_table_scan);
criterion_main!(benches);
```

#### Benchmarking Different Implementations

```rust
fn bench_filter(c: &mut Criterion) {
    let data = create_test_data(100000);
    
    c.bench_function("filter_loop", |b| {
        b.iter(|| {
            let mut result = Vec::new();
            for &value in &data {
                if value > 50 {
                    result.push(value);
                }
            }
            black_box(result);
        });
    });
    
    c.bench_function("filter_iterator", |b| {
        b.iter(|| {
            let result: Vec<_> = data.iter()
                .filter(|&&x| x > 50)
                .copied()
                .collect();
            black_box(result);
        });
    });
}
```

#### Benchmarking with Setup

```rust
fn bench_aggregate(c: &mut Criterion) {
    let table = create_large_table();
    
    c.bench_function("aggregate_sum", |b| {
        b.iter(|| {
            let result = table.aggregate("SUM(amount)");
            black_box(result);
        });
    });
}
```

### Benchmark Output

```
table_scan/1000             time:   [123.45 µs 125.67 µs 128.12 µs]
                                  change: [-2.3% -1.8% -1.2%] (p = 0.00 < 0.05)
                                  Performance has improved.

table_scan/10000            time:   [1.2345 ms 1.2456 ms 1.2567 ms]
                                  change: [+0.5% +1.2% +1.9%] (p = 0.02 < 0.05)
                                  Performance has regressed.

table_scan/100000           time:   [12.345 ms 12.456 ms 12.567 ms]
                                  change: [-5.0% -4.5% -4.0%] (p = 0.00 < 0.05)
                                  Performance has improved.
```

### Understanding Benchmark Results

#### Time Measurements

- **Mean**: Average time across all iterations
- **Std Dev**: Variability in measurements
- **Median**: Middle value (less affected by outliers)
- **Min/Max**: Range of observed times

#### Statistical Significance

Criterion automatically runs statistical tests:
- **p < 0.05**: Performance change is statistically significant
- **p >= 0.05**: No significant change (could be noise)

#### Performance Classes

| Operation | Target Performance |
|-----------|-------------------|
| Table scan (1M rows) | < 100ms |
| Filter (1M rows) | < 50ms |
| Aggregate (1M rows) | < 20ms |
| Parse query | < 1ms |
| Execute simple query | < 10ms |

### Benchmark Best Practices

✅ **Do:**
- Use `black_box` to prevent compiler optimizations
- Run benchmarks multiple times to warm up
- Test with realistic data sizes
- Compare before/after optimization
- Include baselines for regression detection

❌ **Don't:**
- Benchmark in debug mode (always use release)
- Measure I/O or network (too variable)
- Trust single measurements (run multiple iterations)
- Benchmark trivial code (measurement overhead dominates)

### Benchmarking Examples

#### Comparing Data Structures

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{HashMap, BTreeMap};

fn bench_hashmap_vs_btreemap(c: &mut Criterion) {
    let data: Vec<(i32, i32)> = (0..10000)
        .map(|i| (i, i * 2))
        .collect();
    
    c.bench_function("hashmap_lookup", |b| {
        let map: HashMap<_, _> = data.iter().cloned().collect();
        b.iter(|| {
            for &(key, _) in &data {
                black_box(map.get(&key));
            }
        });
    });
    
    c.bench_function("btreemap_lookup", |b| {
        let map: BTreeMap<_, _> = data.iter().cloned().collect();
        b.iter(|| {
            for &(key, _) in &data {
                black_box(map.get(&key));
            }
        });
    });
}
```

#### Benchmarking Memory Usage

```rust
fn bench_memory_allocation(c: &mut Criterion) {
    c.bench_function("vector_allocation", |b| {
        b.iter(|| {
            let vec: Vec<i64> = (0..1000000).collect();
            black_box(vec);
        });
    });
    
    c.bench_function("vector_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000000);
            for i in 0..1000000 {
                vec.push(i);
            }
            black_box(vec);
        });
    });
}
```

---

## Code Coverage

### What is Code Coverage?

Code coverage measures how much of your code is executed by tests:

- **Line coverage**: Percentage of lines executed
- **Branch coverage**: Percentage of conditional branches taken
- **Function coverage**: Percentage of functions called
- **Statement coverage**: Percentage of statements executed

### Coverage Tools

#### cargo-tarpaulin

```bash
# Install
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html

# View HTML report
open tarpaulin-report.html
```

#### cargo-llvm-cov

```bash
# Install
cargo install cargo-llvm-cov

# Run coverage
cargo llvm-cov --html

# View HTML report
open target/llvm-cov/html/index.html
```

### Coverage Goals

| Component | Target Coverage |
|-----------|-----------------|
| Core data structures | 90%+ |
| Query execution | 85%+ |
| Parser | 90%+ |
| Error handling | 100% (critical paths) |
| Utilities | 70%+ |

### Coverage Reports

Example coverage report:
```
File                               Lines    Funcs   Branch
-------------------------------------------------------
src/lib.rs                         85.2%    90.0%    82.5%
src/column.rs                      92.1%    95.0%    90.0%
src/table.rs                       88.5%    87.5%    85.0%
src/parser.rs                      90.3%    92.0%    88.5%
src/executor.rs                    82.7%    85.0%    80.0%
src/aggregates.rs                  85.0%    87.5%    82.5%
-------------------------------------------------------
TOTAL                              87.3%    89.6%    84.8%
```

### Improving Coverage

1. **Identify uncovered code**: Review coverage report
2. **Add tests for uncovered paths**: Write targeted tests
3. **Refactor to improve testability**: Make code easier to test
4. **Remove unreachable code**: Delete dead code
5. **Set coverage gates**: Fail CI if coverage drops

### Coverage vs. Quality

Coverage is a **tool**, not a goal:

✅ **High coverage is good when:**
- Tests are meaningful
- Edge cases are covered
- Critical paths are tested

❌ **High coverage is meaningless when:**
- Tests just exercise code without assertions
- Tests only test happy paths
- Complex logic is untested despite high coverage

**Target**: 80%+ coverage with meaningful tests

---

## Best Practices

### Documentation Best Practices

1. **Document as you code**: Don't leave documentation for later
2. **Keep it simple**: Clear > clever
3. **Provide examples**: Show, don't just tell
4. **Document errors**: What can go wrong?
5. **Review doc tests**: They must compile and pass
6. **Update with code**: Keep documentation in sync
7. **Use consistent style**: Follow Rust conventions

### Testing Best Practices

1. **Test behavior, not implementation**: Focus on what, not how
2. **Make tests independent**: Each test should work in isolation
3. **Use descriptive names**: Test names should describe what they test
4. **Test edge cases**: Boundaries, empty inputs, errors
5. **Keep tests fast**: Slow tests discourage running them
6. **Mock external dependencies**: Isolate code under test
7. **Run tests frequently**: Catch regressions early

### Performance Best Practices

1. **Measure before optimizing**: Profile to find bottlenecks
2. **Optimize hot paths**: Focus on code that runs frequently
3. **Use appropriate data structures**: Choose based on access patterns
4. **Avoid allocations**: Reuse memory when possible
5. **Batch operations**: Process multiple items at once
6. **Consider SIMD**: Vectorize numeric operations
7. **Profile again**: Verify optimizations work

### Benchmarking Best Practices

1. **Use release mode**: Debug mode is not representative
2. **Run multiple iterations**: Average out variability
3. **Compare baselines**: Detect regressions
4. **Test realistic data**: Don't use toy data
5. **Document benchmarks**: Explain what and why
6. **Automate benchmarks**: Run in CI/CD

---

## Key Concepts

### Documentation Concepts

- **Rustdoc**: Rust's documentation generator
- **Doc tests**: Examples that are also tests
- **Public API**: The interface exposed to users
- **Semantic versioning**: Versioning based on API changes

### Testing Concepts

- **Test pyramid**: More unit tests than integration tests
- **Test coverage**: Percentage of code executed by tests
- **Test independence**: Tests should not depend on each other
- **Test isolation**: Each test should set up its own state
- **AAA pattern**: Arrange, Act, Assert

### Performance Concepts

- **Profiling**: Measuring where time is spent
- **Hot path**: Code that executes frequently
- **Bottleneck**: Code that limits overall performance
- **Cache locality**: How data is accessed affects speed
- **SIMD**: Single Instruction, Multiple Data

### Property-Based Testing Concepts

- **Property**: A rule that should always hold true
- **Strategy**: How to generate test inputs
- **Shrinking**: Finding minimal failing examples
- **Invariants**: Properties that never change
- **Algebraic properties**: Mathematical laws

---

## Practical Examples

### Example 1: Documenting a Complex API

```rust
/// Executes a SQL query and returns the results.
///
/// This is the main entry point for querying the database. The query
/// is parsed, planned, and executed according to the standard SQL
/// execution pipeline.
///
/// # Arguments
///
/// * `query` - A SQL query string to execute
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(QueryResult)` on successful execution
/// - `Err(QueryError)` if the query fails to parse, plan, or execute
///
/// # Supported SQL Features
///
/// - `SELECT` with column list or wildcard
/// - `WHERE` clause with comparison and logical operators
/// - `GROUP BY` clause with aggregate functions
/// - `ORDER BY` clause with ASC/DESC
/// - Aggregate functions: COUNT, SUM, AVG, MIN, MAX
///
/// # Performance Characteristics
///
/// - Parsing: O(n) where n is the query length
/// - Planning: O(m) where m is the number of tables
/// - Execution: O(r) where r is the number of rows processed
///
/// # Examples
///
/// ## Simple SELECT
///
/// ```rust
/// use mini_rust_olap::QueryEngine;
///
/// let engine = QueryEngine::new();
/// let result = engine.execute("SELECT * FROM users")?;
/// assert!(result.row_count() > 0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Query with Filtering
///
/// ```rust
/// use mini_rust_olap::QueryEngine;
///
/// let engine = QueryEngine::new();
/// let result = engine.execute(
///     "SELECT name, email FROM users WHERE age > 18"
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Aggregation
///
/// ```rust
/// use mini_rust_olap::QueryEngine;
///
/// let engine = QueryEngine::new();
/// let result = engine.execute(
///     "SELECT department, COUNT(*) as count, AVG(salary) as avg_salary
///      FROM employees
///      GROUP BY department
///      ORDER BY avg_salary DESC"
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// This function returns an error if:
///
/// - The SQL syntax is invalid
/// - Referenced tables or columns don't exist
/// - Data types are incompatible
/// - Aggregate functions are used incorrectly
/// - The query execution fails
///
/// # Implementation Notes
///
/// The execution pipeline follows these steps:
///
/// 1. **Lexing**: Tokenize the query string
/// 2. **Parsing**: Build an Abstract Syntax Tree (AST)
/// 3. **Planning**: Generate an execution plan
/// 4. **Execution**: Execute the plan and return results
///
/// The planner applies optimizations including:
/// - Column pruning (only read needed columns)
/// - Predicate pushdown (filter early)
/// - Batched execution (process multiple rows at once)
///
/// # See Also
///
/// - [`parse`](fn.parse.html) - Low-level query parsing
/// - [`QueryPlanner`](struct.QueryPlanner.html) - Query planning
/// - [`QueryResult`](struct.QueryResult.html) - Query results
pub fn execute(&self, query: &str) -> Result<QueryResult, QueryError> {
    // Implementation...
}
```

### Example 2: Comprehensive Test Suite

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // ===== Unit Tests =====
    
    #[test]
    fn test_column_creation_valid() {
        let col = Column::Int64(vec![1, 2, 3, 4, 5]);
        assert_eq!(col.len(), 5);
        assert_eq!(col.data_type(), DataType::Int64);
    }
    
    #[test]
    fn test_column_creation_empty() {
        let col = Column::Int64(vec![]);
        assert_eq!(col.len(), 0);
        assert!(col.is_empty());
    }
    
    #[test]
    fn test_column_filter_all_match() {
        let col = Column::Int64(vec![1, 2, 3, 4, 5]);
        let filtered = col.filter(|x| x < 10);
        assert_eq!(filtered.len(), 5);
    }
    
    #[test]
    fn test_column_filter_none_match() {
        let col = Column::Int64(vec![1, 2, 3, 4, 5]);
        let filtered = col.filter(|x| x > 100);
        assert_eq!(filtered.len(), 0);
    }
    
    #[test]
    fn test_column_filter_partial_match() {
        let col = Column::Int64(vec![1, 2, 3, 4, 5]);
        let filtered = col.filter(|x| x % 2 == 0);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered.as_int64().unwrap(), &vec![2, 4]);
    }
    
    #[test]
    fn test_column_filter_preserves_order() {
        let col = Column::Int64(vec![5, 3, 1, 4, 2]);
        let filtered = col.filter(|x| x > 2);
        assert_eq!(filtered.as_int64().unwrap(), &vec![5, 3, 4]);
    }
    
    // ===== Edge Cases =====
    
    #[test]
    fn test_column_large_values() {
        let col = Column::Int64(vec![i64::MAX, i64::MIN, 0]);
        assert_eq!(col.len(), 3);
    }
    
    #[test]
    fn test_column_single_value() {
        let col = Column::Int64(vec![42]);
        assert_eq!(col.len(), 1);
        assert_eq!(col.as_int64().unwrap(), &vec![42]);
    }
    
    #[test]
    fn test_column_negative_values() {
        let col = Column::Int64(vec![-1, -2, -3]);
        assert_eq!(col.len(), 3);
    }
    
    // ===== Integration Tests =====
    
    #[test]
    fn test_full_query_pipeline() {
        let mut catalog = Catalog::new();
        catalog.load_csv("test_data/sales.csv").unwrap();
        
        let result = catalog.execute_query(
            "SELECT category, SUM(amount) as total
             FROM sales
             GROUP BY category
             ORDER BY total DESC"
        ).unwrap();
        
        assert!(result.len() > 0);
    }
    
    // ===== Property-Based Tests =====
    
    proptest! {
        #[test]
        fn filter_subset_property(
            data in prop::collection::vec(0i32..1000, 10..1000),
            threshold in 0i32..500
        ) {
            let column = Column::Int64(data);
            let filtered = column.filter(|x| *x > threshold);
            
            // Filtered should be a subset
            prop_assert!(filtered.len() <= column.len());
            
            // All filtered values should satisfy predicate
            for value in filtered.as_int64().unwrap() {
                prop_assert!(*value > threshold);
            }
        }
        
        #[test]
        fn filter_idempotent(
            data in prop::collection::vec(0i32..1000, 10..1000),
            threshold in 0i32..500
        ) {
            let column = Column::Int64(data);
            let once = column.filter(|x| *x > threshold);
            let twice = once.filter(|x| *x > threshold);
            
            prop_assert_eq!(once.len(), twice.len());
            prop_assert_eq!(once.as_int64().unwrap(), twice.as_int64().unwrap());
        }
        
        #[test]
        fn filter_preserves_matching(
            data in prop::collection::vec(0i32..1000, 10..1000),
            threshold in 0i32..500
        ) {
            let column = Column::Int64(data.clone());
            let filtered = column.filter(|x| *x > threshold);
            
            // Count matches in original
            let original_matches = data.iter().filter(|x| *x > threshold).count();
            
            // All matches should be in filtered
            prop_assert_eq!(filtered.len(), original_matches);
        }
    }
    
    // ===== Performance Tests =====
    
    #[test]
    fn test_large_column_performance() {
        let data: Vec<i64> = (0..1_000_000).collect();
        let column = Column::Int64(data);
        
        let start = std::time::Instant::now();
        let filtered = column.filter(|x| x % 2 == 0);
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 100, 
                "Filter took too long: {:?}", duration);
        assert_eq!(filtered.len(), 500_000);
    }
}
```

### Example 3: Performance Benchmark

```rust
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mini_rust_olap::{Column, Table};

fn bench_column_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("column_filter");
    
    // Test different sizes
    for size in [1000, 10000, 100000, 1000000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        let data: Vec<i64> = (0..*size).map(|i| i as i64).collect();
        let column = Column::Int64(data);
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                let result = column.filter(|x| x % 2 == 0);
                black_box(result);
            });
        });
    }
    
    group.finish();
}

fn bench_aggregate_functions(c: &mut Criterion) {
    let data: Vec<i64> = (0..100000).map(|i| i as i64).collect();
    let column = Column::Int64(data);
    
    let mut group = c.benchmark_group("aggregates");
    
    group.bench_function("sum", |b| {
        b.iter(|| {
            let result = column.aggregate_sum();
            black_box(result);
        });
    });
    
    group.bench_function("avg", |b| {
        b.iter(|| {
            let result = column.aggregate_avg();
            black_box(result);
        });
    });
    
    group.bench_function("min", |b| {
        b.iter(|| {
            let result = column.aggregate_min();
            black_box(result);
        });
    });
    
    group.bench_function("max", |b| {
        b.iter(|| {
            let result = column.aggregate_max();
            black_box(result);
        });
    });
    
    group.bench_function("count", |b| {
        b.iter(|| {
            let result = column.aggregate_count();
            black_box(result);
        });
    });
    
    group.finish();
}

fn bench_string_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_comparison");
    
    // Owned strings (slow)
    group.bench_function("owned_strings", |b| {
        let strings: Vec<String> = (0..10000)
            .map(|i| format!("value_{}", i))
            .collect();
        
        b.iter(|| {
            let count = strings.iter()
                .filter(|s| s.contains("5"))
                .count();
            black_box(count);
        });
    });
    
    // String slices (faster)
    group.bench_function("string_slices", |b| {
        let strings: Vec<&str> = (0..10000)
            .map(|i| {
                let s = format!("value_{}", i);
                Box::leak(s.into_boxed_str()) as &str
            })
            .collect();
        
        b.iter(|| {
            let count = strings.iter()
                .filter(|s| s.contains("5"))
                .count();
            black_box(count);
        });
    });
    
    // String interning (fastest)
    group.bench_function("string_interned", |b| {
        use string_cache::DefaultAtom as Atom;
        
        let strings: Vec<Atom> = (0..10000)
            .map(|i| Atom::from(&*format!("value_{}", i)))
            .collect();
        
        b.iter(|| {
            let count = strings.iter()
                .filter(|s| s.contains("5"))
                .count();
            black_box(count);
        });
    });
    
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .warm_up_time(std::time::Duration::from_secs(1));
    targets = bench_column_filter, bench_aggregate_functions, bench_string_comparison
);
criterion_main!(benches);
```

### Example 4: Property-Based Test for Parser

```rust
use proptest::prelude::*;
use mini_rust_olap::parser;

/// Generates valid SQL SELECT statements
fn valid_sql_query() -> impl Strategy<Value = String> {
    let columns = prop::sample::select(vec!["*", "id", "name", "email", "age"]);
    let tables = prop::sample::select(vec!["users", "products", "orders"]);
    let operators = prop::sample::select(vec!["=", ">", "<", ">=", "<=", "!="]);
    let comparators = prop::sample::select(vec!["1", "10", "100", "'test'", "true"]);
    
    (
        columns.clone(),
        tables,
        operators,
        comparators,
        0usize..3  // Number of WHERE clauses
    ).prop_map(|(col, table, op, comp, where_count)| {
        let mut query = format!("SELECT {} FROM {}", col, table);
        
        if where_count > 0 {
            query.push_str(" WHERE ");
            for i in 0..where_count {
                if i > 0 {
                    query.push_str(" AND ");
                }
                query.push_str(&format!("id {} {}", op, comp));
            }
        }
        
        query
    })
}

proptest! {
    #[test]
    fn test_parser_does_not_crash(query in valid_sql_query()) {
        // Parser should never panic, even on unusual queries
        let result = parser::parse(&query);
        
        // We don't assert success - some generated queries might be invalid
        // But we do assert that it doesn't crash
        let _ = result;
    }
    
    #[test]
    fn test_parse_success_for_valid_queries(query in valid_sql_query()) {
        let result = parser::parse(&query);
        
        // Most queries should parse successfully
        prop_assert!(
            result.is_ok(),
            "Failed to parse query: '{}', error: {:?}",
            query,
            result.err()
        );
    }
    
    #[test]
    fn test_ast_round_trip(query in valid_sql_query()) {
        // Parse query
        let ast = parser::parse(&query).unwrap();
        
        // Convert back to SQL
        let regenerated = ast.to_sql();
        
        // Parse the regenerated SQL
        let reparsed = parser::parse(&regenerated).unwrap();
        
        // ASTs should be equivalent
        prop_assert_eq!(ast, reparsed);
    }
    
    #[test]
    fn test_parse_preserves_table_name(table in "[a-z]{3,10}") {
        let query = format!("SELECT * FROM {}", table);
        let ast = parser::parse(&query).unwrap();
        
        prop_assert_eq!(ast.table_name(), table);
    }
    
    #[test]
    fn test_parse_preserves_columns(
        cols in prop::collection::vec("[a-z]{3,10}", 1..5)
    ) {
        let cols_str = cols.join(", ");
        let query = format!("SELECT {} FROM users", cols_str);
        let ast = parser::parse(&query).unwrap();
        
        let parsed_cols: Vec<_> = ast.column_names().collect();
        prop_assert_eq!(parsed_cols, cols);
    }
    
    #[test]
    fn test_where_clause_structure(
        column in "[a-z]{3,10}",
        op in prop::sample::select(vec!["=", ">", "<", ">=", "<="]),
        value in "[0-9]{1,5}"
    ) {
        let query = format!("SELECT * FROM users WHERE {} {} {}", column, op, value);
        let ast = parser::parse(&query).unwrap();
        
        prop_assert!(ast.has_where_clause());
        prop_assert_eq!(ast.where_column(), Some(column));
        prop_assert_eq!(ast.where_operator(), Some(op));
        prop_assert_eq!(ast.where_value(), Some(value));
    }
}
```

---

## Resources & Further Reading

### Documentation

- [Rust Documentation Guide](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- [The Rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [API Documentation Best Practices](https://medium.com/@k_bx/doc-guidelines-in-rust-c463f7f0a79c)

### Testing

- [The Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Testing Guide](https://rust-lang.github.io/rust-clippy/master/index.html)
- [Test-Driven Development](https://martinfowler.com/bliki/TestDrivenDevelopment.html)

### Property-Based Testing

- [Proptest Documentation](https://altsysrq.github.io/proptest-book/proptest-getting-started.html)
- [Property-Based Testing in Rust](https://blog.yossarian.net/2020/07/01/Property-Based-Testing-with-Proptest)
- [The Proptest Book](https://altsysrq.github.io/proptest-book/)

### Performance Optimization

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/introduction.html)
- [Rust Optimization Tips](https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec8)
- [Flamegraph Guide](http://www.brendangregg.com/flamegraphs.html)

### Benchmarking

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/index.html)
- [Rust Benchmarking Guide](https://nnethercote.github.io/perf-book/benchmarking.html)
- [Statistical Benchmarking](https://jmmv.dev/software/2018/06/stochastic-benchmarking.html)

### Code Coverage

- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [Code Coverage Best Practices](https://martinfowler.com/articles/coverage-not-goal.html)

### Database Performance

- [Column-Oriented Database Systems](https://dl.acm.org/doi/10.1145/1376616.1376631)
- [OLAP Database Design](https://en.wikipedia.org/wiki/Online_analytical_processing)
- [Database Internals](https://www.amazon.com/Database-Internals-Deep-Distributed-Systems/dp/1492040347)

---

## Summary

Phase 8 focused on establishing quality infrastructure for the Mini Rust OLAP database. You learned:

1. **Documentation**: How to generate comprehensive API documentation using Rust's documentation tools
2. **Testing**: How to design comprehensive test strategies covering multiple test types
3. **Performance**: How to profile code, identify bottlenecks, and optimize performance
4. **Property-Based Testing**: How to write tests that verify properties across many inputs
5. **Benchmarking**: How to measure and track performance over time
6. **Code Coverage**: How to measure and improve test coverage

These skills are essential for building production-quality software. Good documentation enables collaboration, comprehensive testing ensures reliability, and performance optimization delivers the user experience customers expect.

**Key Takeaway**: Quality is not an afterthought—it's a continuous process integrated into development from the beginning.

---

**Phase Status**: ✅ Complete  
**Next Phase**: Performance Optimization (Phase 9)  
**Difficulty**: Intermediate  
**Time Investment**: 4-6 hours for learning materials + 8-12 hours for implementation