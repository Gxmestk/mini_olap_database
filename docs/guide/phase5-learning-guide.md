# Phase 5 Learning Guide: SQL Parser Implementation
## Building a Recursive Descent SQL Parser in Rust

---

## 📚 Table of Contents

1. [Introduction to SQL Parsing](#chapter-1-introduction-to-sql-parsing)
2. [Tokenizer/Lexer Design](#chapter-2-tokenizerlexer-design)
3. [Abstract Syntax Tree (AST) Design](#chapter-3-abstract-syntax-tree-ast-design)
4. [Recursive Descent Parsing](#chapter-4-recursive-descent-parsing)
5. [Expression Parsing & Operator Precedence](#chapter-5-expression-parsing--operator-precedence)
6. [SQL Clause Parsing](#chapter-6-sql-clause-parsing)
7. [Aggregate Function Parsing](#chapter-7-aggregate-function-parsing)
8. [Error Handling in Parsers](#chapter-8-error-handling-in-parsers)
9. [Testing Parsers](#chapter-9-testing-parsers)
10. [Best Practices & Design Patterns](#chapter-10-best-practices--design-patterns)
11. [Learning Outcomes](#chapter-11-learning-outcomes)
12. [Practical Exercises](#chapter-12-practical-exercises)

---

## Chapter 1: Introduction to SQL Parsing

### 1.1 What is a SQL Parser?

A **SQL parser** is a component that converts raw SQL text into a structured representation (Abstract Syntax Tree or AST) that the database engine can understand and execute.

#### Why Do We Need a Parser?

Without a parser, you'd have to manually interpret every SQL query:

```rust
// ❌ Without a parser: Manual string parsing
fn execute_query(query: &str) -> Result<()> {
    if query.starts_with("SELECT") {
        // Manually extract columns, table name, conditions...
        let columns_part = query[7..].trim(); // After "SELECT"
        // This becomes extremely complex!
    }
    // ...
}
```

With a parser:

```rust
// ✅ With a parser: Clean, structured approach
fn execute_query(sql: &str) -> Result<()> {
    let parser = Parser::new(sql);
    let query = parser.parse()?; // Returns structured AST
    // Now work with the parsed query
    match query {
        Query::Select(stmt) => execute_select(stmt),
    }
}
```

### 1.2 The Two-Phase Approach

Most SQL parsers use a **two-phase approach**:

```
┌─────────────────────────────────────────────────────────┐
│           SQL Parsing Pipeline                          │
├─────────────────────────────────────────────────────────┤
│  Phase 1: Tokenizer/Lexer                             │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Input: "SELECT * FROM users WHERE age > 25"    │  │
│  │ Output: [SELECT, *, FROM, users, WHERE, age, >, │  │
│  │         25, EOF]                                 │  │
│  └──────────────────────────────────────────────────┘  │
│                        ↓                             │
│  Phase 2: Parser                                      │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Input: Token stream                             │  │
│  │ Output: Query AST                                │  │
│  │ {                                                │  │
│  │   SelectStatement {                               │  │
│  │     select_items: [Wildcard],                     │  │
│  │     from_table: "users",                         │  │
│  │     where_clause: BinaryOp(age > 25)           │  │
│  │   }                                              │  │
│  │ }                                                │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 1.3 Learning Objectives

By the end of this chapter, you will understand:

**Concepts:**
- ✅ Difference between tokenizing and parsing
- ✅ What an AST is and why it's useful
- ✅ Recursive descent parsing principles
- ✅ Operator precedence and associativity
- ✅ SQL grammar fundamentals

**Rust Skills:**
- ✅ Implementing complex state machines
- ✅ Working with enums for token types
- ✅ Pattern matching in parsers
- ✅ Error handling with custom error types
- ✅ Testing parsers comprehensively

### 1.4 Phase 5 Deliverables

✅ **Completed Components:**
- Token types and token definitions (20+ token types)
- Tokenizer implementation with error handling
- AST structures (Query, SelectStatement, Expression, etc.)
- Recursive descent parser for full SELECT syntax
- Support for WHERE and GROUP BY clauses
- Aggregate function parsing (COUNT, SUM, AVG, MIN, MAX)
- Comprehensive test suite (19 tests, all passing)
- **Total: 1,479 lines of code**

---

## Chapter 2: Tokenizer/Lexer Design

### 2.1 What is a Tokenizer?

A **tokenizer** (or **lexer**) breaks raw text into meaningful units called **tokens**. It's the first phase of parsing.

#### Analogy: Tokenizing a Sentence

```
Raw text:  "The quick brown fox jumps over the lazy dog"
Tokens:     [THE, QUICK, BROWN, FOX, JUMPS, OVER, THE, LAZY, DOG, EOF]
```

#### SQL Token Example

```rust
// SQL Input
"SELECT name, age FROM users WHERE age > 25"

// Tokens
[
    Token { type: SELECT,   line: 1, column: 1  },
    Token { type: Identifier("name"), line: 1, column: 8  },
    Token { type: Comma,   line: 1, column: 12 },
    Token { type: Identifier("age"),  line: 1, column: 14 },
    Token { type: FROM,    line: 1, column: 18 },
    Token { type: Identifier("users"), line: 1, column: 23 },
    Token { type: WHERE,   line: 1, column: 29 },
    Token { type: Identifier("age"),  line: 1, column: 35 },
    Token { type: Greater, line: 1, column: 39 },
    Token { type: NumberLiteral("25"), line: 1, column: 41 },
    Token { type: EOF,     line: 1, column: 43 },
]
```

### 2.2 Token Type Design

In our implementation, we use an enum to represent all possible token types:

```rust
pub enum TokenType {
    // Keywords
    Select, From, Where, Group, By, And, Or, Not,
    
    // Aggregate functions
    Count, Sum, Avg, Min, Max,
    
    // Comparison operators
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    
    // Arithmetic operators
    Plus, Minus, Asterisk, Divide,
    
    // Punctuation
    LeftParen, RightParen, Comma,
    
    // Literals and identifiers
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    
    // Special
    EOF,
}
```

#### Why Use an Enum for Token Types?

✅ **Type Safety**: The compiler ensures we handle all token types
✅ **Pattern Matching**: Easy to match against specific tokens
✅ **Memory Efficiency**: Each token type is just a single byte in the enum tag
✅ **Self-Documenting**: Token types are clear and readable

### 2.3 Token Position Tracking

We track line and column numbers for better error messages:

```rust
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,    // Line number (1-indexed)
    pub column: usize,  // Column number (1-indexed)
}
```

**Example:**

```rust
"SELECT name FROM users\nWHERE age > 25"
         ^ line 1, column 8

When we report an error:
"Expected 'FROM' at line 2, column 1"
```

### 2.4 Tokenizer State Machine

The tokenizer is essentially a **state machine** that processes characters one at a time:

```rust
pub struct Tokenizer {
    input: Vec<char>,      // Input characters
    position: usize,       // Current position
    line: usize,           // Current line number
    column: usize,         // Current column number
}

impl Tokenizer {
    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(Token::new(TokenType::EOF, self.line, self.column));
        }
        
        let c = self.peek().unwrap();
        match c {
            '(' => self.consume_single(TokenType::LeftParen),
            ')' => self.consume_single(TokenType::RightParen),
            '=' => self.consume_single(TokenType::Equal),
            '0'..='9' => self.number_literal(),
            'a'..='z' | 'A'..='Z' => self.identifier_or_keyword(),
            // ... more cases
            _ => Err(DatabaseError::parser_error("Unexpected character")),
        }
    }
}
```

### 2.5 Handling Different Token Types

#### 2.5.1 Single-Character Tokens

```rust
fn consume_single(&mut self, token_type: TokenType) -> Result<Token> {
    let line = self.line;
    let column = self.column;
    self.advance();  // Consume the character
    Ok(Token::new(token_type, line, column))
}
```

#### 2.5.2 Multi-Character Tokens

Some tokens require looking ahead:

```rust
fn next_token(&mut self) -> Result<Token> {
    // ...
    match c {
        '!' => {
            self.advance();  // Consume '!'
            if self.match_char('=') {  // Look ahead for '='
                Ok(Token::new(TokenType::NotEqual, line, column))
            } else {
                Err(DatabaseError::parser_error("Expected '=' after '!'"))
            }
        }
        '<' => {
            self.advance();
            if self.match_char('=') {
                Ok(Token::new(TokenType::LessEqual, line, column))
            } else {
                Ok(Token::new(TokenType::Less, line, column))
            }
        }
        // ... similar for '>', '-', etc.
    }
}

// Utility: Check if next character matches expected
fn match_char(&mut self, expected: char) -> bool {
    if let Some(&c) = self.peek() {
        if c == expected {
            self.advance();
            return true;
        }
    }
    false
}
```

#### 2.5.3 String Literals

```rust
fn string_literal(&mut self) -> Result<Token> {
    let line = self.line;
    let column = self.column;
    self.advance();  // Skip opening quote
    
    let mut value = String::new();
    while let Some(&c) = self.peek() {
        if c == '\'' {
            self.advance();  // Skip closing quote
            return Ok(Token::new(TokenType::StringLiteral(value), line, column));
        }
        value.push(c);
        self.advance();
    }
    
    Err(DatabaseError::parser_error("Unterminated string literal"))
}
```

**Examples:**

```rust
'Hello'     → StringLiteral("Hello")
'New York'  → StringLiteral("New York")
'O\'Reilly' → StringLiteral("O'Reilly")  // (escaped quotes)
```

#### 2.5.4 Number Literals

```rust
fn number_literal(&mut self) -> Result<Token> {
    let line = self.line;
    let column = self.column;
    let mut value = String::new();
    
    while let Some(&c) = self.peek() {
        if c.is_ascii_digit() || c == '.' {
            value.push(c);
            self.advance();
        } else {
            break;
        }
    }
    
    Ok(Token::new(TokenType::NumberLiteral(value), line, column))
}
```

**Examples:**

```rust
42      → NumberLiteral("42")
3.14    → NumberLiteral("3.14")
-100    → NumberLiteral("-100")  // (handled as unary minus in parser)
```

#### 2.5.5 Identifiers and Keywords

```rust
fn identifier_or_keyword(&mut self) -> Result<Token> {
    let line = self.line;
    let column = self.column;
    let mut value = String::new();
    
    // Collect alphanumeric characters and underscores
    while let Some(&c) = self.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            value.push(c);
            self.advance();
        } else {
            break;
        }
    }
    
    // Check if it's a keyword (case-insensitive)
    let token_type = match value.to_uppercase().as_str() {
        "SELECT" => TokenType::Select,
        "FROM" => TokenType::From,
        "WHERE" => TokenType::Where,
        "COUNT" => TokenType::Count,
        // ... other keywords
        _ => TokenType::Identifier(value.to_lowercase()),  // Lowercase identifiers
    };
    
    Ok(Token::new(token_type, line, column))
}
```

**Examples:**

```rust
users    → Identifier("users")
age      → Identifier("age")
SELECT   → Select
COUNT    → Count
table1   → Identifier("table1")
```

### 2.6 Whitespace Handling

Whitespace is generally ignored in SQL, but we need to track line/column numbers:

```rust
fn skip_whitespace(&mut self) {
    while let Some(&c) = self.peek() {
        if c.is_whitespace() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;  // Reset column at new line
            } else {
                self.column += 1;
            }
            self.advance();
        } else {
            break;
        }
    }
}
```

### 2.7 Tokenizer Testing Strategy

Test the tokenizer in isolation before moving to parsing:

```rust
#[test]
fn test_tokenizer_select() {
    let mut tokenizer = Tokenizer::new("SELECT * FROM users");
    let tokens = tokenizer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Select);
    assert_eq!(tokens[1].token_type, TokenType::Asterisk);
    assert_eq!(tokens[2].token_type, TokenType::From);
    assert!(matches!(tokens[3].token_type, TokenType::Identifier(s) if s == "users"));
}
```

---

## Chapter 3: Abstract Syntax Tree (AST) Design

### 3.1 What is an AST?

An **Abstract Syntax Tree (AST)** is a tree representation of the syntactic structure of source code. It abstracts away details like parentheses and whitespace.

#### Example: SQL to AST

```sql
-- SQL Query
SELECT name, COUNT(*) 
FROM users 
WHERE age > 25 
GROUP BY city
```

```rust
-- Parsed AST
Query::Select(SelectStatement {
    select_items: [
        Expression(Column("name")),
        Expression(AggregateFunction {
            function: "COUNT",
            argument: Column("*"),
        }),
    ],
    from_table: "users",
    where_clause: Some(BinaryOp {
        left: Column("age"),
        operator: Greater,
        right: NumberLiteral("25"),
    }),
    group_by: Some(vec!["city"]),
})
```

### 3.2 AST Hierarchy

Our AST has a clear hierarchy:

```
Query
└─ SelectStatement
   ├─ select_items: Vec<SelectItem>
   │  ├─ Wildcard
   │  └─ Expression
   ├─ from_table: String
   ├─ where_clause: Option<Expression>
   └─ group_by: Option<Vec<String>>

Expression
├─ Column(String)
├─ StringLiteral(String)
├─ NumberLiteral(String)
├─ AggregateFunction { function, argument }
├─ BinaryOp { left, operator, right }
└─ UnaryOp { operator, operand }
```

### 3.3 Expression Tree Structure

Expressions are the most complex part of the AST. Consider:

```sql
age > 25 AND city = 'New York'
```

This becomes:

```
                BinaryOp(AND)
               /             \
        BinaryOp(>)       BinaryOp(=)
        /      \         /       \
    Column   Number   Column   String
    (age)    (25)    (city)    ("New York")
```

In Rust:

```rust
Expression::BinaryOp {
    left: Box::new(Expression::Column("age".to_string())),
    operator: BinaryOperator::Greater,
    right: Box::new(Expression::NumberLiteral("25".to_string())),
}
```

### 3.4 Binary Operators

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    // Comparison
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    
    // Logical
    And, Or,
    
    // Arithmetic
    Plus, Minus, Multiply, Divide,
}
```

### 3.5 Unary Operators

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,   // NOT condition
    Minus, // -value
}
```

### 3.6 Aggregate Functions

```rust
Expression::AggregateFunction {
    function: "COUNT",
    argument: Box::new(Expression::Column("*".to_string())),
}
```

Supported aggregate functions:
- **COUNT**: Counts non-NULL values
- **SUM**: Sums numeric values
- **AVG**: Calculates average
- **MIN**: Finds minimum value
- **MAX**: Finds maximum value

### 3.7 AST Design Benefits

✅ **Type Safety**: Compiler ensures all variants are handled
✅ **Pattern Matching**: Easy to destructure and analyze
✅ **Serializable**: Can be easily printed, cloned, and compared
✅ **Extensible**: Easy to add new expression types
✅ **Self-Documenting**: AST structure clearly represents SQL structure

---

## Chapter 4: Recursive Descent Parsing

### 4.1 What is Recursive Descent Parsing?

**Recursive descent parsing** is a top-down parsing technique where each grammar rule is implemented as a function that calls other functions (recursively).

#### Analogy: Parsing a Sentence

```
Grammar:
  sentence → subject verb object
  subject → "I" | "You" | "The dog"
  verb → "see" | "eat" | "chase"
  object → "it" | "the cat" | "the ball"

Parser Functions:
  parse_sentence() → parse_subject() + parse_verb() + parse_object()
  parse_subject() → match "I" | "You" | "The dog"
  parse_verb() → match "see" | "eat" | "chase"
  parse_object() → match "it" | "the cat" | "the ball"
```

### 4.2 SQL Grammar for SELECT Statements

```
query → select_statement

select_statement → SELECT select_items 
                    FROM identifier 
                    [WHERE expression] 
                    [GROUP BY group_by_list]

select_items → select_item [',' select_item]*

select_item → '*' | expression

expression → or_expression

or_expression → and_expression [OR and_expression]*

and_expression → comparison_expression [AND comparison_expression]*

comparison_expression → additive_expression 
                        [('=' | '!=' | '<' | '>' | '<=' | '>=') additive_expression]

additive_expression → multiplicative_expression 
                      [('+' | '-') multiplicative_expression]*

multiplicative_expression → unary_expression 
                            [('*' | '/') unary_expression]*

unary_expression → [('NOT' | '-')] primary_expression

primary_expression → identifier 
                    | string_literal 
                    | number_literal 
                    | aggregate_function 
                    | '(' expression ')'

aggregate_function → (COUNT | SUM | AVG | MIN | MAX) '(' expression ')'
```

### 4.3 Parser Structure

```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,  // Current position in token stream
}

impl Parser {
    pub fn parse(&mut self) -> Result<Query> {
        self.parse_query()
    }
    
    fn parse_query(&mut self) -> Result<Query> {
        // Currently only SELECT is supported
        match self.peek_token_type() {
            Some(TokenType::Select) => {
                let select_stmt = self.parse_select_statement()?;
                Ok(Query::Select(select_stmt))
            }
            Some(t) => Err(DatabaseError::parser_error(
                format!("Expected SELECT, found {:?}", t)
            )),
            None => Err(DatabaseError::parser_error("Unexpected end of input")),
        }
    }
}
```

### 4.4 Utility Methods

The parser needs several utility methods:

#### 4.4.1 Lookahead (Peek)

```rust
fn peek_token(&self) -> Option<&Token> {
    self.tokens.get(self.position)
}

fn peek_token_type(&self) -> Option<TokenType> {
    self.peek_token().map(|t| t.token_type.clone())
}
```

#### 4.4.2 Advance Position

```rust
fn advance(&mut self) {
    if self.position < self.tokens.len() {
        self.position += 1;
    }
}
```

#### 4.4.3 Match Token

```rust
fn match_token(&mut self, expected: TokenType) -> bool {
    if let Some(token) = self.peek_token() {
        if token.token_type == expected {
            self.advance();
            return true;
        }
    }
    false
}
```

**Usage:**

```rust
// Check if next token is 'WHERE', consume it if so
if self.match_token(TokenType::Where) {
    let condition = self.parse_expression()?;
    // ... use condition
}
```

#### 4.4.4 Consume Token (with Error)

```rust
fn consume_token(&mut self, expected: TokenType, error_message: &str) -> Result<()> {
    if let Some(token) = self.peek_token() {
        if token.token_type == expected {
            self.advance();
            return Ok(());
        }
        
        return Err(DatabaseError::parser_error(format!(
            "{} at line {}, column {}",
            error_message, token.line, token.column
        )));
    }
    
    Err(DatabaseError::parser_error(format!(
        "{} (found EOF)",
        error_message
    )))
}
```

**Usage:**

```rust
// Require next token to be FROM
self.consume_token(TokenType::From, "Expected FROM")?;
```

### 4.5 Parsing the SELECT Statement

```rust
fn parse_select_statement(&mut self) -> Result<SelectStatement> {
    // SELECT clause
    self.consume_token(TokenType::Select, "Expected SELECT")?;
    let select_items = self.parse_select_items()?;
    
    // FROM clause (required)
    self.consume_token(TokenType::From, "Expected FROM")?;
    let from_table = self.parse_identifier()?;
    
    // WHERE clause (optional)
    let where_clause = if self.match_token(TokenType::Where) {
        Some(self.parse_expression()?)
    } else {
        None
    };
    
    // GROUP BY clause (optional)
    let group_by = if self.match_token(TokenType::Group) {
        self.consume_token(TokenType::By, "Expected BY after GROUP")?;
        Some(self.parse_group_by_columns()?)
    } else {
        None
    };
    
    // Should be at end of statement
    self.consume_token(TokenType::EOF, "Expected end of statement")?;
    
    Ok(SelectStatement {
        select_items,
        from_table,
        where_clause,
        group_by,
    })
}
```

### 4.6 Parsing SELECT Items

```rust
fn parse_select_items(&mut self) -> Result<Vec<SelectItem>> {
    let mut items = Vec::new();
    
    // First item (required)
    items.push(self.parse_select_item()?);
    
    // Additional items separated by commas
    while self.match_token(TokenType::Comma) {
        items.push(self.parse_select_item()?);
    }
    
    Ok(items)
}

fn parse_select_item(&mut self) -> Result<SelectItem> {
    if self.match_token(TokenType::Asterisk) {
        Ok(SelectItem::Wildcard)
    } else {
        let expr = self.parse_expression()?;
        Ok(SelectItem::Expression(expr))
    }
}
```

**Examples:**

```sql
-- Becomes [Wildcard]
SELECT * FROM users

-- Becomes [Expression(Column("name")), Expression(Column("age"))]
SELECT name, age FROM users

-- Becomes [Expression(AggregateFunction { function: "COUNT", argument: Column("*") })]
SELECT COUNT(*) FROM users
```

---

## Chapter 5: Expression Parsing & Operator Precedence

### 5.1 Operator Precedence

Operator precedence determines the order in which operations are evaluated when there are multiple operators in an expression.

#### Precedence Hierarchy (Highest to Lowest)

```
1. Primary expressions (literals, identifiers, parentheses, functions)
2. Unary operators (NOT, -)
3. Multiplicative (*, /)
4. Additive (+, -)
5. Comparison (=, !=, <, >, <=, >=)
6. Logical AND
7. Logical OR
```

#### Examples

```sql
-- Without precedence, this is ambiguous:
age > 25 AND city = 'New York'

-- With precedence (AND has higher precedence than OR, but lower than >):
((age > 25) AND (city = 'New York'))

-- Another example:
age + 10 > 25

-- With precedence (+ has higher precedence than >):
((age + 10) > 25)
```

### 5.2 Implementing Precedence

We implement precedence through the order of function calls:

```rust
fn parse_expression(&mut self) -> Result<Expression> {
    self.parse_or_expression()  // Start with lowest precedence
}

fn parse_or_expression(&mut self) -> Result<Expression> {
    let mut left = self.parse_and_expression()?;
    
    while self.match_token(TokenType::Or) {
        let right = self.parse_and_expression()?;
        left = Expression::BinaryOp {
            left: Box::new(left),
            operator: BinaryOperator::Or,
            right: Box::new(right),
        };
    }
    
    Ok(left)
}

fn parse_and_expression(&mut self) -> Result<Expression> {
    let mut left = self.parse_comparison_expression()?;
    
    while self.match_token(TokenType::And) {
        let right = self.parse_comparison_expression()?;
        left = Expression::BinaryOp {
            left: Box::new(left),
            operator: BinaryOperator::And,
            right: Box::new(right),
        };
    }
    
    Ok(left)
}

// ... continue for comparison, additive, multiplicative, unary, primary
```

### 5.3 Left-Associative Operators

Operators like `+` and `AND` are **left-associative**, meaning:

```sql
-- This is parsed as:
age + 10 + 5

-- Like this:
((age + 10) + 5)
```

Our implementation handles this correctly:

```rust
fn parse_additive_expression(&mut self) -> Result<Expression> {
    let mut left = self.parse_multiplicative_expression()?;
    
    while self.match_token(TokenType::Plus) {  // ← Note: while loop
        let right = self.parse_multiplicative_expression()?;
        left = Expression::BinaryOp {
            left: Box::new(left),   // ← Previous result becomes left operand
            operator: BinaryOperator::Plus,
            right: Box::new(right),
        };
    }
    
    Ok(left)
}
```

### 5.4 Parsing Comparison Operators

```rust
fn parse_comparison_expression(&mut self) -> Result<Expression> {
    let mut left = self.parse_additive_expression()?;
    
    while let Some(token_type) = self.peek_token_type() {
        let operator = match token_type {
            TokenType::Equal => {
                self.advance();
                BinaryOperator::Equal
            }
            TokenType::NotEqual => {
                self.advance();
                BinaryOperator::NotEqual
            }
            TokenType::Less => {
                self.advance();
                BinaryOperator::Less
            }
            TokenType::Greater => {
                self.advance();
                BinaryOperator::Greater
            }
            TokenType::LessEqual => {
                self.advance();
                BinaryOperator::LessEqual
            }
            TokenType::GreaterEqual => {
                self.advance();
                BinaryOperator::GreaterEqual
            }
            _ => break,
        };
        
        let right = self.parse_additive_expression()?;
        left = Expression::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }
    
    Ok(left)
}
```

### 5.5 Parsing Unary Operators

```rust
fn parse_unary_expression(&mut self) -> Result<Expression> {
    if let Some(token_type) = self.peek_token_type() {
        match token_type {
            TokenType::Not => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                return Ok(Expression::UnaryOp {
                    operator: UnaryOperator::Not,
                    operand: Box::new(operand),
                });
            }
            TokenType::Minus => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                return Ok(Expression::UnaryOp {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(operand),
                });
            }
            _ => {}
        }
    }
    
    self.parse_primary_expression()
}
```

**Note:** We call `parse_unary_expression()` recursively to handle:
```sql
NOT NOT age      → NOT (NOT (age))
- - age          → - (- (age))
NOT (age > 25)   → NOT (BinaryOp(age > 25))
```

### 5.6 Primary Expressions

Primary expressions are the base case of our recursion:

```rust
fn parse_primary_expression(&mut self) -> Result<Expression> {
    let token_type = self.peek_token_type();
    
    match token_type {
        Some(TokenType::Identifier(name)) => {
            let name = name.clone();
            self.advance();
            // Check if this is a function call
            if self.match_token(TokenType::LeftParen) {
                self.parse_aggregate_function(name)
            } else {
                Ok(Expression::Column(name))
            }
        }
        Some(TokenType::StringLiteral(value)) => {
            let value = value.clone();
            self.advance();
            Ok(Expression::StringLiteral(value))
        }
        Some(TokenType::NumberLiteral(value)) => {
            let value = value.clone();
            self.advance();
            Ok(Expression::NumberLiteral(value))
        }
        Some(TokenType::Asterisk) => {
            self.advance();
            Ok(Expression::Column("*".to_string()))
        }
        Some(TokenType::LeftParen) => {
            self.advance();
            let expr = self.parse_expression()?;
            self.consume_token(TokenType::RightParen, "Expected ')' after expression")?;
            Ok(expr)
        }
        Some(t) => Err(DatabaseError::parser_error(format!(
            "Unexpected token: {:?}", t
        ))),
        None => Err(DatabaseError::parser_error("Unexpected end of input")),
    }
}
```

---

## Chapter 6: SQL Clause Parsing

### 6.1 SELECT Clause

The SELECT clause specifies which columns or expressions to return:

```rust
fn parse_select_items(&mut self) -> Result<Vec<SelectItem>> {
    let mut items = Vec::new();
    
    items.push(self.parse_select_item()?);
    
    while self.match_token(TokenType::Comma) {
        items.push(self.parse_select_item()?);
    }
    
    Ok(items)
}
```

**Examples:**

```sql
-- Wildcard
SELECT * FROM users

