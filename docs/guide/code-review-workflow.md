# Code Review Workflow: Complete Guide
## Mastering Pull Requests and Code Reviews for Collaborative Development

---

## 📚 Table of Contents

1. [Introduction](#1-introduction)
2. [Code Review Fundamentals](#2-code-review-fundamentals)
3. [Git/GitHub Workflow Overview](#3-github-workflow-overview)
4. [Branching Strategies](#4-branching-strategies)
5. [Pull Request Creation](#5-pull-request-creation)
6. [Automated Code Reviews](#6-automated-code-reviews)
7. [Manual Code Review Process](#7-manual-code-review-process)
8. [Code Review Types](#8-code-review-types)
9. [Communication Best Practices](#9-communication-best-practices)
10. [Tools and Platforms](#10-tools-and-platforms)
11. [Advanced Workflows](#11-advanced-workflows)
12. [Troubleshooting](#12-troubleshooting)
13. [Code Review Checklist](#13-code-review-checklist)
14. [Best Practices Summary](#14-best-practices-summary)
15. [Examples](#15-examples)

---

## 1. Introduction

### 1.1 What is a Code Review?

A **code review** is the process of examining source code by one or more people to find mistakes, improve code quality, and ensure it follows best practices. In the context of Pull Requests (PRs), code review is the gatekeeper for code entering the main branch.

**Key Objectives:**
- **Quality Assurance**: Ensure code meets project standards
- **Bug Detection**: Catch issues before they reach production
- **Knowledge Sharing**: Team members learn from each other
- **Mentorship**: Senior developers guide junior developers
- **Maintainability**: Keep codebase clean and understandable

### 1.2 Why Code Reviews Matter

**Benefits:**
1. **Fewer Bugs**: Reviews catch issues early
2. **Better Code Quality**: Multiple perspectives improve design
3. **Knowledge Transfer**: Team learns patterns and anti-patterns
4. **Documentation**: Reviews expose missing or unclear documentation
5. **Consistency**: Ensures team follows same coding standards

**Real-World Impact:**
```
Without Code Review:
  → 50+ bugs per 1000 lines of code
  → 20% of development time fixing bugs
  → Technical debt accumulates rapidly

With Code Review:
  → <10 bugs per 1000 lines of code
  → 5% of development time fixing bugs
  → Technical debt managed effectively
```

### 1.3 Pull Request vs Direct Commits

**Direct Commits (Bad Practice):**
```bash
# Developer commits directly to main
git checkout main
git add .
git commit -m "Add new feature"
git push origin main
```
**Problems:**
- No peer review
- Code may break existing functionality
- Hard to track who made what changes
- Risk of introducing bugs into main branch

**Pull Request (Best Practice):**
```bash
# Developer creates branch
git checkout -b feature/new-tables

# Makes changes
git add .
git commit -m "Add column compression"

# Pushes to remote
git push origin feature/new-tables

# Creates Pull Request (on GitHub)
# → Code review happens here!
# → Merged only after approval
```
**Benefits:**
- Peer review before code enters main
- Discussion happens asynchronously
- Clean git history
- Easy to revert if needed

### 1.4 Roles in Code Review

| Role | Responsibilities |
|-------|----------------|
| **Author** | Writes code, creates PR, responds to feedback |
| **Reviewer** | Reviews PR code, provides feedback, approves/rejects |
| **Maintainer** | Has final say on merging, manages releases |
| **CI Bot** | Runs automated tests, reports results |

---

## 2. Code Review Fundamentals

### 2.1 The Code Review Cycle

```
┌─────────────────────────────────────────────────────────────────┐
│                                                         │
│  ┌─────────────┐        ┌─────────────────────┐        │
│  │   Author    │        │     Reviewer      │        │
│  └──────┬──────┘        └──────┬─────────────┘        │
│         │                       │                    │        │
│         ▼                       ▼                    │        │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │        Code Review Process                 │         │
│  └───────────────────────────────────────────────────────────┘ │
│                         │                            │        │
│                         ▼                            │        │
│  ┌─────────────┐        ┌─────────────────────┐        │
│  │   Main     │◀──────│   Author (updated)  │        │
│  └─────────────┘        └─────────────────────┘        │
│                                                         │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 What to Review

**Code Quality:**
- Correctness: Does it work as intended?
- Style: Does it follow project conventions?
- Efficiency: Is it performant?
- Safety: Are there potential security issues?

**Architecture:**
- Design: Is the approach sound?
- Modularity: Is code properly separated?
- Extensibility: Can it be easily extended?
- Coupling: Are components loosely coupled?

**Documentation:**
- API docs: Are public functions documented?
- Comments: Are complex sections explained?
- Examples: Do code examples exist and work?

**Testing:**
- Coverage: Are all paths tested?
- Quality: Are tests meaningful and thorough?
- Edge cases: Are corner cases handled?

### 2.3 Code Review Mindset

**For Reviewers:**
- **Be Constructive**: Focus on improvement, not criticism
- **Explain Why**: Don't just say "change this" - explain the reasoning
- **Be Specific**: Point to exact lines and explain issues
- **Suggest Alternatives**: Offer solutions, not just problems
- **Ask Questions**: Clarify decisions you don't understand

**For Authors:**
- **Be Humble**: Accept feedback gracefully
- **Ask for Clarification**: Don't be afraid to ask
- **Defend Thoughtfully**: Explain your reasoning if you disagree
- **Iterate**: Make changes incrementally based on feedback
- **Thank Reviewers**: Appreciate their time and effort

### 2.4 Code Review Anti-Patterns

❌ **"LGTM" (Looks Good To Me)**
- **Problem**: Doesn't help improve code
- **Better**: Be specific about what's good or what to improve

❌ **"Nitpicking"**
- **Problem**: Focusing on trivial issues
- **Better**: Prioritize substantive issues

❌ **"Change Everything"**
- **Problem**: Overwhelming, not actionable
- **Better**: Focus on critical path items first

❌ **"Just Use X Library"**
- **Problem**: Doesn't explain why
- **Better**: "Consider using X because it handles Y more efficiently"

✅ **Good Practice:**
```markdown
## Review Comments

### Performance
- The nested loops here are O(n²) in worst case. Consider using a HashMap for O(n) lookups.

### Readability
- The `calculate_aggregate` function is quite long. Consider splitting it into smaller helper functions for clarity.

### Safety
- This slice operation could panic if `end > len(data)`. Consider adding bounds checking.

### Architecture
- Consider extracting the validation logic into a separate module to improve separation of concerns.
```

---

## 3. GitHub Workflow Overview

### 3.1 Git Workflow Models

**Centralized Workflow (GitHub Default):**
```
Developer → Fork → Branch → Commit → Push → PR → Review → Merge
                └─────────────────────────────┘
                        ↓
                (to upstream repo)
```

**Fork-and-Branch Workflow:**
```
Upstream Repo
    ↓ (Fork)
Fork Repo (Yours)
    ↓ (Clone)
Local Repo
    ↓ (Branch)
Feature Branch
    ↓ (Commit & Push)
Fork (Remote)
    ↓ (Create PR)
Upstream Repo (Main)
```

### 3.2 Remote Repositories Explained

**Upstream (Original Repository):**
- The official project repository
- Maintained by project owners
- Where PRs are submitted
- Example: `github.com/org/mini_rust_olap`

**Origin (Your Fork):**
- Your personal copy of the upstream repo
- You push changes here
- Example: `github.com/yourname/mini_rust_olap`

**Remote Configuration:**
```bash
# View remotes
git remote -v

# Typical output:
# origin    https://github.com/yourname/mini_rust_olap.git (fetch)
# origin    https://github.com/yourname/mini_rust_olap.git (push)
# upstream  https://github.com/org/mini_rust_olap.git (fetch)

# Add upstream (one-time setup)
git remote add upstream https://github.com/org/mini_rust_olap.git
```

### 3.3 Syncing with Upstream

**Why Sync?**
- Get latest changes from main branch
- Avoid merge conflicts later
- Stay up-to-date with project

**Sync Process:**
```bash
# 1. Fetch from upstream
git fetch upstream

# 2. Checkout main (or your development branch)
git checkout main

# 3. Merge or rebase upstream changes
git merge upstream/main
# OR
git rebase upstream/main

# 4. Push to your fork
git push origin main
```

**Before Starting New Work:**
```bash
# Always start from a clean, updated main
git fetch upstream
git checkout main
git pull upstream main
git checkout -b feature/new-work
```

---

## 4. Branching Strategies

### 4.1 Feature Branch Workflow

**Recommended for Open Source Projects:**
```
main (protected branch)
  ├─ feature/authentication
  ├─ feature/database-migration
  ├─ feature/new-api
  └─ ...
```

**Best Practices:**
- Short-lived branches (days to weeks)
- One feature per branch
- Descriptive names: `feature/add-user-auth`, not `fix-stuff`
- Delete branch after merge: `git branch -d feature/add-user-auth`

**Workflow:**
```bash
# Start from updated main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/add-column-compression

# Make changes
# ... work ...

# Commit and push
git add .
git commit -m "Add column compression using LZ4"
git push origin feature/add-column-compression

# Create PR (on GitHub)
# Select feature/add-column-compression → main
```

### 4.2 Branch Naming Conventions

**Good Names:**
```bash
feature/add-user-authentication
bugfix/memory-leak-in-column
refactor/optimize-query-execution
docs/update-readme
chore/update-dependencies
hotfix/critical-security-patch
```

**Bad Names:**
```bash
fix              # Too vague
new-branch       # Not descriptive
stuff            # Unprofessional
my-changes       # Doesn't describe purpose
```

### 4.3 Long-Running Branches

**Feature Branches vs Long-Running Branches:**

| Type | Duration | When to Use | Pros | Cons |
|-------|---------|--------------|------|------|
| **Feature Branch** | Days to weeks | New features | Clean history | More branches |
| **Long-Running** | Weeks to months | Continuous integration | Always up-to-date | Complex conflicts |
| **Release Branch** | Release cycle | Preparing for release | Isolated | Stale quickly |

**Recommendation:** Use feature branches for most work. Use long-running branches only for continuous integration work or release preparation.

### 4.4 Protected Branches

**What are Protected Branches?**
Branch rules that enforce certain conditions before allowing direct commits or merges.

**Common Protections:**
- **Require pull request**: Prevents direct commits to main
- **Require status checks**: CI must pass
- **Require review**: At least N approvals required
- **Require conversation resolution**: All review comments must be resolved
- **Require up-to-date**: Branch must be current with main

**Setting Up Protected Branches (GitHub):**
1. Go to repository Settings → Branches
2. Find `main` branch
3. Click "Edit" (pencil icon)
4. Check protections:
   - [x] Require a pull request before merging
   - [x] Require branches to be up to date before merging
   - [x] Require status checks to pass before merging
   - [ ] Require linear history (optional)
5. Add required reviewers if desired
6. Click "Save changes"

---

## 5. Pull Request Creation

### 5.1 Pre-PR Checklist

Before creating a PR, ensure:

```markdown
## Pre-PR Checklist

### Code Quality
- [ ] All tests pass locally (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Builds successfully (`cargo check`)
- [ ] Documentation is updated

### Testing
- [ ] New functionality has tests
- [ ] Edge cases are covered
- [ ] Integration tests pass
- [ ] Manual testing completed

### Documentation
- [ ] README.md updated (if user-facing)
- [ ] CHANGELOG.md updated (if breaking changes)
- [ ] Inline comments for complex code
- [ ] API documentation for public functions

### Cleanliness
- [ ] No debug prints or TODOs in final code
- [ ] No commented-out code
- [ ] No unnecessary files committed
- [ ] `target/` not included
```

### 5.2 Writing a Great PR Title

**Formula:** `[Type]: [Component] [What Changed]`

**Good Examples:**
```markdown
# Feature
feat: Add column compression using LZ4 algorithm

# Bugfix
fix: Memory leak in `Column::get()` method

# Refactor
refactor: Simplify `DataType` enum with derive macros

# Documentation
docs: Update README with new usage examples

# Performance
perf: Optimize `GROUP BY` query execution

# Breaking Change
breaking!: Change `Column::get()` return type from reference to owned
```

**Bad Examples:**
```markdown
# Too vague
Update

# Not descriptive
Fixed stuff

# Doesn't follow convention
add new function
```

### 5.3 Writing a Great PR Description

**Template:**
```markdown
## Changes
Brief description of what this PR does and why it's needed.

## Related Issues
Closes #123
Related to #456

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement

## Motivation
Why this change is necessary. What problem does it solve?

## Implementation Details
High-level overview of how the change works.

## Screenshots / Diagrams
If applicable, include screenshots or diagrams.

## Testing
Describe how this was tested.

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Added tests for new functionality
- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG updated (if breaking)
```

**Example Description:**
```markdown
## Changes
This PR adds column compression to reduce memory usage for string columns.

## Related Issues
Closes #234: "Reduce memory footprint of StringColumn"

## Type of Change
- [ ] New feature

## Motivation
StringColumn currently stores all strings individually, leading to high memory usage and poor cache locality. This PR introduces dictionary encoding to compress repeated strings, similar to how ClickHouse handles string data.

## Implementation Details
- Added `DictionaryColumn` struct with HashMap for string → ID mapping
- Modified `StringColumn` to use dictionary for values >10 chars
- Compression ratio: ~3-5x for typical datasets
- No API changes - fully backward compatible

## Performance Impact
- Memory: 60-80% reduction for datasets with many repeated strings
- Read speed: ~5% overhead for compression/decompression
- Write speed: ~10% overhead for building dictionary

## Testing
- Added `compression_test` module
- Tested with synthetic data (100K rows, 100 unique values)
- Verified no data loss in compression/decompression cycle
- Ran existing integration tests: All pass

## Breaking Changes
None - fully backward compatible

## Checklist
- [x] Code follows style guidelines (clippy clean)
- [x] Self-review completed
- [x] Added tests for compression logic
- [x] Added tests for edge cases (empty strings, very long strings)
- [x] All tests pass (cargo test)
- [x] Documentation updated (inline comments)
- [x] Performance benchmarks added
- [ ] CHANGELOG updated (not breaking change)
```

### 5.4 Linking Issues and Commits

**Closes Issues:**
- In PR description: `Closes #123` or `Fixes #456`
- GitHub will automatically close the issue when PR merges

**Reference Commits:**
```markdown
Ref: abc123 (Initial implementation)
Ref: def456 (Fixed edge case for empty strings)
Ref: ghi789 (Performance optimization)
```

### 5.5 Adding Screenshots and Demos

**When to Add:**
- UI changes
- New features with visual component
- Complex workflows
- Performance improvements (with before/after metrics)

**How to Add:**
```markdown
## Demo

![Demo GIF](https://i.imgur.com/example.gif)

Compressing 1M rows of string data:
Before: 156MB
After: 28MB (82% reduction)
```

---

## 6. Automated Code Reviews

### 6.1 Your CI Pipeline as Automated Review

**How Our Pre-Push Hook Helps:**
```bash
# Your pre-push hook (already set up!) automatically:
# ✓ Runs all tests
# ✓ Checks formatting
# ✓ Runs clippy
# ✓ Validates documentation
# ✓ Prevents pushing bad code

# Before PR creation, run:
git push origin feature/new-tables

# Hook runs all checks...
# Only passes if everything is perfect!
```

### 6.2 GitHub Actions (Optional)

**Setting Up GitHub Actions for PRs:**

Create `.github/workflows/pr-check.yml`:
```yaml
name: Pull Request Checks

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-
      - name: Build
        run: cargo build --verbose
      - name: Run tests
        run: cargo test --verbose
      - name: Run clippy
        run: cargo clippy --all-targets
      - name: Check formatting
        run: cargo fmt --all -- --check
```

**Benefits of GitHub Actions:**
- Runs on fresh environment (catches platform-specific issues)
- Provides public CI status badge on PR
- Runs automatically for every PR
- Free for open source

### 6.3 Code Coverage Tools

**Coveralls:**
- Shows code coverage reports on PR
- Tracks coverage changes
- Highlights untested new code

**Codecov:**
- Similar to Coveralls
- More advanced features
- Supports multiple languages

**Setting Up:**
```toml
# In Cargo.toml
[workspace.metadata]
coveralls = { repo-token = "PROJECT_TOKEN" }

# Then add coverage command to GitHub Actions
```

### 6.4 Linting Tools

**Additional Linters Beyond Clippy:**
- **rustfmt**: Formatting (you already have this!)
- **clippy**: Rust-specific linter (you already have this!)
- **cargo-deny**: Checks for license compliance and security advisories
- **cargo-udeps**: Checks for outdated dependencies
- **cargo-audit**: Checks for security vulnerabilities

**GitHub Integration:**
```bash
# Install cargo-binstall
cargo install cargo-binstall

# Install linter via binstall
cargo-binstall cargo-udeps
cargo-binstall cargo-audit
```

### 6.5 Pre-commit Frameworks

**What are Pre-commit Frameworks?**
Tools that manage git hooks automatically. Examples:
- **pre-commit**: Python-based, most popular
- **lefthook**: Rust-based, faster
- **husky**: Git hooks made easy

**Example: Using pre-commit with Rust**
```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: fmt
        name: rustfmt
        entry: cargo fmt --
        language: system
        types: [rust]
      - id: clippy
        name: clippy
        entry: cargo clippy --
        language: system
        types: [rust]
        pass_filenames: true
```

---

## 7. Manual Code Review Process

### 7.1 Getting Notified of PR

**How to Know About New PRs:**
1. **Email Notifications** - GitHub sends email to watchers
2. **GitHub Desktop App** - Shows PRs in app
3. **GitHub Web** - Shows PRs in Pull Requests tab
4. **Slack/Discord Integration** - GitHub bot posts to channel

**Setting Up Notifications:**
1. Go to repository Settings → Notifications
2. Configure which events trigger notifications:
   - [x] Pull requests
   - [x] Pull request reviews
   - [x] Pull request comments

### 7.2 Initial Review Steps

**When You See a New PR:**

**Step 1: Read the Title and Description**
- Quick understanding of what's being changed
- Look at the checklist in the description
- Check if it's something you care about

**Step 2: Look at Changed Files**
- Click "Files changed" tab
- Identify which modules are affected
- Assess impact on code you're familiar with

**Step 3: View the Diff**
- Click "View file" for each changed file
- Use "Rich diff" or "Split" view
- Focus on lines changed, not the entire file

**Step 4: Clone and Test Locally**
```bash
# Clone the branch
git fetch origin
git checkout feature/new-feature

# Run tests
cargo test

# Build
cargo build

# Run specific tests
cargo test module::feature
```

### 7.3 In-Depth Review Process

**What to Look For:**

**1. Correctness**
```rust
// Is this logic correct?
fn calculate_average(values: &[i64]) -> f64 {
    if values.is_empty() {
        return 0.0;  // ❌ BUG: Should return None or handle differently
    }
    
    let sum: i64 = values.iter().sum();
    sum as f64 / values.len() as f64
}
```

**2. Edge Cases**
```rust
// What happens with empty input?
fn process_data(data: Vec<String>) -> Result<String> {
    data[0].clone()  // ❌ BUG: Panics if data is empty
}

// Better
fn process_data(data: Vec<String>) -> Result<String> {
    if data.is_empty() {
        return Err("Data is empty".into());
    }
    Ok(data[0].clone())
}
```

**3. Performance**
```rust
// O(n²) nested loops
fn find_duplicates(data: &Vec<i64>) -> Vec<i64> {
    let mut duplicates = Vec::new();
    for i in data {
        for j in data {
            if i == j && !duplicates.contains(&i) {
                duplicates.push(i);
            }
        }
    }
    duplicates
}

// Better: O(n) using HashMap
use std::collections::HashSet;

fn find_duplicates(data: &Vec<i64>) -> Vec<i64> {
    let seen: HashSet<i64> = data.iter().cloned().collect();
    data.iter().filter(|x| seen.contains(x)).cloned().collect()
}
```

**4. Security**
```rust
// SQL injection vulnerability
fn execute_query(query: &str, conn: &Connection) -> Result<QueryResult> {
    let sql = format!("SELECT * FROM users WHERE name = '{}'", query);
    // ❌ BUG: Vulnerable to SQL injection
    
    // Better: Use prepared statements
    conn.execute("SELECT * FROM users WHERE name = $1", &[query])
}

// Integer overflow
fn multiply(a: i64, b: i64) -> i64 {
    a * b  // ❌ BUG: Can overflow
}

// Better
fn multiply(a: i64, b: i64) -> Option<i64> {
    a.checked_mul(b)
}
```

### 7.4 Providing Feedback

**Good Comment Style:**
```markdown
## Performance
The nested loops in `find_duplicates` are O(n²) in worst case. Consider using a HashSet for O(n) lookups.

## Code snippet
```rust
// Current (slow)
for i in data {
    for j in data {
        // ...
    }
}

// Suggested (fast)
use std::collections::HashSet;
let mut seen: HashSet<i64> = HashSet::new();
for item in data {
    seen.insert(item);
}
```

## Why this matters
With large datasets (100K+ rows), the current implementation can take minutes. The HashSet version completes in milliseconds.

## Alternative
Another option is to use `itertools::unique()` if you're open to adding it as a dependency.
```

**Components of a Good Review Comment:**
1. **Type**: Performance, correctness, style, security, etc.
2. **Problem**: Describe the issue clearly
3. **Evidence**: Show the problematic code snippet
4. **Solution**: Provide improved code
5. **Explanation**: Why the solution is better
6. **Alternatives**: Mention other approaches with pros/cons
7. **Impact**: How this affects the codebase

### 7.5 Common Issues and Solutions

**Issue 1: Code Too Complex**
```rust
// ❌ Too complex, hard to understand
fn process_query(
    sql: &str,
    params: &HashMap<String, String>,
    flags: QueryFlags,
    options: QueryOptions,
    context: &QueryContext,
    cache: Option<&mut QueryCache>,
    tx: Option<&Transaction>,
) -> Result<ResultSet>
{
    // 50 lines of complex logic
}
```
**Solution:**
```rust
// ✅ Split into smaller, focused functions
fn process_query(sql: &str, params: &HashMap<String, String>) -> Result<ResultSet> {
    let options = QueryOptions::default();
    let context = QueryContext::default();
    let mut cache = QueryCache::new();
    
    execute_query_with_context(sql, params, flags, options, context, Some(&mut cache))
}

fn execute_query_with_context(
    sql: &str,
    params: &HashMap<String, String>,
    flags: QueryFlags,
    options: &QueryOptions,
    context: &QueryContext,
    cache: Option<&mut QueryCache>,
) -> Result<ResultSet> {
    // Actual query execution
}
```

**Issue 2: Repeated Code**
```rust
// ❌ Duplicated in multiple places
fn validate_int(value: i64) -> bool {
    value >= 0 && value <= 100
}

fn validate_age(value: i64) -> bool {
    value >= 0 && value <= 120
}

fn validate_score(value: i64) -> bool {
    value >= 0 && value <= 100
}
```
**Solution:**
```rust
// ✅ Extract to reusable function
fn validate_range(value: i64, min: i64, max: i64) -> bool {
    value >= min && value <= max
}

fn validate_int(value: i64) -> bool {
    validate_range(value, 0, 100)
}

fn validate_age(value: i64) -> bool {
    validate_range(value, 0, 120)
}

fn validate_score(value: i64) -> bool {
    validate_range(value, 0, 100)
}
```

**Issue 3: Magic Numbers**
```rust
// ❌ Magic number - what does 42 mean?
fn process_data(data: &Vec<i64>) -> Vec<i64> {
    data.iter().filter(|x| x > 42).cloned().collect()
}
```
**Solution:**
```rust
// ✅ Use named constants
const MAXIMUM_THRESHOLD: i64 = 42;

fn process_data(data: &Vec<i64>) -> Vec<i64> {
    data.iter().filter(|x| x > MAXIMUM_THRESHOLD).cloned().collect()
}
```

### 7.6 Requesting Changes

**When Changes Are Needed:**
1. **Explain why**: "This adds caching to reduce database queries"
2. **Be specific**: "Change line 123 in column.rs to use HashMap"
3. **Offer to make the change**: "I can submit a follow-up PR for this"
4. **Allow for discussion**: "Or would you prefer a different approach?"

**Requesting Changes Template:**
```markdown
## Requested Changes

### Performance Optimization
I noticed the nested loops in `find_duplicates` (lines 156-162) have O(n²) complexity. Could we optimize this using a HashSet for O(n) lookups? This would significantly improve performance for large datasets.

### Code Clarity
The `process_query` function is getting quite large (100+ lines). Would you consider splitting it into smaller helper functions? For example:
- `parse_query_sql()`
- `validate_parameters()`
- `execute_query_plan()`

### Documentation
Some public functions in `Column` trait lack documentation. Could we add examples showing how to use `slice()` and `get()`?

Happy to make these changes or discuss further!
```

---

## 8. Code Review Types

### 8.1 Types of Reviews

| Type | Purpose | Timing | Depth | Participants |
|-------|---------|---------|--------|-------------|
| **Lightweight Review** | Quick validation | <1 hour | 1-2 reviewers |
| **Standard Review** | Thorough validation | 1-2 hours | 1-3 reviewers |
| **In-Depth Review** | Deep dive into logic | 2-4 hours | 1-2 reviewers |
| **Formal Review** | Security/Architecture | 1-3 hours | 2+ reviewers |

### 8.2 Lightweight Review

**When to Use:**
- Trivial bug fixes
- Documentation updates
- Test improvements
- Comment/typo fixes

**Example:**
```markdown
## Review

Looks good! Minor suggestion: line 45 has a trailing space.

Approved! ✅
```

### 8.3 Standard Review

**When to Use:**
- New features
- Bug fixes with complexity
- Refactoring
- Performance improvements

**Process:**
```markdown
## Summary
This PR adds user authentication functionality. Overall, the approach is solid and well-tested.

## Issues Found

### Critical (Must Fix)
1. **Authentication Bypass**: Line 123 in `auth.rs` doesn't check if user is verified before returning `true`. Security issue!

### Important (Should Fix)
1. **Missing Tests**: No tests for password reset functionality.
2. **Error Messages**: Generic error messages don't help users debug issues.

### Suggestions (Nice to Have)
1. **Code Style**: Some functions are >50 lines. Consider splitting.
2. **Documentation**: `User` struct could benefit from a `Debug` implementation.

## Conclusion
Please address the critical security issue before merging. Otherwise, this looks good to merge! 👍
```

### 8.4 In-Depth Review

**When to Use:**
- Core architecture changes
- New major features
- Performance-critical code
- Security-sensitive code

**Process:**
```markdown
## In-Depth Review

### Architecture Assessment
I spent significant time reviewing the new query execution engine. Here are my thoughts:

#### Current Design
- Volcano model with batch processing
- Separation of physical and logical plans
- Vectorized execution

#### Strengths
- Clean separation of concerns
- Well-documented
- Good use of traits

#### Areas of Concern
1. **Memory Management**: The `Batch` struct clones data for each operator. Could we use references to reduce allocations?

2. **Error Propagation**: Some operators suppress errors and return empty results. Consider using `?` to surface errors.

3. **Parallelism**: The code is currently single-threaded. Could we leverage Rayon for parallel query execution?

### Detailed Code Review

#### Execution Engine (src/execution.rs)

**Line 45-67: Query Planning**
The planner is clean and well-structured. However, I have a suggestion:

```rust
// Current
fn plan_query(sql: &str) -> Result<PhysicalPlan> {
    let parsed = parse_sql(sql)?;
    let optimized = optimize_plan(parsed)?;
    Ok(optimized)
}

// Suggested: Add caching
fn plan_query(sql: &str, cache: &QueryCache) -> Result<PhysicalPlan> {
    if let Some(plan) = cache.get(sql) {
        return Ok(plan.clone());
    }
    
    let parsed = parse_sql(sql)?;
    let optimized = optimize_plan(&parsed)?;
    
    cache.insert(sql.to_string(), optimized.clone());
    Ok(optimized)
}
```

#### Vectorized Execution (src/operators.rs)

**Lines 123-189: Scan Operator**

Good use of SIMD-friendly patterns! The slice operations are efficient. One suggestion:

```rust
// Current
fn filter_batch(&self, batch: &[Value]) -> Vec<Value> {
    batch.iter().filter(|v| v.matches(&self.predicate)).cloned().collect()
}

// Suggested: Consider using iterator methods more extensively
fn filter_batch(&self, batch: &[Value]) -> Vec<Value> {
    batch.iter().filter_map(|v| v.matches(&self.predicate).then(|| v.cloned()))
              .collect()
}
```

### Performance Analysis

I ran benchmarks with the new vectorized execution:

```
Dataset: 100K rows
Query: SELECT AVG(price) FROM products WHERE category = 'Electronics'

Before (row-oriented):  234ms
After (vectorized):   45ms
Speedup: 5.2x 🚀
```

Excellent improvement! The vectorized execution is significantly faster.

### Security Review

I reviewed the authentication logic for potential vulnerabilities:

1. **Timing Attacks**: Password comparison uses `==` which is vulnerable to timing attacks. Consider using constant-time comparison.

2. **Input Validation**: The `username` field doesn't validate for length or special characters. Could lead to DoS attacks.

### Recommendations

1. **Memory**: Investigate using references in `Batch` struct
2. **Parallelism**: Consider adding `rayon` for parallel execution
3. **Security**: Address timing attacks in password comparison
4. **Testing**: Add performance regression tests

## Conclusion

This is a solid implementation of vectorized query execution. The architecture is well-thought-out. Once the memory concerns and security issues are addressed, I'd be happy to approve.

Requires changes: Critical
Overall: ✅ Ready to merge after fixes
```

### 8.5 Formal Review

**When to Use:**
- Major architectural changes
- Breaking changes
- Security reviews
- Release blockers

**Process:**
```markdown
## Formal Review: Release Candidate v1.0

This PR is being considered for the v1.0 release. All changes must be thoroughly reviewed.

## Review Checklist

### Functionality
- [ ] All features from PRD implemented
- [ ] Manual testing completed
- [ ] Integration tests passing
- [ ] Performance benchmarks meet targets

### Quality
- [ ] Code is clean and documented
- [ ] No known critical bugs
- [ ] Security review passed
- [ ] API changes are backward compatible

### Documentation
- [ ] README.md updated with new features
- [ ] API documentation complete
- [ ] Migration guide provided (if breaking changes)
- [ ] Release notes drafted

## Reviewers

@alice - Architecture and performance
@bob - Security and correctness
@charlie - Testing and documentation

## Timeline
- Code review deadline: Friday, 5pm
- Testing deadline: Sunday, 5pm
- Release target: Monday, 9am

## Decision
[ ] Approved for release
[ ] Needs more work
[ ] Reject
```

---

## 9. Communication Best Practices

### 9.1 Writing Constructive Feedback

**Tone and Style:**
- Be respectful and professional
- Focus on the code, not the person
- Use "I" statements to own your perspective
- Ask questions rather than making assumptions

**Good Examples:**
```markdown
I'm concerned about the memory usage here. Have you considered using references instead of cloning?

This looks like a potential race condition. Could you explain how this is thread-safe?

I'm not sure I understand the logic here. Could you add some inline comments?
```

**Bad Examples:**
```markdown
This is wrong. You shouldn't do it this way.

Why would you write code like this? It's obvious this won't work.
```

### 9.2 Handling Disagreements

**When You Disagree:**
1. **Provide reasoning**: Explain your perspective
2. **Be specific**: Point to exact code/lines
3. **Suggest alternatives**: Offer a different approach
4. **Ask questions**: "Have you considered...?"

**Example:**
```markdown
## Re: Using HashMap vs Vec

I'm not convinced that `HashMap` is the right choice here for this use case.

### Why Vec is Better
1. **Memory Overhead**: HashMap has overhead for key storage and hashing
2. **Cache Performance**: Vec has better cache locality
3. **Predictability**: Simple iteration is easier to optimize
4. **Small Datasets**: For N < 1000, Vec is likely faster

### When HashMap Might Be Better
- When you need O(1) lookups repeatedly
- When N is very large (>10000)
- When you need to check existence frequently

### Suggestion
Let's benchmark both approaches with your test data. If Vec is faster for our typical use cases, let's go with that.

### Alternative
What do you think about using `SmallVec<[T; N]>` for small fixed-size datasets?
```

**When You Agree:**
```markdown
## Re: Performance Optimization

Great idea! Your suggestion to use `HashSet` for duplicate detection is spot on. The O(n²) → O(n) improvement is exactly what we need.

### Additional Suggestion
Consider also adding `itertools::Itertools::unique()` which provides a ready-made implementation.

### My Approach
I've added a benchmark showing the improvement:

```rust
use std::time::Instant;
use std::collections::HashSet;

// Old approach: O(n²)
fn find_duplicates_old(data: &[i64]) -> Vec<i64> {
    let mut dupes = Vec::new();
    for i in data {
        for j in data {
            if i == j && !dupes.contains(&i) {
                dupes.push(i);
            }
        }
    }
    dupes
}

// New approach: O(n)
fn find_duplicates_new(data: &[i64]) -> Vec<i64> {
    let mut dupes = Vec::new();
    let seen: HashSet<i64> = HashSet::new();
    for item in data {
        if !seen.insert(item) {
            dupes.push(item);
        }
    }
    dupes
}

fn main() {
    let data: Vec<i64> = (0..10000).collect();
    
    let start = Instant::now();
    let _ = find_duplicates_old(&data);
    println!("Old: {:?}", start.elapsed());
    
    let start = Instant::now();
    let _ = find_duplicates_new(&data);
    println!("New: {:?}", start.elapsed());
}

// Output:
// Old: 2.3s
// New: 0.5ms
// Speedup: 4.6x! 🚀
```

Let me know if you want me to submit a follow-up PR with this change.
```

### 9.3 Using Emoji and Formatting

**Formatting for Readability:**
```markdown
## Performance ⚡

The nested loops are O(n²). Consider using HashSet.

## Security 🔒

There's a potential SQL injection vulnerability here. Use prepared statements instead.

## Documentation 📖

This function lacks examples showing how to handle edge cases.
```

**Best Practices:**
- Use emojis sparingly (only for emphasis)
- Use code blocks for examples
- Use bold/italics for emphasis
- Keep paragraphs short (2-3 sentences)
- Use lists for clarity

### 9.4 Asking Questions

**Types of Questions:**
1. **Clarification**: "I don't understand line 123. Can you explain?"
2. **Intent**: "What was your rationale for using HashMap here?"
3. **Alternatives**: "Have you considered using Vec instead?"
4. **Edge Cases**: "What happens if this receives an empty input?"
5. **Performance**: "Have you benchmarked this against other approaches?"

**Good Examples:**
```markdown
## Question

In `Table::add_column()`, you're using `Box<dyn Column>`. How does this affect performance compared to using generics `<C: Column>`? I'm concerned about the virtual function call overhead.

## Question

For the GROUP BY implementation, why did you choose `HashMap` over `BTreeMap`? For our typical dataset size (100K rows), the memory overhead of HashMap nodes might not be worth it compared to BTree's cache-friendly layout.

## Question

The `DataType::String` variant stores heap-allocated strings. Have you considered using an arena allocator or interning for better memory locality?
```

### 9.5 Providing Actionable Feedback

**Make Feedback Actionable:**
❌ "The code is messy."
✅ "The `process_query` function is 200 lines. Consider splitting into smaller functions like `parse_query()`, `build_plan()`, and `execute()`."

❌ "This doesn't work."
✅ "The `validate()` function doesn't handle empty input. It will panic. Consider returning `Option<T>` or `Result<T, E>`."

❌ "Add tests."
✅ "The new compression feature needs tests for:
- Empty strings
- Very long strings (>1MB)
- Unicode characters
- Already compressed strings"

---

## 10. Tools and Platforms

### 10.1 GitHub Features for Code Review

**Pull Request Template:**
- Go to repository Settings → Pull Requests
- Set up PR template
- Standardizes PR descriptions

**Example Template:**
```markdown
## Description
<!-- Describe your changes in detail -->

## Motivation and Context
<!-- Why is this change needed? What problem does it solve? -->

## Type of Change
- [ ] Bugfix
- [ ] Feature
- [ ] Breaking change
- [ ] Documentation
- [ ] Refactoring
- [ ] Performance Improvement
- [ ] Tests
- [ ] Code Style Update (formatting, linting)
- [ ] Other (please describe):

## Related Issue Number
<!-- If this PR closes an issue, reference it here -->

## Checklist
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing tests pass locally with my changes
```

**GitHub Features:**
- **Assignees**: Assign specific reviewers
- **Labels**: Add labels like `bug`, `enhancement`, `performance`, `documentation`
- **Milestones**: Link PRs to milestones
- **Projects**: Organize PRs into projects
- **Draft PRs**: Create PRs as drafts, request review when ready

### 10.2 Code Review Tools

**GitHub Built-in:**
- **Pull Request diff view**: See changes with side-by-side comparison
- **Rich diffs**: Better visualization of changes
- **Commit history**: See sequence of changes
- **Blame**: See who changed each line

**Browser Extensions:**
- **Octotree**: Visualizes git history as tree
- **GitLens**: Adds useful git information to GitHub
- **Refined GitHub**: Improves GitHub UI

**Local Tools:**
- **gitg**: TUI for git log browsing
- **tig**: Terminal UI for git log
- **lazygit**: Terminal-based git wrapper

### 10.3 IDE Features

**VS Code:**
- **Pull Requests**: GitHub PRs in VS Code
- **Pull Request Creator**: Create PRs from within VS Code
- **GitHub Pull Requests**: Alternative PR extension
- **GitLens**: See file blame, history, etc.

**IntelliJ IDEA:**
- **GitHub Integration**: Built-in GitHub PR support
- **Code Review Plugin**: View and discuss PRs in IDE

### 10.4 Third-Party Services

**Codecov:** Code coverage reports
**Coveralls:** Code coverage with PR comments
**Danger Zone:** Blocks PRs with issues
**Dependabot:** Checks for outdated dependencies

---

## 11. Advanced Workflows

### 11.1 Rebase vs Merge

**Rebase:**
```bash
# Replay your commits on top of main
git checkout feature/new-feature
git fetch upstream
git rebase upstream/main

# History becomes:
# A-B-C-D-E-F (linear)
#     ↑
# main (updated)
```
**Pros:**
- Clean, linear history
- Easier to understand timeline
- Avoids merge commits

**Cons:**
- Rewrites history (changes commit hashes)
- Can cause conflicts if not careful
- Loses context of merge commits

**Merge:**
```bash
# Combine histories
git checkout feature/new-feature
git fetch upstream
git merge upstream/main

# History becomes:
# A-B-C-D-E-F (linear)
#     ↑         ↘
# main (updated)     M (merge commit)
```
**Pros:**
- Preserves original commit history
- Shows merge point explicitly
- Safer (rewrites less)

**Cons:**
- Creates merge commits
- History can become messy
- Harder to read timeline

**Recommendation:** Use `rebase` for personal branches, use `merge` for feature branches that might be merged soon.

### 11.2 Handling Conflicts

**What is a Conflict?**
When the same line has been changed in both branches and Git can't automatically merge them.

**Example Scenario:**
```bash
# Main branch
// src/column.rs:42: let data = Vec::new();

# Your feature branch
// src/column.rs:42: let data = Vec::with_capacity(100);

# When you try to merge/rebase
# Git shows conflict at line 42
```

**Resolving Conflicts:**
```bash
# 1. Rebase with main
git rebase upstream/main

# Git shows conflicts
Auto-merging src/column.rs
CONFLICT (content): Merge conflict in src/column.rs
Auto-merging src/table.rs
CONFLICT (content): Merge conflict in src/table.rs

# 2. Open files
git status
# Shows:
# both modified:   src/column.rs
# both modified:   src/table.rs

# 3. Edit files to resolve conflicts
# Look for conflict markers:
<<<<<<< HEAD
let data = Vec::with_capacity(100);
=======
let data = Vec::new();
>>>>>>> upstream/main

# Keep what you want, remove the rest

# 4. Mark conflicts as resolved
git add src/column.rs
git add src/table.rs

# 5. Continue rebase
git rebase --continue

# Repeat until no more conflicts
```

**Best Practices for Conflicts:**
1. **Communicate**: Tell reviewers you're working on conflicts
2. **Ask for help**: Don't be afraid to ask in PR comments
3. **Work incrementally**: Resolve one file at a time
4. **Test after**: Always run tests after resolving conflicts

### 11.3 Squashing Commits

**Why Squash?**
- Clean history for main branch
- Keep logical changes together
- Reduce noise in git log

**Interactive Rebase:**
```bash
# Rebase with -i flag for interactive
git rebase -i upstream/main

# For each commit, Git asks what to do:
# pick, reword, edit, squash, fixup, drop, etc.
```

**Squashing Multiple Commits:**
```bash
# 1. Interactive rebase
git rebase -i upstream/main

# 2. Mark multiple commits as "squash"
pick 45123a Add Column trait
pick 789b12 Implement IntColumn
squash 903c45 Implement FloatColumn
squash 123e78 Implement StringColumn

# 3. Continue
# Git will combine these into one commit
# And ask for commit message
```

**Automated Squash:**
```bash
# Soft reset to commit before first one
git reset --soft HEAD~3

# Commit all changes as one
git commit -m "Implement all column types (Int64, Float64, String)"
```

### 11.4 Cherry-Picking

**What is Cherry-Picking?**
Applying specific commits from one branch to another.

**Use Cases:**
- Backporting fixes to release branches
- Applying specific changes without merging full branch
- Resolving issues by cherry-picking fixes

**Process:**
```bash
# 1. Find the commit hash
git log upstream/main

# Output:
# abc1234 Add column compression

# 2. Cherry-pick to your branch
git checkout feature/my-work
git cherry-pick abc1234

# 3. Resolve conflicts if any
# Edit conflicting files
git add .
git cherry-pick --continue
```

**Interactive Cherry-Pick:**
```bash
# Cherry-pick with edits
git cherry-pick -e abc1234
```

### 11.5 Bisecting for Bugs

**What is Git Bisect?**
Binary search through git history to find which commit introduced a bug.

**When to Use:**
- Introduced a bug in recent commits
- Don't know which commit caused it
- Regression occurred but unclear when

**Process:**
```bash
# 1. Start bisect
git bisect start

# 2. Mark current commit as bad (bug exists)
git bisect bad

# 3. Mark a known good commit as good
git bisect good HEAD~10  # known working version

# 4. Git will checkout middle commit and ask for testing
# You test, then mark as good or bad
# Repeat until bug is found

# Example interaction:
# Bisecting: abc1234..def5678 (10 commits left)
# 123e789 is the first bad commit
# Test this commit (good/bad)?

# After testing, you respond:
good

# Bisect continues...
# abc1234 is the first bad commit
# abc1234 introduced the bug

# 5. Show summary
git bisect visualize
```

### 11.6 Code Review with Multiple Maintainers

**When Multiple Reviewers:**
```markdown
## Reviewers
@alice - Focus on architecture and performance
@bob - Focus on correctness and edge cases
@charlie - Focus on documentation and testing
```

**Division of Labor:**
- **Primary Reviewer**: Does main review
- **Secondary Reviewers**: Review specific areas
- **Approver**: Has final say

**Code Review Process:**
```markdown
## My Review

I've reviewed the code and have a few suggestions. @alice and @charlie, could you please review:

### @alice
Please review the query planning logic. I'm concerned about the performance of the join operation.

### @charlie
Please review the test coverage for the new compression feature. I want to ensure all edge cases are covered.

## My Findings

I found a potential issue in the `Column::get()` method at line 234. The bounds checking seems inconsistent with other methods.

## Approval
I'll leave the final approval to @maintainer after @alice and @charlie have reviewed.
```

### 11.7 Maintaining Multiple Maintainer Approvals

**Configuring Protected Branches:**
```yaml
# In GitHub repository settings → Branches → main → Edit
Require status checks to pass before merging:
- [x] CI/tests
- [x] Code coverage
- [ ] Deployment (if applicable)

Require branches to be up to date before merging:
- [x]

Require conversation resolution before merging:
- [x]

Required reviewers:
- @alice (Architecture)
- @bob (Security)
```

**GitHub App Combinations:**
```bash
# Example: Require approval from specific team
github.com/org/repo/settings/branches/main
→ Require pull request before merging
→ Require approvals: @team-leads
→ Number of required approvals: 2
```

---

## 12. Troubleshooting

### 12.1 Common Code Review Issues

**Issue: PR Not Being Reviewed**
**Symptoms:**
- PR is open for days with no comments
- No assignees
- Stale status

**Solutions:**
1. **Tag maintainers**: `@maintainer please review`
2. **Ping team**: `@team/ios-core we need review on this`
3. **Check requirements**: Ensure PR has all required information
4. **Update title**: Make title more descriptive to attract reviewers
5. **Add tests**: More tests = more confidence

**Example Comment:**
```markdown
@maintainer Could you please review when you have a chance? This PR has been open for a week without any activity. Happy to make changes if needed!
```

**Issue: Conflicting Reviews**
**Symptoms:**
- Reviewer A says "Use HashMap"
- Reviewer B says "Use Vec"
- Author is confused about which to follow

**Solutions:**
```markdown
## Conflicting Feedback

@reviewer-a suggested using HashMap for O(1) lookups, but @reviewer-b prefers Vec for cache locality. Could we discuss the trade-offs here?

My thoughts:
- HashMap: Faster for large N with many lookups
- Vec: Better cache locality, no allocation overhead
- Our typical dataset: ~100K rows, ~10K unique values

Benchmarking would help us make a data-driven decision. What do you think?

@reviewer-a @reviewer-b Could you both weigh in on which approach makes sense for our use case?
```

**Issue: Author Unresponsive**
**Symptoms:**
- Reviewer provides feedback
- No response for days
- PR is blocked

**Solutions:**
1. **Private message**: Reach out privately first
2. **Offer help**: "I'm happy to help make these changes if you're busy"
3. **Check if PR is stale**: Maybe author moved on
4. **Escalate**: If no response for 1-2 weeks, comment "This PR is being closed due to inactivity"

**Example Comment:**
```markdown
## Status Check

Hi @author, following up on the feedback from @reviewer. Are you still working on this? 

We're planning to close this PR in a week if there's no activity. Let me know if you need more time or if you'd like someone else to take over.

Thanks!
```

### 12.2 CI Failures

**Understanding CI Failures:**
- CI checks are automated
- Red ❌ = Block merge
- Yellow ⚠️ = Warning but allowed
- Green ✅ = Passed

**Common CI Failures and Fixes:**

**1. Build Failure**
```bash
# Error
error[E0382]: use of moved value: `value`
   --> src/column.rs:123:9

# Fix: Don't use moved value after moving it
let value = column.as_vec().clone();  // ❌ Bad
let value = column.as_vec();          // ✅ Good
```

**2. Test Failure**
```bash
# Error
thread 'tests::test_column_slice' panicked at 'tests/column_tests.rs:45:9:
assertion failed: `left == right'

# Fix: Debug the failing test
// In test file
#[test]
fn test_column_slice() {
    let col = create_test_column();
    let slice = col.slice(Some(1..3));
    assert_eq!(slice.len(), 2);
    
    // Add debug output
    eprintln!("Slice: {:?}", slice);
}
```

**3. Formatting Failure**
```bash
# Error
Left: 
+    let data = Vec::new();
Right:
-    let data = Vec::new();

# Fix: Run cargo fmt
cargo fmt

# Then commit
git add .
git commit -m "Fix formatting"
```

**4. Clippy Warnings**
```bash
# Warning
warning: this expression borrows a local value here...
   --> src/column.rs:67:9

# Fix: Follow clippy's suggestion
// Before
for item in data {
    println!("{}", item);
}

// After
for item in &data {
    println!("{}", item);
}
```

### 12.3 Git Issues During PR Process

**Issue: Push Rejected**
```bash
# Error
error: failed to push some refs to 'origin'
hint: Updates were rejected because the tip of your current branch is behind its remote counterpart

# Fix: Pull before pushing
git pull origin feature/new-tables
git push origin feature/new-tables
```

**Issue: Merge Conflict**
```bash
# Error
Auto-merging src/column.rs
CONFLICT (content): Merge conflict in src/column.rs

# Fix: Resolve conflicts
# See Section 11.2: Handling Conflicts
```

**Issue: Detached HEAD**
```bash
# Error
fatal: You are not currently on a branch.

# Fix: Checkout a branch
git checkout main
# or
git checkout -b temp-branch
```

### 12.4 PR State Machine

**PR States:**
```
Open → In Review → Approved → Merged → Closed
  ↓         ↓         ↓         ↓
Rejected   Requested          Closed
```

**State Descriptions:**
- **Open**: PR is open, awaiting review
- **In Review**: Under active review, changes requested
- **Approved**: All requirements met, ready to merge
- **Rejected**: Changes not accepted, PR closed
- **Merged**: Changes integrated into main
- **Closed**: PR closed without merging

**Getting Stuck:**
```markdown
## Stuck in "In Review" State

This PR has been in review for 2 weeks. The requested changes are substantial, but we haven't received feedback.

### Options
1. **Request status update**: Ask reviewers if they're still planning to review
2. **Split the PR**: Break into smaller, reviewable chunks
3. **Find alternative reviewers**: Ask other team members to review
4. **Consider merging without review**: For low-risk, trivial changes
5. **Close the PR**: If no longer relevant

@maintainer Could you please help resolve this stale PR?
```

---

## 13. Code Review Checklist

### 13.1 Pre-Submission Checklist

**For Authors:**
```markdown
## Before Submitting PR

### Code Quality
- [ ] All tests pass locally (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy --all-targets`)
- [ ] Builds in release mode (`cargo check --release`)
- [ ] No `TODO` or `FIXME` in production code
- [ ] No debug prints in production code

### Testing
- [ ] New functionality has unit tests
- [ ] Edge cases are covered
- [ ] Integration tests added/updated
- [ ] Manual testing completed
- [ ] Performance benchmarks (if applicable)

### Documentation
- [ ] Public API has documentation
- [ ] README.md updated (if user-facing)
- [ ] Complex logic has inline comments
- [ ] PR description clearly explains changes
- [ ] Breaking changes documented

### Cleanliness
- [ ] No unnecessary files committed
- [ ] `target/` directory not in commit
- [ ] No merge conflicts (git rebase done)
- [ ] Commit messages are descriptive
- [ ] Branch is up-to-date with main

### Ready
- [ ] Reviewed own code
- [ ] Tested on another machine (optional but recommended)
- [ ] All checklist items complete
```

### 13.2 Reviewer Checklist

**For Reviewers:**
```markdown
## Code Review Checklist

### Understanding
- [ ] Read and understood the PR description
- [ ] Cloned branch and tested locally
- [ ] Reviewed all changed files
- [ ] Understand the context and motivation

### Code Quality
- [ ] Code is correct and bug-free
- [ ] Code is efficient (no obvious performance issues)
- [ ] Code is readable and follows project conventions
- [ ] No security vulnerabilities
- [ ] No memory leaks or unsafe issues

### Testing
- [ ] Tests are comprehensive
- [ ] Tests cover edge cases
- [ ] Tests are meaningful (not just for coverage)
- [ ] Integration tests pass

### Documentation
- [ ] Public API is documented
- [ ] Complex logic is explained
- [ ] Examples are provided (if applicable)
- [ ] Breaking changes are noted

### Final Decision
- [ ] Changes are ready to merge
- [ ] Changes require revision
- [ ] Changes should be rejected

### Feedback
- [ ] Provided constructive, specific feedback
- [ ] Explained reasoning for suggestions
- [ ] Offered alternatives where appropriate
```

### 13.3 Merge Checklist

**For Maintainers:**
```markdown
## Merge Checklist

### Requirements Met
- [ ] All requested changes included
- [ ] No unintended changes included
- [ ] CI is green (all checks passing)
- [ ] Required approvals received
- [ ] Code review feedback addressed

### Testing
- [ ] Manual testing completed
- [ ] Release build successful
- [ ] Performance benchmarks meet targets
- [ ] No regressions detected

### Documentation
- [ ] README.md updated
- [ ] CHANGELOG.md updated
- [ ] Breaking changes documented
- [ ] Migration guide provided (if needed)

### Safety
- [ ] No known critical bugs
- [ ] Security review passed
- [ ] Database migrations validated (if applicable)

### Merge Strategy
- [ ] Squashed commits (if needed)
- [ ] Clean merge history
- [ ] Merge commit message is clear

### Post-Merge
- [ ] PR branch deleted
- [ ] Deployed to staging/production
- [ ] Release notes published
- [ ] Team notified
```

---

## 14. Best Practices Summary

### 14.1 For Contributors

**Do:**
✅ Start from updated `main` branch
✅ Use feature branches
✅ Write descriptive commit messages
✅ Run tests before committing
✅ Keep PRs small and focused
✅ Respond to review feedback promptly
✅ Ask questions when unclear
✅ Self-review your code before submitting
✅ Test on multiple platforms/environments
✅ Update documentation

**Don't:**
❌ Commit directly to `main`
❌ Push without testing
❌ Ignore CI failures
❌ Use vague commit messages
❌ Leave PRs stale without communication
❌ Be defensive when receiving feedback
❌ Merge without review (even if you have access)
❌ Submit huge PRs (hard to review)
❌ Include unrelated changes in PR

### 14.2 For Reviewers

**Do:**
✅ Be specific and constructive in feedback
✅ Explain the "why" behind suggestions
✅ Provide code examples
✅ Review PRs in a timely manner
✅ Ask questions to understand intent
✅ Focus on most important issues first
✅ Be patient with contributors
✅ Test code locally before commenting
✅ Check for similar issues in codebase
✅ Acknowledge good code

**Don't:**
❌ Be harsh or critical
❌ Make personal attacks
❌ Be vague ("this needs work")
❌ Ignore PRs from new contributors
❌ Gatekeep unnecessarily
❌ Review when tired or rushed
❌ Make the author feel bad

### 14.3 For Maintainers

**Do:**
✅ Set clear contribution guidelines
✅ Configure CI/CD appropriately
✅ Respond to PRs quickly
✅ Make yourself available for questions
✅ Delegate reviews to trusted team members
✅ Protect `main` branch
✅ Set up branch protection rules
✅ Keep merge history clean
✅ Document all breaking changes
✅ Provide timely feedback

**Don't:**
❌ Be unavailable for long periods
❌ Accept low-quality PRs
❌ Merge without proper review
❌ Ignore community feedback
❌ Keep secrets in repository
❌ Make changes without PR
❌ Ignore security issues
❌ Release without testing

---

## 15. Examples

### 15.1 Example: Good Pull Request

**Title:** `feat: Add column compression using dictionary encoding`

**Description:**
```markdown
## Changes
This PR adds dictionary encoding to StringColumn to reduce memory usage for datasets with repeated string values.

## Motivation
StringColumn currently stores all strings individually in a `Vec<String>`, leading to:
- High memory usage (each String is separately heap-allocated)
- Poor cache locality (random memory access patterns)
- Compression ratio of 1:1 for most datasets

This PR introduces a dictionary-based compression similar to how ClickHouse handles string data, achieving:
- 60-80% memory reduction for typical workloads
- Minimal performance overhead (<10% read, <15% write)

## Implementation Details

### Changes
1. **New `DictionaryColumn` struct** with `HashMap<String, u32>` for encoding
2. **Modified `StringColumn`** to use dictionary for values with length >10 chars
3. **Added compression test suite** in `tests/compression_test.rs`
4. **Benchmark suite** in `benches/compression.rs`

### Key Design Decisions
- **Compression threshold**: Strings >10 chars (configurable)
- **Dictionary per column**: Each column has its own dictionary (vs global)
- **Backward compatible**: No API changes, full compatibility with existing code
- **Opt-out opt-in**: Compression can be disabled if needed

## Performance Impact

### Memory Reduction
Benchmarks with synthetic data (100K rows, 100 unique values):

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Memory usage | 156MB | 45MB | 71% reduction |
| Read speed | 12ms | 14ms | +16% overhead |
| Write speed | 25ms | 28ms | +12% overhead |

For real-world datasets with more repetition, memory reduction reaches 80%+.

## Testing

### Unit Tests
- `DictionaryColumn`: New struct tests
- `StringColumn`: Modified function tests
- Edge cases: Empty strings, unicode, very long strings, compression/decompression

### Integration Tests
- `compression_test`: Full integration test suite
- Existing tests: All passing, no regressions

## Breaking Changes
None - fully backward compatible.

## Checklist
- [x] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [x] Self-review completed
- [x] Added comprehensive unit tests
- [x] Added integration tests
- [x] Performance benchmarks added
- [x] Documentation updated (inline comments)
- [x] No `TODO` or `FIXME` in final code
- [x] All tests pass (`cargo test`)
- [ ] CHANGELOG.md updated (not breaking change)
- [ ] README.md updated (internal change)
```

### 15.2 Example: Good Code Review

**Reviewer Comment:**
```markdown
## Code Review

Overall, this is excellent work! The compression algorithm is well-thought-out and the test coverage is comprehensive.

### Performance Analysis

I ran the benchmarks with your synthetic data:

```
Dataset: 100K rows, 100 unique strings
Before: 156MB
After: 45MB
Reduction: 71%
Overhead: <15% read, <12% write
```

These are impressive results! The 71% memory reduction for such minimal overhead makes this a great addition.

### Code Quality

**Strengths:**
- ✅ Clean, readable code
- ✅ Excellent documentation
- ✅ Comprehensive test coverage
- ✅ Benchmarks included
- ✅ Thread-safe implementation (using RwLock for dictionary)

**Minor Suggestions:**

**1. Dictionary Growth**
In `DictionaryColumn::add_string()`, the HashMap can grow unbounded in worst case. Consider:
```rust
// Current
fn add_string(&mut self, value: String) -> u32 {
    let id = self.dictionary.len() as u32;
    self.dictionary.insert(value, id);
    id
}

// Suggested
fn add_string(&mut self, value: String) -> Result<u32> {
    if self.dictionary.len() >= u32::MAX {
        return Err("Dictionary full".into());
    }
    
    let id = self.dictionary.len() as u32;
    self.dictionary.insert(value, id);
    Ok(id)
}
```

**2. Compression Threshold**
You hardcoded 10 characters as the threshold. Consider making it configurable:
```rust
// In StringColumn
pub struct StringColumn {
    data: Vec<String>,
    dictionary: DictionaryColumn,
    compression_threshold: usize,  // Configurable
}

// Default constructor
impl StringColumn {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            dictionary: DictionaryColumn::new(),
            compression_threshold: 10,  // Default threshold
        }
    }
    
    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            data: Vec::new(),
            dictionary: DictionaryColumn::new(),
            compression_threshold: threshold,
        }
    }
}
```

**3. Test Edge Case: Empty Dictionary**
Your tests don't cover the case where the dictionary is empty (first row insertion). Consider adding:
```rust
#[test]
fn test_empty_dictionary() {
    let mut col = StringColumn::new();
    
    // First insert should work
    col.add_string("test".to_string()).unwrap();
    
    // Verify it was added
    let ids = col.get_all_ids();
    assert_eq!(ids, vec![0]);
}
```

### Security Review

I reviewed the compression/decompression logic and found no security vulnerabilities. The code correctly handles:
- String bounds checking
- Dictionary key validation
- Thread safety (RwLock usage)

### Documentation

Excellent inline comments! The explanation of the dictionary encoding algorithm in `DictionaryColumn::encode()` is clear and well-written.

## Conclusion

This is a high-quality PR that provides significant memory benefits with minimal overhead. The code is clean, well-tested, and well-documented.

**Requested Changes:**
- [x] Add configurable compression threshold
- [x] Add dictionary size limit
- [ ] Add test for empty dictionary case

Once these minor suggestions are addressed, this is ready to merge! 

Great work! 🚀
```

### 15.3 Example: Handling Issues in PR

**Reviewer Comment:**
```markdown
## Issue Found: Potential Memory Leak

In `Column::get()` at line 234:

```rust
pub fn get(&self, index: usize) -> Result<Value> {
    // Create a vector with capacity for potential decompression
    let mut result = Vec::with_capacity(10);
    
    // ... decompression logic ...
    
    Ok(result.into_iter().next().unwrap())
}
```

**Problem:** If `into_iter()` is never consumed (due to early return or panic), the allocated vector is leaked.

**Suggested Fix:**
```rust
pub fn get(&self, index: usize) -> Result<Value> {
    // Better: Return Result without intermediate allocation
    // Check if compressed
    if let Some(compressed_id) = self.compressed_data.get(&index) {
        return Ok(self.decompress(compressed_id)?);
    }
    
    // Fallback: Return value directly
    self.data.get(index)
        .cloned()
        .map(|v| Value::String(v))
        .ok_or_else(|| DatabaseError::column_error(format!("Index {} out of bounds", index)))
}
```

This avoids the potential allocation and is more efficient for the common case (no compression).

**Another Example:**
If the allocation is truly needed (for compression result), ensure the result is always consumed:

```rust
pub fn get(&self, index: usize) -> Result<Value> {
    let result = self._get_internal(index)?;  // May allocate
    
    // Always return the result
    Ok(result)
}
```

**Impact:** This is a minor optimization but good to fix for correctness and efficiency.
```

**Author's Response:**
```markdown
## Fix Applied

Great catch! I've updated the implementation as suggested:

```rust
pub fn get(&self, index: usize) -> Result<Value> {
    // Check if value is compressed
    if let Some(compressed_id) = self.compressed_data.get(&index) {
        return Ok(self.decompress(compressed_id)?);
    }
    
    // Return uncompressed value directly
    self.data.get(index)
        .cloned()
        .map(|v| Value::String(v))
        .ok_or_else(|| DatabaseError::column_error(format!("Index {} out of bounds", index)))
}
```

This simplifies the logic and avoids the potential memory leak. Thanks for catching that!

I also added the empty dictionary test case you mentioned:
```rust
#[test]
fn test_empty_dictionary() {
    let mut col = StringColumn::new();
    let id = col.add_string("first".to_string()).unwrap();
    assert_eq!(id, 0);
}
```

All tests passing locally. Ready for re-review!
```

---

## 📋 Quick Reference

### Creating a PR
```bash
git checkout -b feature/new-feature
git add .
git commit -m "Add new feature"
git push origin feature/new-feature
# Then create PR on GitHub
```

### Updating a PR
```bash
# Make changes
git add .
git commit -m "Fix issue found in review"
git push origin feature/new-feature
# PR automatically updates
```

### Syncing with Main
```bash
git fetch upstream
git checkout main
git pull upstream main
git checkout -b feature/new-work
```

### Resolving Conflicts
```bash
git fetch upstream
git rebase upstream/main
# Edit conflicting files
git add .
git rebase --continue
```

---

## 🎓 Conclusion

### Key Takeaways

1. **Automate Early**: Use CI hooks to catch issues before they reach PR
2. **Communicate Clearly**: Be specific in feedback and responses
3. **Test Thoroughly**: Cover edge cases and real-world scenarios
4. **Keep PRs Small**: Focused changes are easier to review
5. **Respond Promptly**: Don't leave reviewers waiting
6. **Ask Questions**: Clarify instead of assuming
7. **Be Constructive**: Focus on code quality, not criticism
8. **Document Everything**: Good code is documented code
9. **Test Locally**: CI is a safety net, not a replacement
10. **Learn from Each PR**: Every review is a learning opportunity

### Getting Started

1. **Read this guide** - Understand the workflow
2. **Clone the repository** - Get the code locally
3. **Create a feature branch** - `git checkout -b feature/my-change`
4. **Make your changes** - Follow best practices
5. **Test thoroughly** - Run `cargo test`
6. **Commit and push** - `git push origin feature/my-change`
7. **Create PR** - Describe your changes clearly
8. **Wait for review** - Be patient and responsive

### Continuous Improvement

Every PR is an opportunity to:
- Learn from experienced developers
- Improve your code quality
- Understand the codebase better
- Contribute to project culture
- Build your reputation

---

**Happy Code Reviewing!** Every review makes the codebase better and helps everyone grow as developers. 🚀