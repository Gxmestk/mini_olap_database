<tool_call>edit_file<arg_key>path</arg_key><arg_value>mini_rust_olap/docs/code-review-assessment.md</arg_value><arg_key>mode</arg_key><arg_value>overwrite</arg_value><arg_key>display_description</arg_key><arg_value>Add code review assessment section with multiple choice questions</arg_value></tool_call>

---

## 16. Code Review Assessment

Test your understanding of code review workflows and best practices with these multiple-choice questions.

### Q1. How do you trigger a code review when making a pull request to main?

A. Submit code directly to main branch
B. Create a feature branch, commit changes, then push to your fork
C. Create a feature branch, push to your fork, then open a Pull Request on GitHub
D. Create a Pull Request directly from main branch

**Answer:** **C** - Creating a feature branch, pushing to your fork, then opening a Pull Request on GitHub triggers the code review workflow.

**Explanation:** The Pull Request itself is what triggers the code review. When you open a PR on GitHub, it notifies reviewers and sets the PR state to "Needs Review". Your local CI hooks ensure code quality before the PR is even created, making reviewers' job easier.

---

### Q2. What is the first thing that happens after you push to your fork and create a Pull Request?

A. CI/CD pipeline runs automatically
B. Maintainers get an email notification
C. Reviewers see the PR in the Pull Requests tab
D. Your local git hooks run
E. The PR is immediately merged

**Answer:** **C** - Reviewers see the PR in the Pull Requests tab and get notified.

**Explanation:** After you create a PR, GitHub notifies watchers/reviewers. Your local hooks have already run during `git push`, so code quality is ensured. The PR appears in the repository's Pull Requests list, and reviewers can click on it to start reviewing.

---

### Q3. What is the recommended Git workflow for making changes to main?

A. git checkout main → make changes → git commit → git push
B. git checkout -b feature → make changes → git commit → git push → create PR → merge
C. git checkout main → git pull → git checkout -b feature → make changes → git commit → git push → create PR
D. Directly commit to main branch (requires bypassing hooks)

**Answer:** **B** - Using feature branches is the recommended workflow.

**Explanation:** Direct commits to main bypass code review and pollute git history. Feature branches isolate changes, enable easier testing, and provide a clean history when merged. The workflow is: `main → pull → checkout feature → work → push → PR → merge`.

---

### Q4. Which command pushes your local changes to the remote repository?

A. git push
B. git commit
C. git merge
D. git fetch

**Answer:** **A** - `git push` uploads your local commits to the remote repository.

**Explanation:** `git push` is the command that transfers your local commits to the remote repository (like GitHub). You must have a remote configured (typically `origin`) and push access rights. This command triggers any configured pre-push hooks.

---

### Q5. What should you do BEFORE creating a Pull Request?

A. Nothing, just create the PR
B. Push your changes to remote
C. Run all tests locally
D. Update documentation

**Answer:** **D** - Push your changes to remote and ensure all tests pass locally.

**Explanation:** Before creating a PR, you should push your feature branch to your remote repository and ensure all tests pass. Your local pre-commit hooks already checked formatting and linting, but pushing and running full test suite confirms everything works in a clean environment. This prevents creating PRs with broken or untested code.

---

### Q6. What happens if a reviewer requests changes on your Pull Request?

A. PR is closed
B. Changes are automatically merged
C. You must push new commits to your branch
D. PR status changes to "Needs Work"

**Answer:** **C** - You must push new commits to your branch and the PR status changes to "Needs Work".

**Explanation:** When a reviewer requests changes, they don't modify your code directly. They add review comments explaining what needs to be fixed. You then make those changes, commit them to your branch, and push. GitHub updates the PR with your new commits, and reviewers see the changes in the updated diff.

---

### Q7. What is the purpose of a "protected branch" in GitHub?

A. To prevent accidental deletions
B. To enforce code review before merging
C. To block direct commits to main
D. All of the above

**Answer:** **D** - All of the above: prevent accidental deletions, enforce code review, block direct commits.

**Explanation:** Protected branches enforce rules like "Require pull request before merging" and "Require status checks to pass". This prevents anyone from pushing directly to main, ensures CI runs successfully, and typically requires reviewer approval. It's a gatekeeper for main branch.

---

### Q8. How do you update your feature branch with the latest changes from main?

A. git checkout main && git merge main
B. git fetch upstream && git rebase upstream/main
C. git pull origin main
D. git merge upstream/main

**Answer:** **B** - `git fetch upstream && git rebase upstream/main`