-- Single column
SELECT name FROM users

-- Multiple columns
SELECT name, age, city FROM users

-- Expressions
SELECT age + 10, UPPER(name) FROM users

-- Aggregate functions
SELECT COUNT(*), AVG(age) FROM users

-- Mixed
SELECT city, COUNT(*), AVG(age) FROM users
```

### 6.2 FROM Clause

The FROM clause specifies which table to query:

```rust
fn parse_select_statement(&mut self) -> Result<SelectStatement> {
    // ... parse SELECT items
    self.consume_token(TokenType::From, "Expected FROM")?;
    let from_table = self.parse_identifier()?;
    // ...
}
```

**Examples:**

```sql
SELECT * FROM users
SELECT * FROM orders
SELECT * FROM products
```

### 6.3 WHERE Clause

The WHERE clause filters rows based on conditions:

```rust
// In parse_select_statement:
let where_clause = if self.match_token(TokenType::Where) {
    Some(self.parse_expression()?)
} else {
    None
};
```

**Examples:**

```sql
-- Simple comparison
SELECT * FROM users WHERE age > 25

-- Multiple conditions
SELECT * FROM users WHERE age > 25 AND city = 'New York'

-- Complex expression
SELECT * FROM users WHERE (age > 25 OR city = 'New York') AND status = 'active'

