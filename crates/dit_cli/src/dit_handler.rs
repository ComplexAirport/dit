use crate::cli::CommandKind;
use crate::error::{CliResult, DitCliError};
use dit_core::{
    dit::Dit,
    helpers::resolve_absolute_path,
    DIT_ROOT,
};
use std::path::{Path, PathBuf};

pub struct DitHandler {
    dit: Option<Dit>,
}

impl DitHandler {
    pub fn new() -> CliResult<Self> {
        let cwd = std::env::current_dir()
            .map_err(|_| DitCliError::CwdError)?;

        let project_root = Self::find_dit_root(cwd);
        match project_root {
            Some(project_root) => {
                let dit = Dit::from(project_root)?;
                Ok(Self { dit: Some(dit) })
            },
            None => Ok(Self{ dit: None })
        }
    }

    pub fn handle(&mut self, command: CommandKind) -> CliResult<()> {
        match command {
            CommandKind::Init => self.handle_init(),
            CommandKind::History { count } => self.handle_history(count),
            CommandKind::Status => self.handle_status(),
            CommandKind::Add { files } => self.handle_add(files),
            CommandKind::Unstage { files } => self.handle_unstage(files),
            CommandKind::Commit { author, message } => self.handle_commit(author, message),
            CommandKind::Branch { name, new } => self.handle_branch(name, new),
        }
    }

    /// Returns the [`Dit`] if the `.dit` project was found, otherwise prints an error message
    /// and exits the program
    pub fn get_dit(&mut self) -> &mut Dit {
        match &mut self.dit {
            Some(dit) => dit,
            None => {
                eprintln!("error: not a dit project (or any of the parent directories)");
                eprintln!("hint: initialize with `dit init`");
                std::process::exit(1);
            }
        }
    }
}

impl DitHandler {
    pub fn handle_init(&mut self) -> CliResult<()> {
        let cwd = std::env::current_dir()
            .map_err(|_| DitCliError::CwdError)?;
        let mut dit = Dit::from(&cwd)?;

        // default behavior:
        // if no head branch is found, a default "main" branch will be created
        if dit.branch().is_none() {
            dit.create_branch("main")?;
        }

        self.dit = Some(dit);
        println!("[+] Initialized a new dit project.");
        Ok(())
    }

    pub fn handle_history(&mut self, count: isize) -> CliResult<()> {
        let dit = self.get_dit();

        let branch_name = dit.branch();
        let commits = dit.history(count)?;

        if let Some(branch_name) = branch_name {
            println!("History for the branch '{branch_name}':\n");
        } else {
            println!("History (no head):\n");
        }

        for (idx, commit) in commits.iter().enumerate() {
            let hash_slice = &commit.hash[0..8];
            println!("  {}. {hash_slice}..", idx + 1);
            println!("{} - {}", commit.author, commit.message);
        }

        Ok(())
    }

    pub fn handle_status(&mut self) -> CliResult<()> {
        // todo: this needs to be changed
        // also need to display files which are not tracked
        // and also the files which were changed compared to the last commit
        // or last time being staged

        let dit = self.get_dit();
        let branch_name = dit.branch();
        let staged_files = dit.staged_files()?;

        if let Some(branch_name) = branch_name {
            println!("On branch '{branch_name}'")
        } else {
            println!("No current branch");
        }

        println!();
        println!("Changes to be committed: ");
        for path in staged_files.files.keys() {
            println!("       {}", path.display());
        }

        Ok(())
    }

    pub fn handle_add(&mut self, files: Vec<PathBuf>) -> CliResult<()> {
        for file in files {
            let abs_path = resolve_absolute_path(&file)?;
            self.get_dit().stage(&abs_path)?;
            println!("[+] Added '{}' to the staged files", file.display());
        }
        Ok(())
    }

    pub fn handle_unstage(&mut self, files: Vec<PathBuf>) -> CliResult<()> {
        for file in files {
            let abs_path = resolve_absolute_path(&file)?;
            self.get_dit().unstage(&abs_path)?;
            println!("[+] Unstaged the file `{}`", file.display());
        }
        Ok(())
    }

    pub fn handle_commit(&mut self, author: String, message: String) -> CliResult<()> {
        self.get_dit().commit(&author, &message)?;
        println!("[+] Committed the changes");
        Ok(())
    }

    pub fn handle_branch(&mut self, name: String, is_new: bool) -> CliResult<()> {
        if is_new {
            self.get_dit().create_branch(&name)?;
            println!("[+] Created a new branch '{name}'");
        } else {
            eprintln!("[-] switching branches is not supported yet"); // todo
        }
        Ok(())
    }
}

impl DitHandler {
    /// Recursively searches for `.dit` starting from `start_dir` \
    /// Returns the path to the root of the dit repo if found, None otherwise
    fn find_dit_root<P: AsRef<Path>>(start_dir: P) -> Option<PathBuf> {
        let start_dir = start_dir.as_ref();
        let mut current = Some(start_dir);

        while let Some(dir) = current {
            if dir.join(DIT_ROOT).is_dir() {
                return Some(dir.to_path_buf());
            }
            current = dir.parent();
        }
        None
    }
}
