# Phase 5 Assessment: SQL Parser Implementation

## 📋 Assessment Overview

This assessment tests your understanding of SQL parsing, tokenization, AST design, and recursive descent parsing techniques covered in Phase 5 of Mini Rust OLAP.

### Assessment Structure

| Part | Topic | Questions | Difficulty |
|-------|--------|------------|------------|
| 1 | Tokenizer/Lexer Fundamentals | 10 | Beginner |
| 2 | AST Design and Representation | 10 | Beginner/Intermediate |
| 3 | Recursive Descent Parsing | 10 | Intermediate |
| 4 | Expression Parsing & Operator Precedence | 10 | Intermediate |
| 5 | SQL Clause Parsing | 8 | Intermediate |
| 6 | Aggregate Functions | 5 | Intermediate |
| 7 | Error Handling in Parsers | 5 | Intermediate/Advanced |
| 8 | Testing Strategies | 5 | Intermediate |
| 9 | Advanced Topics | 4 | Advanced |
| **Total** | | **67** | |

### Scoring Guide

- **57-67**: Excellent understanding - ready for Phase 6 ✅
- **47-56**: Good understanding - review weak areas 👍
- **38-46**: Fair understanding - review all topics 👌
- **Below 38**: Needs review - revisit Phase 5 materials 📚

---

## Part 1: Tokenizer/Lexer Fundamentals

### Q1. What is the primary purpose of a tokenizer (lexer)?

A. Execute SQL queries directly
B. Convert raw SQL text into a stream of tokens
C. Optimize query performance
D. Validate SQL syntax

### Q2. Which of the following is NOT a valid token type in our SQL parser?

A. `TokenType::Identifier`
B. `TokenType::StringLiteral`
C. `TokenType::Variable`
D. `TokenType::NumberLiteral`

### Q3. What information is stored in each token for better error reporting?

A. Token type and value only
B. Token type, value, line number, and column number
C. Token type and position only
D. Token value and column number only

### Q4. How does the tokenizer handle whitespace in SQL?

A. Converts whitespace to tokens
B. Skips whitespace but tracks line/column numbers
C. Treats whitespace as syntax errors
D. Stores whitespace in special tokens

### Q5. What happens when the tokenizer encounters "!=" in the SQL input?

A. Returns two tokens: ! and =
B. Returns one `NotEqual` token
C. Returns an error
D. Returns `Not` and `Equal` tokens

### Q6. Which method does the tokenizer use to look ahead at the next character without consuming it?

A. `advance()`
B. `match_char()`
C. `peek()`
D. `is_at_end()`

### Q7. How are string literals tokenized?