-- With NOT
SELECT * FROM users WHERE NOT (age < 18)
```

### 6.4 GROUP BY Clause

The GROUP BY clause groups rows that have the same values:

```rust
fn parse_group_by_columns(&mut self) -> Result<Vec<String>> {
    let mut columns = Vec::new();
    
    columns.push(self.parse_identifier()?);
    
    while self.match_token(TokenType::Comma) {
        columns.push(self.parse_identifier()?);
    }
    
    Ok(columns)
}
```

**Examples:**

```sql
-- Single column
SELECT city, COUNT(*) FROM users GROUP BY city

-- Multiple columns
SELECT city, status, COUNT(*) FROM users GROUP BY city, status
```

### 6.5 Complete Query Example

```sql
SELECT 
    city, 
    COUNT(*) AS user_count, 
    AVG(age) AS avg_age
FROM 
    users
WHERE 
    age >= 18 
    AND status = 'active'
GROUP BY 
    city
```

**Parsed AST:**

```rust
Query::Select(SelectStatement {
    select_items: [
        Expression(Column("city")),
        Expression(AggregateFunction {
            function: "COUNT",
            argument: Column("*"),
        }),
        Expression(AggregateFunction {
            function: "AVG",
            argument: Column("age"),
        }),
    ],
    from_table: "users",
    where_clause: Some(BinaryOp {
        left: Box::new(BinaryOp {
            left: Box::new(Column("age")),
            operator: GreaterEqual,
            right: Box::new(NumberLiteral("18")),
        }),
        operator: And,
        right: Box::new(BinaryOp {
            left: Box::new(Column("status")),
            operator: Equal,
            right: Box::new(StringLiteral("active")),
        }),
    }),
    group_by: Some(vec!["city"]),
})
```

---

## Chapter 7: Aggregate Function Parsing

### 7.1 What are Aggregate Functions?

Aggregate functions perform calculations on a set of values and return a single value:

```sql
COUNT(*)      -- Count all rows
SUM(salary)   -- Sum of all salary values
AVG(age)      -- Average age
MIN(price)    -- Minimum price
MAX(price)    -- Maximum price
```

### 7.2 Parsing Aggregate Functions

```rust
fn parse_aggregate_function(&mut self, function: String) -> Result<Expression> {
    let argument = self.parse_expression()?;
    self.consume_token(
        TokenType::RightParen,
        "Expected ')' after aggregate function argument",
    )?;
    
    Ok(Expression::AggregateFunction {
        function,
        argument: Box::new(argument),
    })
}
```

### 7.3 COUNT(*)

The asterisk in `COUNT(*)` is special:

```sql
COUNT(*)  -- Counts all rows, including NULL values
COUNT(age) -- Counts non-NULL values in age column
```

In our parser, we handle this by treating `*` as a column named `"*"`:

```rust
fn parse_primary_expression(&mut self) -> Result<Expression> {
    // ...
    Some(TokenType::Asterisk) => {
        self.advance();
        Ok(Expression::Column("*".to_string()))  // Treat as column "*"
    }
    // ...
}
```

Then in the execution engine, we can check for this special case.

### 7.4 Aggregate Function Arguments

Aggregate functions can take different types of arguments:

```sql
-- Wildcard (COUNT only)
COUNT(*)

