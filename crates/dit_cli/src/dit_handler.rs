use crate::cli::CommandKind;
use dit_core::dit::Dit;
use dit_core::resolve_absolute_path;
use std::io;
use std::path::{Path, PathBuf};

pub struct DitHandler {
    dit: Dit,
}

impl DitHandler {
    pub fn from<P: AsRef<Path>>(project_path: P) -> io::Result<Self> {
        let project_path = project_path.as_ref();
        let dit = Dit::from(project_path)?;
        Ok(Self {
            dit,
        })
    }

    pub fn handle(&mut self, command: CommandKind) -> io::Result<()> {
        match command {
            CommandKind::Add { file } => self.handle_add(file),
            CommandKind::Unstage { file } => self.handle_unstage(file),
            CommandKind::Commit { author, message } => self.handle_commit(author, message),
        }
    }
}

impl DitHandler {
    pub fn handle_add(&mut self, file: PathBuf) -> io::Result<()> {
        let file = resolve_absolute_path(&file)?;
        self.dit.stage(&file)?;
        println!("[+] Added `{}` to the staged files", file.display());
        Ok(())
    }

    pub fn handle_unstage(&mut self, file: PathBuf) -> io::Result<()> {
        let file = resolve_absolute_path(&file)?;
        self.dit.unstage(&file)?;
        println!("[+] Unstaged the file `{}`", file.display());
        Ok(())
    }

    pub fn handle_commit(&mut self, author: String, message: String) -> io::Result<()> {
        self.dit.commit(&author, &message)?;
        println!("[+] {}", author);
        println!(" -  {}", message);
        Ok(())
    }
}
