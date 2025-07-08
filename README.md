# Dit

**Dit** is a minimal version control system inspired by Git.  
It supports staging, committing, and viewing history, etc.

---

## Usage

```bash
dit <COMMAND> [OPTIONS]
```

---

##  Available Commands

### `init`

Initialize a new dit repository in the current directory.

```bash
dit init
```

---

### `status`

Show the current repository status: staged files, unstaged changes, and untracked files.

```bash
dit status
```

---

### `add <files...>`

Stage one or more files for the next commit.

```bash
dit add file1.txt dir/file2.rs
```

---

### `unstage <files...>`

Remove one or more files from the staging area.

```bash
dit unstage file1.txt dir/file2.rs
```

---

### `commit -m <message> -a <author>`

Create a new commit with a message and author.

```bash
dit commit -m "Initial commit" -a "ComplexAirport"
```

---

### `history [-c <count>]`

Show the latest commits (default: 5).

```bash
dit history
dit history -c 10
```

---

## Example Workflow

```bash
dit init
dit add src/main.rs
dit add Cargo.toml
dit status
dit commit -m "Initial commit" -a "ComplexAirport"
dit history
```