-- Column reference
COUNT(age)
SUM(salary)
AVG(price)

-- Expression (not common, but possible)
SUM(quantity * unit_price)
```

Our parser allows any expression as the argument:

```rust
fn parse_aggregate_function(&mut self, function: String) -> Result<Expression> {
    let argument = self.parse_expression()?;  // ← Any expression
    // ...
}
```

### 7.5 Supported Aggregate Functions

Our parser supports five aggregate functions:

| Function | Description | Return Type |
|----------|-------------|-------------|
| COUNT | Count non-NULL values | Int64 |
| SUM | Sum numeric values | Int64/Float64 |
| AVG | Average of values | Float64 |
| MIN | Minimum value | Same as input |
| MAX | Maximum value | Same as input |

**Examples:**

```sql
SELECT COUNT(*) FROM users
SELECT SUM(salary) FROM employees
SELECT AVG(age) FROM users
SELECT MIN(price) FROM products
SELECT MAX(price) FROM products
```

### 7.6 Multiple Aggregate Functions

You can use multiple aggregate functions in one query:

```sql
SELECT 
    COUNT(*) AS total_users,
    SUM(age) AS total_age,
    AVG(age) AS avg_age,
    MIN(age) AS min_age,
    MAX(age) AS max_age
FROM users
```

---

## Chapter 8: Error Handling in Parsers

### 8.1 Error Types in Parsers

Parsers need to handle various error scenarios:

1. **Syntax Errors**: Invalid SQL syntax
2. **Unexpected Tokens**: Token doesn't match expected pattern
3. **Unterminated Constructs**: Unclosed parentheses, strings
4. **Incomplete Statements**: Missing required clauses

### 8.2 Error Messages

Good error messages should include:
- What was expected
- What was found
- Where the error occurred (line and column)

```rust
// Good error:
"Expected 'FROM' at line 2, column 5"

