# Dit

**Dit** is a minimal version control system inspired by Git.  
It supports staging, committing, resetting to previous commits, branching, viewing history, etc.



---

## Example workflow

```bash
# 1. Initialise
dit init

# 2. Stage files
dit add src/main.rs Cargo.toml

# 3. Commit
dit commit -m "Initial commit" -a "ComplexAirport <complexaiport@example.com>"

# 4. Create and switch to a feature branch
dit branch new feature_login
dit branch switch feature_login

# ... work on th files
# 5. Stage, commit...
dit add src/auth.rs
dit commit -m "Add basic features" -a "ComplexAirport <complexaiport@example.com>"

# 6. Review history
dit history --count 10
```

---

## Commands

### `dit init`

Create a `.dit/` directory in the current working directory (if it didn't already exist) and set up default branch **main**.

---

### `dit history [-c|--count <N>]`

Print the latest `N` commits (default **5**) in reverse chronological order.

---

### `dit status`

Show:

* Untracked files
* Tracked but modified files
* Staged files awaiting commit

---

### `dit add <FILES…>`

Stage one or more paths (files or directories) for the next commit.

### `dit unstage <FILES…>`

Remove paths from the staging area without touching working-tree content.

---

### `dit commit -m|--message <MSG> -a|--author <AUTHOR>`

Create a new commit from the staging area.

> **Author format:** `"Name <email>"`
> Example: `-a "ComplexAirport <complexaiport@example.com>"`

---

### `dit branch …`

| Sub-command     | Purpose                                                          | Options                                   |
|-----------------|------------------------------------------------------------------|-------------------------------------------|
| `new <name>`    | Create a new branch which will point to the current commit head. |                                           |
| `switch <name>` | Switches to the specified branch                                 | `--hard` = throw away uncommitted changes |
| `remove <name>` | Removes a branch.                                                |                                           |

---

### `dit reset <COMMIT> [--mode <soft|mixed|hard>]`

| Mode                  | Description                                         |
|-----------------------|-----------------------------------------------------|
| **mixed** *(default)* | Retains the files not included in the target commit |
| **hard**              | Erases all new files and removes all the changes    |

---

### `dit clear`
Clears all the staged changes