**Explanation:** `git fetch` gets the latest changes from upstream (the official repository). `git rebase` replays your commits on top of the latest main, resulting in a cleaner linear history compared to merge. This helps avoid merge conflicts and keeps your branch up-to-date with the project.

---

### Q9. What should you do if your pre-commit hook fails?

A. Commit with --no-verify flag
B. Force commit with --force flag
C. Fix the issues reported by the hook
D. Disable the hook temporarily

**Answer:** **C** - Fix the issues reported by the hook.

**Explanation:** The hook is there to help you, not to block you. If formatting check fails, run `cargo fmt` and commit the formatted code. If tests fail, fix the failing tests. If clippy has warnings, address them. Always fix the underlying issues rather than bypassing the checks.

---

### Q10. What is the purpose of adding reviewers to a Pull Request?

A. To automatically approve the PR
B. To get credit for the review
C. To ensure people see the PR
D. To ask for specific expertise

**Answer:** **D** - To ask for specific expertise.

**Explanation:** Adding reviewers notifies them that their input is needed and assigns them to review specific aspects of your PR. You might add a database expert for schema changes, or a performance expert for optimization PRs. This ensures the right people are looking at your code.

---

### Q11. How many reviewers should you typically assign to a PR?

A. 1-2 reviewers for small changes
B. 3-5 reviewers for medium changes
C. 6+ reviewers for large or critical changes
D. As many as possible to catch all issues

**Answer:** **B** - 3-5 reviewers for medium changes.

**Explanation:** For most changes, 3-5 reviewers is a good balance. Too few reviewers might miss issues, while too many can be overwhelming and slow down the process. Assign reviewers based on the scope and complexity of changes - more reviewers for architectural or security-sensitive changes.

---

### Q12. What is a "squash merge" in Git?

A. Combining all commits into one
B. Deleting commit messages
C. Rewriting commit history
D. Applying all commits from branch as one

**Answer:** **A** - Combining all commits from the branch into a single commit.

**Explanation:** A squash merge takes all the commits from your feature branch and combines them into a single commit when merging to main. This keeps the main branch history clean and focused. Instead of seeing 10 "WIP" commits, main gets one well-described commit like "Add column compression feature".

---

### Q13. What does `git rebase` do?

A. Rewrites commit history
B. Moves commits to a different base
C. Combines multiple commits
D. Creates merge commits

**Answer:** **B** - Moves commits to a different base.

**Explanation:** `git rebase` detaches your commits from their current base (like your feature branch starting point) and re-applies them on top of a new base commit (like updated main). This creates a linear, cleaner history compared to merge, which creates merge commits.

---

### Q14. What is the difference between `git merge` and `git rebase`?

A. merge creates a merge commit, rebase moves commits
B. merge preserves history, rebase rewrites history
C. merge is safer, rebase is cleaner
D. merge is for integrating branches, rebase is for updating branches

**Answer:** **A** - merge creates a merge commit, rebase moves commits.

**Explanation:** The key difference is that `merge` combines histories by creating a new merge commit, while `rebase` moves (replays) commits on top of a different base. Merge is safer and preserves original commit hashes, while rebase creates a cleaner linear history but rewrites commit hashes.

---

### Q15. What is the first thing a maintainer should do before merging a PR?

A. Check if all reviewers approved
B. Verify CI is green
C. Review the code diff
D. Merge immediately without review

**Answer:** **C** - Verify CI is green.

**Explanation:** Before merging, a maintainer should ensure all automated checks pass. The green checkmark on the PR indicates CI has run successfully (tests passed, builds succeeded, formatting is correct, linting is clean). This provides confidence that the code is ready for production.

---

## Scoring Guide

### Calculate Your Score

```
Git Workflow:        _____ / 5
PR Creation:         _____ / 5
Review Process:       _____ / 5
Best Practices:       _____ / 5
------------------------------------------------
TOTAL SCORE:          _____ / 20
```

### Interpret Your Score

| Score | Level | What It Means |
|-------|--------|----------------|
| **18-20** | Expert 🎉 | Ready to conduct code reviews professionally |
| **15-17** | Advanced 👍 | Good understanding of code review workflows |
| **10-14** | Intermediate 👌 | Familiar with basic concepts, some gaps |
| **Below 10** | Beginner 📚 | Needs to review code review fundamentals |

### Recommended Next Steps

1. **Read the Guide**: Study `docs/code-review-workflow.md` for detailed explanations
2. **Practice Locally**: Try creating a feature branch and PR in a test repository
3. **Review Others' PRs**: Look at real open source projects for examples
4. **Learn Tools**: Experiment with git commands, GitHub features
5. **Retake Assessment**: After studying, try the assessment again