// Bad error:
"Syntax error"
```

### 8.3 Implementation

```rust
fn consume_token(&mut self, expected: TokenType, error_message: &str) -> Result<()> {
    if let Some(token) = self.peek_token() {
        if token.token_type == expected {
            self.advance();
            return Ok(());
        }
        
        return Err(DatabaseError::parser_error(format!(
            "{} at line {}, column {}",
            error_message, token.line, token.column
        )));
    }
    
    Err(DatabaseError::parser_error(format!(
        "{} (found EOF)",
        error_message
    )))
}
```

### 8.4 Common Parser Errors

#### 8.4.1 Missing Required Clause

```sql
-- Invalid
SELECT name users

-- Error: Expected 'FROM' at line 1, column 13
```

#### 8.4.2 Unterminated String

```sql
-- Invalid
SELECT 'hello FROM users

-- Error: Unterminated string literal
```

#### 8.4.3 Unexpected Token

```sql
-- Invalid
SELECT * FROM WHERE age > 25

-- Error: Expected identifier, found WHERE at line 1, column 15
```

#### 8.4.4 Missing Parentheses

```sql
-- Invalid
SELECT COUNT* FROM users

-- Error: Expected '(' after aggregate function
```

### 8.5 Error Recovery (Advanced)

Error recovery allows the parser to continue after finding errors and report multiple errors at once. This is more complex but provides a better user experience.

**Simple error recovery strategy:**

```rust
fn synchronize(&mut self) {
    // Skip tokens until we find a synchronization point
    while let Some(token_type) = self.peek_token_type() {
        match token_type {
            TokenType::Semicolon | TokenType::EOF => {
                self.advance();
                return;
            }
            // Keywords that can start a new statement
            TokenType::Select => return,
            _ => self.advance(),
        }
    }
}
```

---

## Chapter 9: Testing Parsers

### 9.1 Why Test Parsers?

Parser testing is crucial because:
- Parsers are complex with many edge cases
- Errors can be subtle and hard to find
- SQL has many valid and invalid combinations
- Good tests serve as documentation

### 9.2 Test Categories

#### 9.2.1 Unit Tests for Tokenizer

Test individual tokenization scenarios:

```rust
#[test]
fn test_tokenizer_keywords() {
    let mut tokenizer = Tokenizer::new("SELECT FROM WHERE GROUP BY");
    let tokens = tokenizer.tokenize().unwrap();
    
    assert_eq!(tokens[0].token_type, TokenType::Select);
    assert_eq!(tokens[1].token_type, TokenType::From);
    assert_eq!(tokens[2].token_type, TokenType::Where);
    assert_eq!(tokens[3].token_type, TokenType::Group);
    assert_eq!(tokens[4].token_type, TokenType::By);
}
```

#### 9.2.2 Unit Tests for Parser

Test parsing of individual constructs:

```rust
#[test]
fn test_basic_select() {
    let mut parser = Parser::new("SELECT * FROM users");
    let result = parser.parse();
    
    assert!(result.is_ok());
    let query = result.unwrap();
    
    match query {
        Query::Select(stmt) => {
            assert_eq!(stmt.select_items.len(), 1);
            assert!(matches!(stmt.select_items[0], SelectItem::Wildcard));
            assert_eq!(stmt.from_table, "users");
        }
        _ => panic!("Expected SELECT query"),
    }
}
```

#### 9.2.3 Integration Tests

Test complete, realistic queries:

```rust
#[test]
fn test_complex_query() {
    let sql = "SELECT city, COUNT(*), AVG(age) FROM users WHERE age >= 18 GROUP BY city";
    let mut parser = Parser::new(sql);
    let result = parser.parse();
    
    assert!(result.is_ok());
    
    let query = result.unwrap();
    let Query::Select(stmt) = query;
    
    assert_eq!(stmt.select_items.len(), 3);
    assert!(stmt.where_clause.is_some());
    assert!(stmt.group_by.is_some());
}
```

### 9.3 Test Coverage Goals

Aim for coverage of:
- All token types
- All SQL clauses (SELECT, FROM, WHERE, GROUP BY)
- All operators (=, !=, <, >, <=, >=, AND, OR, NOT, +, -, *, /)
- All aggregate functions (COUNT, SUM, AVG, MIN, MAX)
- All expression types
- Error cases (invalid syntax, unexpected tokens)
- Edge cases (empty strings, negative numbers, etc.)

### 9.4 Property-Based Testing

Consider using property-based testing (e.g., with `proptest`) to test parser invariants:

```rust
// Property: Parsing and re-serializing should round-trip
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_parse_roundtrip(sql in "[a-zA-Z]+\\s*(\\*|[a-zA-Z]+)\\s*FROM\\s*[a-zA-Z]+") {
            let parser = Parser::new(&sql);
            let result = parser.parse();
            
            // If it parses, we should be able to re-serialize
            if let Ok(query) = result {
                let serialized = query.to_string();
                let parser2 = Parser::new(&serialized);
                assert!(parser2.parse().is_ok());
            }
        }
    }
}
```

---

## Chapter 10: Best Practices & Design Patterns

### 10.1 Parser Design Patterns

#### 10.1.1 Single Responsibility

Each parsing function should handle one grammar rule:

```rust
// Good: Each function has one responsibility
fn parse_select_statement() → Handles entire SELECT statement
fn parse_select_items() → Handles SELECT clause items
fn parse_expression() → Handles expressions