A. Up to the next whitespace character
B. Between single quotes (')
C. Between double quotes (")
D. Any sequence of letters

### Q8. What determines whether an identifier is a keyword or a regular identifier?

A. The length of the identifier
B. Case-insensitive comparison against keyword list
C. The position in the SQL statement
D. The presence of parentheses after it

### Q9. What does the tokenizer return when it reaches the end of input?

A. `None`
B. An empty string
C. A special `EOF` token
D. An error

### Q10. Why do we store token positions (line and column numbers)?

A. For performance optimization
B. To provide helpful error messages
C. For query planning
D. To reduce memory usage

---

## Part 2: AST Design and Representation

### Q11. What does AST stand for?

A. Abstract Syntax Tree
B. Abstract Structure Table
C. Advanced Syntax Tree
D. Algorithmic Structure Tree

### Q12. What is the top-level enum representing all possible SQL queries in our parser?

A. `QueryType`
B. `Statement`
C. `Query`
D. `SQLStatement`

### Q13. Which structure represents a complete SELECT statement?

A. `SelectItem`
B. `SelectStatement`
C. `SelectClause`
D. `QueryExpression`

### Q14. What does the `SelectItem` enum represent?

A. A column in the SELECT clause
B. An item that can be selected (wildcard or expression)
C. A table in the FROM clause
D. A condition in the WHERE clause

### Q15. How is an aggregate function represented in the AST?

A. As a string literal
B. As a special `AggregateFunction` expression variant
C. As a column reference
D. As a binary operation

### Q16. What does the `Expression` enum represent?

A. Only simple column references
B. Any value, column reference, or operation in SQL
C. Only numeric literals
D. Only string literals

### Q17. How are binary operations (e.g., `age > 25`) represented in the AST?

A. As a flat string
B. As `BinaryOp` with left operand, operator, and right operand
C. As two separate expressions
D. As a unary operation

### Q18. What is stored in the `SelectStatement` struct?

A. Only selected columns
B. SELECT items, FROM table, WHERE clause, and GROUP BY columns
C. The entire SQL string
D. Only table name

### Q19. What does the `UnaryOp` expression represent?

A. Operations with two operands
B. Operations with one operand (e.g., NOT, -)
C. Operations with three operands
D. Operations with no operands

### Q20. Why do we use `Box<Expression>` in `BinaryOp` and `UnaryOp`?

A. To enable pattern matching
B. To make expressions mutable
C. Because recursive types need indirection
D. For performance optimization

---

## Part 3: Recursive Descent Parsing

### Q21. What is recursive descent parsing?

A. Bottom-up parsing technique
B. Top-down parsing where each grammar rule is a function
C. Parsing that never uses recursion
D. Left-to-right parsing with lookback

### Q22. How does the parser determine which parsing function to call?

A. Based on the current token type
B. By guessing the next token
C. Using a lookup table
D. Randomly

### Q23. What does the `parse_query()` function do?

A. Parses only SELECT statements
B. Dispatches to the appropriate parsing function based on the query type
C. Parses WHERE clauses
D. Parses aggregate functions

### Q24. What does `consume_token()` do compared to `match_token()`?

A. `consume_token()` returns a boolean, `match_token()` returns an error on failure
B. `match_token()` returns a boolean, `consume_token()` returns an error on failure
C. They are identical
D. `consume_token()` advances position, `match_token()` doesn't

### Q25. What is the purpose of the `position` field in the Parser struct?

A. To store the line number
B. To track the current position in the token stream
C. To count the number of tokens
D. To store the column number

### Q26. How does the parser handle optional clauses like WHERE and GROUP BY?

A. They are required and cause errors if missing
B. By checking if the appropriate token exists and conditionally parsing
C. By always parsing them as empty
D. By ignoring them entirely

### Q27. What does `peek_token()` return?

A. The next token and advances position
B. The next token without advancing position
C. All remaining tokens
D. The previous token

### Q28. What happens when `consume_token()` finds an unexpected token?

A. It continues parsing
B. It returns an error with a descriptive message
C. It panics
D. It returns `None`

### Q29. What is the entry point for parsing a SQL query?

A. `parse_select_statement()`
B. `parse_expression()`
C. `parse()`
D. `parse_select_items()`

### Q30. How are comma-separated lists (like in SELECT items) parsed?

A. Using a loop that checks for commas
B. By parsing one item and stopping
C. Using a special list parser
D. Treating commas as errors

---

## Part 4: Expression Parsing & Operator Precedence

### Q31. What is operator precedence?

A. Order in which operators are evaluated when multiple appear in an expression
B. The speed of operator evaluation
C. The order in which tokens are generated
D. The priority of error messages

### Q32. Which operators have the HIGHEST precedence in our parser?

A. Logical AND/OR
B. Comparison operators (=, <, >, etc.)
C. Arithmetic operators (*, /)
D. Primary expressions (literals, identifiers, parentheses)

### Q33. Which operators have the LOWEST precedence?

A. Primary expressions
B. Unary operators (NOT, -)
C. Arithmetic operators
D. Logical OR

### Q34. How is operator precedence implemented in our parser?

A. By the order of function calls
B. Using a priority table
C. Through if-else statements
D. By token ordering

### Q35. What is the expression parsing hierarchy (from highest to lowest precedence)?

A. primary → unary → multiplicative → additive → comparison → and → or
B. or → and → comparison → additive → multiplicative → unary → primary
C. primary → or → and → comparison → additive → multiplicative → unary
D. unary → primary → multiplicative → additive → comparison → and → or

### Q36. What is left-associativity?

A. Operators are evaluated from right to left
B. Operators are evaluated from left to right
C. Operators have no fixed evaluation order
D. Only comparison operators are associative

### Q37. How does our parser handle left-associative operators like `+`?

A. Using a while loop that updates the left side
B. Using recursion on the right side only
C. Treating them as right-associative
D. Not handling associativity

### Q38. What happens with parentheses in expressions?

A. They are ignored
B. They override default precedence
C. They cause errors
D. They are treated as unary operators

### Q39. How is `age + 10 + 5` parsed?

A. `((age + 10) + 5)`
B. `(age + (10 + 5))`
C. `(age + 10) 5`
D. `age (10 + 5)`

### Q40. How is `NOT age > 25` parsed?

A. `(NOT (age > 25))`
B. `((NOT age) > 25)`
C. `(NOT age) 25`
D. `NOT (age > 25)` (depends on precedence)

---

## Part 5: SQL Clause Parsing

### Q41. What does the SELECT clause specify?

A. Which table to query
B. Which columns or expressions to return
C. How to filter rows
D. How to group results

### Q42. How is the wildcard (*) represented in the AST?

A. As `SelectItem::Wildcard`
B. As `Expression::Column("*")`
C. As a special `Star` token
D. As an error

### Q43. What does the FROM clause specify?

A. Which columns to select
B. Which table to query
C. How to filter rows
D. How to sort results

### Q44. How are multiple columns in the SELECT clause parsed?

A. As a single expression
B. Using commas as separators in a loop
C. As separate SELECT statements
D. As a special `MultiSelect` variant

### Q45. What does the WHERE clause contain?

A. A table name
B. A list of columns
C. A filtering expression
D. An aggregate function

### Q46. What does the GROUP BY clause specify?

A. Which table to use
B. How to sort results
C. Which columns to group by
D. How many rows to limit

### Q47. How are multiple GROUP BY columns parsed?

A. As a single column name
B. Using commas in a loop
C. As separate GROUP BY clauses
D. Not supported

### Q48. What is the order of clauses in a SELECT statement?

A. SELECT, WHERE, FROM, GROUP BY
B. SELECT, FROM, WHERE, GROUP BY
C. FROM, SELECT, WHERE, GROUP BY
D. SELECT, FROM, GROUP BY, WHERE

---

## Part 6: Aggregate Functions

### Q49. Which aggregate functions does our parser support?

A. COUNT only
B. COUNT, SUM, AVG, MIN, MAX
C. All SQL aggregate functions
D. COUNT and SUM only

### Q50. How is `COUNT(*)` represented in the AST?

A. As a special `CountAll` variant
B. As `AggregateFunction` with function "COUNT" and argument `Column("*")`
C. As a wildcard token
D. As an error

### Q51. What can be the argument of an aggregate function?

A. Only a column reference
B. Only an asterisk
C. Any expression
D. Only a number

### Q52. Where can aggregate functions appear in a SQL query?

A. Only in the SELECT clause
B. Only in the WHERE clause
C. In both SELECT and WHERE clauses
D. Only in GROUP BY

### Q53. How does the parser distinguish a column reference from a function call?

A. By checking the case of the identifier
B. By looking for a left parenthesis after the identifier
C. They are treated the same way
D. By checking a keyword list

---

## Part 7: Error Handling in Parsers

### Q54. What information should parser error messages include?

A. Only that an error occurred
B. What was expected and what was found
C. What was expected, what was found, and the location (line/column)
D. The entire SQL query

### Q55. What type does parser functions return?

A. `Option<Result>`
B. `Result<Query>`
C. `Result<T>` where T varies by function
D. `DatabaseError`

### Q56. How does the tokenizer report unterminated string literals?

A. Returns an empty string
B. Returns a special error token
C. Returns a `DatabaseError` with message "Unterminated string literal"
D. Ignores the error

### Q57. What happens when the parser encounters an unexpected token at the start of a query?

A. It continues parsing
B. It returns an error like "Expected SELECT, found FROM"
C. It assumes it's a comment
D. It panics

### Q58. Why is it important to include line and column numbers in errors?

A. For performance tracking
B. To help users locate and fix errors
C. For automated testing
D. It's required by the Rust compiler

---

## Part 8: Testing Strategies

### Q59. What is the benefit of testing the tokenizer in isolation?

A. It's faster than testing the full parser
B. Issues in tokenization can be identified before integration
C. It's required by Rust
D. It produces better error messages

### Q60. What should unit tests for the parser cover?

A. Only successful parsing cases
B. Only error cases
C. Both successful and error cases
D. Only complex queries

### Q61. What is an integration test for a parser?

A. Testing individual parsing functions
B. Testing complete, realistic SQL queries
C. Testing only error cases
D. Testing without a database

### Q62. Why is it important to test error cases?

A. To ensure the parser doesn't panic
B. To verify error messages are helpful
C. Both A and B
D. To increase test coverage numbers

### Q63. What is a good practice when testing parsers?

A. Test only one feature per test
B. Test complex scenarios only
C. Avoid testing edge cases
D. Only test with valid SQL

---

## Part 9: Advanced Topics

### Q64. What is error recovery in parsers?

A. Fixing the SQL query automatically
B. Continuing to parse after finding errors to report multiple errors
C. Recovering from crashes
D. Undoing parsing changes

### Q65. What is a parser combinator?

A. A tool for combining multiple parsers
B. A parser that combines tokens
C. A specialized type of tokenizer
D. A visual parser builder

### Q66. What is the main advantage of recursive descent parsing?

A. It's the fastest parsing technique
B. It's easy to understand and implement
C. It can parse any grammar
D. It requires no memory

### Q67. What would be required to add support for JOIN operations?

A. Only modifying the tokenizer
B. Adding new AST structures and parsing logic for join clauses
C. Only adding new tests
D. No changes needed

---

## 📊 Answer Key

### Part 1: Tokenizer/Lexer Fundamentals

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q1 | **B** | The tokenizer's job is to convert raw SQL text into tokens that the parser can understand |
| Q2 | **C** | `TokenType::Variable` is not a token type; we have Identifier, StringLiteral, NumberLiteral, etc. |
| Q3 | **B** | Tokens include type, value, line number, and column number for precise error reporting |
| Q4 | **B** | Whitespace is skipped but line/column numbers are tracked for accurate error positions |
| Q5 | **B** | `!=` is a single two-character token (NotEqual), not two separate tokens |
| Q6 | **C** | `peek()` returns the current character without consuming it for lookahead |
| Q7 | **B** | String literals are enclosed in single quotes (') in our implementation |
| Q8 | **B** | Identifiers are compared case-insensitively against a keyword list to determine if they're keywords |
| Q9 | **C** | A special EOF token is added to signal the end of input |
| Q10 | **B** | Line and column numbers allow parsers to provide helpful error messages like "Expected FROM at line 2, column 5" |

### Part 2: AST Design and Representation

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q11 | **A** | AST stands for Abstract Syntax Tree, representing the syntactic structure of SQL |
| Q12 | **C** | `Query` is the top-level enum with variants for different query types (currently only Select) |
| Q13 | **B** | `SelectStatement` contains all components of a SELECT query (SELECT items, FROM, WHERE, GROUP BY) |
| Q14 | **B** | `SelectItem` represents anything that can be in a SELECT clause: Wildcard or Expression |
| Q15 | **B** | Aggregate functions are `Expression::AggregateFunction` with function name and argument |
| Q16 | **B** | `Expression` represents any value, column, or operation: columns, literals, aggregates, binary/unary ops |
| Q17 | **B** | Binary operations are `Expression::BinaryOp` with left, operator, and right fields |
| Q18 | **B** | `SelectStatement` contains: select_items, from_table, where_clause (optional), and group_by (optional) |
| Q19 | **B** | `UnaryOp` represents operations with one operand: NOT and unary minus |
| Q20 | **C** | Recursive types like Expression containing other Expressions require Box for indirection |

### Part 3: Recursive Descent Parsing

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q21 | **B** | Recursive descent is top-down parsing where each grammar rule is implemented as a function |
| Q22 | **A** | The parser checks the current token type to determine which parsing function to call |
| Q23 | **B** | `parse_query()` dispatches to appropriate parsers (like `parse_select_statement()`) based on token type |
| Q24 | **B** | `match_token()` returns bool, `consume_token()` returns Result with error on failure |
| Q25 | **B** | `position` tracks the current index in the token stream, not line/column |
| Q26 | **B** | Optional clauses are parsed by checking if the keyword token exists and conditionally parsing |
| Q27 | **B** | `peek_token()` returns a reference to the current token without advancing position |
| Q28 | **B** | `consume_token()` returns a `DatabaseError` with a descriptive message when token doesn't match |
| Q29 | **C** | `parse()` is the public entry point that calls `parse_query()` internally |
| Q30 | **A** | Comma-separated lists use a loop that parses the first item, then checks for commas and parses more |

### Part 4: Expression Parsing & Operator Precedence

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q31 | **A** | Operator precedence determines evaluation order when multiple operators appear in an expression |
| Q32 | **D** | Primary expressions (literals, identifiers, parentheses) have highest precedence |
| Q33 | **D** | Logical OR has the lowest precedence in our precedence hierarchy |
| Q34 | **A** | Precedence is implemented by the order of function calls - higher precedence functions are called first |
| Q35 | **A** | The hierarchy is: primary → unary → multiplicative → additive → comparison → and → or |
| Q36 | **B** | Left-associative operators are evaluated from left to right |
| Q37 | **A** | While loops update the left side, making left-associative operators work correctly |
| Q38 | **B** | Parentheses override default precedence by forcing evaluation of enclosed expressions first |
| Q39 | **A** | `+` is left-associative, so `(age + 10) + 5` |
| Q40 | **D** | In standard SQL, NOT has lower precedence than comparison, so `age > 25` is evaluated first |

### Part 5: SQL Clause Parsing

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q41 | **B** | The SELECT clause specifies which columns or expressions to return from the query |
| Q42 | **A** | Wildcard is represented as `SelectItem::Wildcard` in the AST |
| Q43 | **B** | The FROM clause specifies which table to query |
| Q44 | **B** | Multiple columns are parsed using commas as separators in a loop |
| Q45 | **C** | The WHERE clause contains a filtering expression |
| Q46 | **C** | The GROUP BY clause specifies which columns to group results by |
| Q47 | **B** | Multiple GROUP BY columns are parsed using commas in a loop |
| Q48 | **B** | Correct order: SELECT, FROM, WHERE, GROUP BY |

### Part 6: Aggregate Functions

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q49 | **B** | Our parser supports COUNT, SUM, AVG, MIN, and MAX aggregate functions |
| Q50 | **B** | `COUNT(*)` is `AggregateFunction` with function "COUNT" and argument `Column("*")` |
| Q51 | **C** | Aggregate functions can take any expression as argument |
| Q52 | **A** | Aggregate functions typically appear in the SELECT clause |
| Q53 | **B** | A function call is identified by a left parenthesis after the identifier |

### Part 7: Error Handling in Parsers

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q54 | **C** | Good error messages include what was expected, what was found, and the location |
| Q55 | **C** | Parser functions return `Result<T>` where T varies (Query, Expression, SelectStatement, etc.) |
| Q56 | **C** | Unterminated strings return a `DatabaseError` with message "Unterminated string literal" |
| Q57 | **B** | Unexpected tokens at query start return errors like "Expected SELECT, found FROM" |
| Q58 | **B** | Line and column numbers help users locate and fix errors in their SQL |

### Part 8: Testing Strategies

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q59 | **B** | Testing tokenizer in isolation helps identify tokenization issues before integration |
| Q60 | **C** | Unit tests should cover both successful parsing and error cases |
| Q61 | **B** | Integration tests test complete, realistic SQL queries end-to-end |
| Q62 | **C** | Testing error cases ensures the parser doesn't panic and has helpful error messages |
| Q63 | **A** | Good practice is to test one feature per test to make failures easier to debug |

### Part 9: Advanced Topics

| Question | Correct Answer | Explanation |
|----------|----------------|-------------|
| Q64 | **B** | Error recovery allows continuing to parse after finding errors to report multiple errors |
| Q65 | **A** | Parser combinators are functions that can be combined to build parsers |
| Q66 | **B** | Recursive descent is easy to understand and implement, making it great for learning |
| Q67 | **B** | Adding JOIN would require new AST structures (join types, join conditions) and parsing logic |

---

## 🎯 Scoring and Feedback

### Calculate Your Score

```
Part 1 (Tokenizer/Lexer):                     _____ / 10
Part 2 (AST Design):                          _____ / 10
Part 3 (Recursive Descent):                    _____ / 10
Part 4 (Expression Parsing):                   _____ / 10
Part 5 (SQL Clause Parsing):                  _____ / 8
Part 6 (Aggregate Functions):                  _____ / 5
Part 7 (Error Handling):                       _____ / 5
Part 8 (Testing Strategies):                   _____ / 5
Part 9 (Advanced Topics):                      _____ / 4
--------------------------------------------------------
TOTAL SCORE:                                      _____ / 67
```

### Interpret Your Score

#### 57-67 Points: Excellent! 🎉
- You have mastered SQL parsing fundamentals
- Ready to tackle Phase 6 (Query Planning) confidently
- Consider reviewing Phase 5 code to solidify your knowledge
- Try the advanced exercises in the learning guide

#### 47-56 Points: Good! 👍
- You understand most concepts well
- Review the questions you missed in each part
- Revisit corresponding chapters in the learning guide
- Focus on weak areas (likely expression parsing or AST design)

#### 38-46 Points: Fair 👌
- You have basic understanding but gaps remain
- Review all Phase 5 materials systematically
- Practice with code examples in `src/parser.rs`
- Try beginner and intermediate exercises in learning guide

#### Below 38 Points: Needs Review 📚
- Return to Phase 5 learning guide
- Study chapters for areas where you struggled
- Run the parser tests and examine failures
- Start with beginner exercises in learning guide

---

## 📚 Study Resources

Based on your performance, focus on:

### Struggled with Tokenizer (Part 1)?
- Review `src/parser.rs` tokenizer implementation (lines ~60-350)
- Practice tokenizing different SQL statements manually
- Study the `Token` and `TokenType` structures
- Understand how whitespace and special characters are handled

### Struggled with AST Design (Part 2)?
- Review AST structures in `src/parser.rs` (lines ~350-500)
- Draw AST trees for various SQL queries
- Understand why `Box<Expression>` is needed for recursive types
- Practice matching AST patterns with Rust code

### Struggled with Recursive Descent (Part 3)?
- Study parsing functions in `src/parser.rs` (lines ~500-700)
- Trace through parsing of a simple query step by step
- Understand how `match_token()` and `consume_token()` work
- Review the two-phase approach (tokenize then parse)

### Struggled with Expression Parsing (Part 4)?
- Review expression parsing functions (lines ~700-900)
- Draw operator precedence hierarchies
- Practice parsing expressions with different operators
- Understand how precedence is implemented via function call order

### Struggled with SQL Clauses (Part 5)?
- Review clause parsing functions (lines ~900-1000)
- Practice writing different SQL queries
- Understand the order of SQL clauses
- Test with various clause combinations

### Struggled with Aggregate Functions (Part 6)?
- Review aggregate function parsing (lines ~1000-1100)
- Practice with different aggregate functions
- Understand how `COUNT(*)` is special
- Test complex queries with multiple aggregates

### Struggled with Error Handling (Part 7)?
- Review error handling throughout `src/parser.rs`
- Examine error messages in test failures
- Understand how line/column tracking works
- Practice with invalid SQL to see different errors

### Struggled with Testing (Part 8)?
- Review test cases in `src/parser.rs` (lines ~1100-1474)
- Write additional test cases
- Practice testing both success and error cases
- Understand why each test is important

### Struggled with Advanced Topics (Part 9)?
- Read advanced chapters in learning guide
- Try the advanced exercises
- Research parser theory (LL vs LR grammars)
- Explore parser combinator libraries (like `nom`)

---

## 💡 Tips for Learning

1. **Draw ASTs**: Visualizing SQL as AST trees helps understanding
2. **Trace Execution**: Step through parser code with a debugger
3. **Write SQL**: Practice writing complex SQL queries
4. **Read Code**: Study the actual parser implementation
5. **Modify Code**: Make small changes to see what breaks
6. **Add Features**: Try adding support for new SQL features
7. **Teach Others**: Explaining concepts solidifies understanding

---

## 📋 Preparation Checklist for Phase 6

Before moving to Phase 6 (Query Planning), make sure you:

- [ ] Understand how the parser converts SQL to AST
- [ ] Can read and interpret AST structures
- [ ] Know all supported SQL features (SELECT, FROM, WHERE, GROUP BY)
- [ ] Understand operator precedence and associativity
- [ ] Can identify how errors are reported
- [ ] Have reviewed the parser test suite
- [ ] Have tried writing your own SQL queries
- [ ] Understand the relationship between tokenizer, parser, and AST

---

**Good luck! This assessment is designed to help you identify strengths and areas for improvement. Use it as a learning tool, not just a test!** 🦀