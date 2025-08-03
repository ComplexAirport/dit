mod manager;
mod private_helpers;
mod branch_switch;
mod branch_merge;
mod manipulate_head;

pub use manager::BranchMgr;
pub use branch_switch::*;
pub use branch_merge::*;
pub use manipulate_head::*;
pub use private_helpers::*;