// Bad: One function does too much
fn parse_select() → Handles SELECT, FROM, WHERE, GROUP BY, expressions, etc.
```

#### 10.1.2 Prefer Immutability

Parse the input without mutating it:

```rust
// Good: Read from token stream without modifying
fn parse_expression(&mut self) -> Result<Expression> {
    let left = self.parse_additive_expression()?;
    // ...
}

// Good: Use index instead of removing tokens
pub struct Parser {
    tokens: Vec<Token>,  // Immutable vector
    position: usize,     // Mutable index
}
```

#### 10.1.3 Use Pattern Matching

Leverage Rust's pattern matching for clean code:

```rust
// Good: Clear pattern matching
match token_type {
    TokenType::Select => self.parse_select(),
    TokenType::Insert => self.parse_insert(),
    TokenType::Update => self.parse_update(),
    _ => Err(error),
}

// Less clear: if-else chains
if token_type == TokenType::Select {
    self.parse_select()
} else if token_type == TokenType::Insert {
    self.parse_insert()
} else {
    // ...
}
```

### 10.2 Performance Considerations

#### 10.2.1 Avoid String Allocations

Minimize unnecessary string allocations:

```rust
// Bad: Allocates new string for each comparison
if token.value() == "SELECT".to_string() { ... }

// Good: Use enum comparison
if token.token_type == TokenType::Select { ... }

