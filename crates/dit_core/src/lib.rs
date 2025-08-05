mod managers;
use managers::{blob, tree, commit, stage, branch};

pub mod errors;
pub mod helpers;
mod models;
mod api;

pub use api::{Dit, DIT_ROOT, Repo};