// Good: Compare with &str if needed
if matches!(token.token_type, TokenType::Identifier(s) if s == "users") { ... }
```

#### 10.2.2 Pre-allocate Collections

When possible, pre-allocate with capacity:

```rust
// Good: Pre-allocate if size is known
let mut items = Vec::with_capacity(5);
items.push(self.parse_select_item()?);

// Good: Pre-allocate with estimated size
let mut tokens = Vec::with_capacity(input.len() / 4);  // ~4 chars per token
```

#### 10.2.3 Use Cow for Conditionally-Owned Data

Use `Cow` when data might be borrowed or owned:

```rust
use std::borrow::Cow;

enum TokenValue {
    Identifier(Cow<'static, str>),  // Can be borrowed or owned
    StringLiteral(String),
    NumberLiteral(String),
}
```

### 10.3 Error Handling Best Practices

#### 10.3.1 Provide Context

Include location information in errors:

```rust
// Good: Includes line and column
Err(DatabaseError::parser_error(format!(
    "Expected 'FROM' at line {}, column {}",
    line, column
)))

// Bad: Generic error
Err(DatabaseError::parser_error("Syntax error"))
```

#### 10.3.2 Use Specific Error Types

Create specific error variants for different error cases:

```rust
pub enum DatabaseError {
    ParserError(String),           // General parser errors
    TokenizerError(String),         // Tokenizer-specific errors
    UnexpectedToken(String),        // Specific token errors
    UnterminatedString,             // Specific construct errors
    // ...
}
```

### 10.4 Documentation

#### 10.4.1 Document Grammar Rules

Document the grammar rules your parser implements:

```rust
/// Parses a SELECT statement according to the grammar:
/// 
/// select_statement ::= SELECT select_items 
///                      FROM identifier 
///                      [WHERE expression] 
///                      [GROUP BY identifier_list]
fn parse_select_statement(&mut self) -> Result<SelectStatement> {
    // ...
}
```

#### 10.4.2 Provide Examples

Include usage examples in documentation:

```rust
/// Parses a SQL query string into an AST.
///
/// # Examples
///
/// ```
/// use mini_rust_olap::parser::Parser;
/// 
/// let mut parser = Parser::new("SELECT * FROM users");
/// let query = parser.parse().unwrap();
/// 
/// // Use the parsed query...
/// ```
pub fn parse(&mut self) -> Result<Query> {
    // ...
}
```

### 10.5 Testing Best Practices

#### 10.5.1 Test Error Cases

Don't just test success cases:

```rust
#[test]
fn test_missing_from_clause() {
    let mut parser = Parser::new("SELECT * users");
    let result = parser.parse();
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Expected FROM"));
}
```

#### 10.5.2 Test Edge Cases

Test boundary conditions:

```rust
#[test]
fn test_empty_string_literal() {
    let mut parser = Parser::new("SELECT '' FROM users");
    let result = parser.parse();
    
    assert!(result.is_ok());
    // Check that empty string is handled correctly
}
```

#### 10.5.3 Use Test Helpers

Create helper functions to reduce test duplication:

```rust
fn parse_sql(sql: &str) -> Query {
    let mut parser = Parser::new(sql);
    parser.parse().unwrap()
}

#[test]
fn test_basic_select() {
    let query = parse_sql("SELECT * FROM users");
    // ... assertions
}

#[test]
fn test_select_with_where() {
    let query = parse_sql("SELECT * FROM users WHERE age > 25");
    // ... assertions
}
```

---

## Chapter 11: Learning Outcomes

### 11.1 Rust Concepts Mastered

By completing Phase 5, you should now understand:

✅ **Pattern Matching**
- Using `match` with enums
- Pattern guards
- Destructuring

✅ **Error Handling**
- Custom error types with `thiserror`
- `Result` types
- Error propagation with `?`

✅ **Ownership and Borrowing**
- Managing references in parsers
- When to clone vs borrow
- Lifetime considerations

✅ **Traits**
- Implementing traits for custom types
- Using trait objects (`Box<dyn Trait>`)
- Derive macros (`Debug`, `Clone`, `PartialEq`)

✅ **Advanced Enums**
- Enum with data variants
- Recursive types (with `Box`)
- Pattern matching on enums

### 11.2 Database Concepts Mastered

✅ **SQL Parsing**
- Tokenization/Lexing
- Abstract Syntax Trees (AST)
- Recursive descent parsing
- Operator precedence

✅ **SQL Syntax**
- SELECT clause
- FROM clause
- WHERE clause
- GROUP BY clause
- Aggregate functions

✅ **Expression Parsing**
- Column references
- Literals (strings, numbers)
- Binary operations
- Unary operations
- Parenthesized expressions

### 11.3 Systems Programming

✅ **State Machines**
- Implementing tokenizers
- Managing state transitions
- Handling edge cases

✅ **Parsers**
- Top-down parsing
- Recursive descent
- Grammar rules
- Error recovery

✅ **Testing Strategies**
- Unit testing
- Integration testing
- Property-based testing (advanced)

### 11.4 Practical Skills

✅ **Code Organization**
- Structuring large modules
- Separating concerns (tokenizer vs parser)
- Documentation

✅ **Debugging**
- Debugging parsers
- Writing helpful error messages
- Understanding parse errors

✅ **Performance**
- Minimizing allocations
- Efficient string handling
- Pre-allocation strategies

---

## Chapter 12: Practical Exercises

### 12.1 Beginner Exercises

#### Exercise 1: Extend Tokenizer

Add support for these additional tokens:
- Semicolon (`;`)
- Double quotes (`"`) for string literals
- Comments (`--` for line comments, `/* */` for block comments)

**Hints:**
- Add new token types to `TokenType` enum
- Update `next_token()` method
- Test each new token type

#### Exercise 2: Add ORDER BY Clause

Extend the parser to support ORDER BY:

```sql
SELECT * FROM users ORDER BY age DESC
SELECT * FROM users ORDER BY name, age ASC
```

**Hints:**
- Add `order_by` field to `SelectStatement`
- Parse `ORDER BY` after `GROUP BY`
- Handle `ASC` and `DESC` keywords
- Support multiple columns

#### Exercise 3: Add LIMIT Clause

Add support for LIMIT:

```sql
SELECT * FROM users LIMIT 10
SELECT * FROM users LIMIT 10 OFFSET 20
```

**Hints:**
- Add `limit` and `offset` fields to `SelectStatement`
- Parse `LIMIT` keyword
- Parse `OFFSET` keyword (optional)

### 12.2 Intermediate Exercises

#### Exercise 4: Support INSERT Statements

Extend the parser to support INSERT:

```sql
INSERT INTO users (name, age, city) VALUES ('John', 30, 'NYC')
INSERT INTO users VALUES ('Jane', 25, 'LA')
```

**Hints:**
- Add `InsertStatement` variant to `Query` enum
- Parse `INSERT INTO` keywords
- Parse column list (optional)
- Parse `VALUES` keyword
- Parse value lists (parenthesized)

#### Exercise 5: Support DELETE Statements

Extend the parser to support DELETE:

```sql
DELETE FROM users WHERE age < 18
DELETE FROM users
```

**Hints:**
- Add `DeleteStatement` variant to `Query` enum
- Parse `DELETE FROM` keywords
- Parse WHERE clause (optional)

#### Exercise 6: Support UPDATE Statements

Extend the parser to support UPDATE:

```sql
UPDATE users SET age = 30, city = 'LA' WHERE name = 'John'
```

**Hints:**
- Add `UpdateStatement` variant to `Query` enum
- Parse `UPDATE` keyword and table name
- Parse `SET` keyword and assignments
- Parse WHERE clause (optional)

### 12.3 Advanced Exercises

#### Exercise 7: Support JOIN Operations

Extend the parser to support JOIN:

```sql
SELECT * FROM users JOIN orders ON users.id = orders.user_id
SELECT * FROM users LEFT JOIN orders ON users.id = orders.user_id
```

**Hints:**
- Add `join_clause` field to `SelectStatement`
- Parse `JOIN`, `LEFT JOIN`, `RIGHT JOIN`, `INNER JOIN`
- Parse `ON` keyword and join condition
- Support multiple joins

#### Exercise 8: Support Subqueries

Extend the parser to support subqueries:

```sql
SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 100)
SELECT (SELECT COUNT(*) FROM users) AS user_count
```

**Hints:**
- Allow `SELECT` statements as expressions
- Handle nested parentheses
- Update `parse_primary_expression()` to parse subqueries
- Ensure proper scoping

#### Exercise 9: Support UNION

Extend the parser to support UNION:

```sql
SELECT name FROM users
UNION
SELECT name FROM customers
```

**Hints:**
- Add `Union` variant to `Expression` or `Query` enum
- Parse `UNION` keyword
- Parse multiple SELECT statements
- Handle `UNION ALL` option

### 12.4 Performance Exercises

#### Exercise 10: Optimize Tokenizer

Optimize the tokenizer for better performance:
- Pre-allocate token vector
- Use `String::with_capacity()` for literals
- Avoid unnecessary clones
- Benchmark before and after

**Hints:**
- Use `cargo bench` for benchmarking
- Profile with `cargo flamegraph`
- Compare with previous implementation

#### Exercise 11: Implement Error Recovery

Add error recovery to the parser:
- Continue after finding errors
- Report multiple errors at once
- Implement synchronization points

**Hints:**
- Add `synchronize()` method
- Define synchronization tokens (e.g., `;`, keywords)
- Collect errors instead of returning early

#### Exercise 12: Implement Parser Combinators

Refactor using parser combinators (e.g., with `nom` crate):
- Define small combinator functions
- Combine combinators
- Compare with recursive descent approach

**Hints:**
- Study the `nom` crate
- Understand parser combinator concepts
- Refactor incrementally

### 12.5 Testing Exercises

#### Exercise 13: Improve Test Coverage

Aim for 100% code coverage:
- Use `cargo tarpaulin` for coverage reports
- Add tests for uncovered code
- Test all error paths

**Hints:**
- Generate coverage report
- Identify untested branches
- Add targeted tests

#### Exercise 14: Implement Property-Based Tests

Add property-based tests:
- Use `proptest` crate
- Define invariants
- Test random inputs

**Hints:**
- Install `proptest`
- Define SQL grammar generators
- Test round-trip properties

#### Exercise 15: Add Fuzz Testing

Add fuzz testing:
- Use `cargo fuzz`
- Generate random SQL strings
- Find crashes and edge cases

**Hints:**
- Install `cargo-fuzz`
- Create fuzz harness
- Run fuzzing for extended period

---

## 📝 Conclusion

Congratulations on completing Phase 5 of Mini Rust OLAP! You've built a fully functional SQL parser from scratch using recursive descent parsing techniques.

### What You've Accomplished

✅ **Tokenizer**: Implemented a complete SQL tokenizer with 20+ token types
✅ **AST Design**: Created comprehensive AST structures for SQL queries
✅ **Parser**: Built a recursive descent parser for full SELECT syntax
✅ **Error Handling**: Implemented helpful error messages with line/column tracking
✅ **Testing**: Created 19 comprehensive tests with 100% pass rate
✅ **Documentation**: Added extensive inline documentation

### Next Steps

You're now ready for **Phase 6: Query Planning**, where you'll:
- Design a query planner
- Convert AST to execution plans
- Optimize queries
- Execute plans using the operators from Phase 4

### Key Takeaways

1. **Parsers are essential**: They bridge the gap between human-readable SQL and machine-executable code
2. **Design matters**: Good AST design makes both parsing and execution easier
3. **Error handling is critical**: Helpful error messages improve user experience
4. **Testing is vital**: Parsers have many edge cases that must be tested
5. **Incremental development**: Start small and add features incrementally

### Keep Learning

- Study parser theory (LL, LR, LALR grammars)
- Explore parser generators (e.g., `lalrpop`, `pest`)
- Learn about optimization techniques (predicate pushdown, etc.)
- Experiment with different parsing approaches

**Happy parsing! 🦀**