/*!
# Crate pkg_uild-remote

This crate provides the implementation for the  `pkg-get-remote`
command.

*/
pub mod vcs_system;
pub use vcs_system::VcsSystem;
pub mod build_request;
pub use build_request::*;
pub mod constants;
pub mod build_server;
pub use build_server::BuildServer;
pub mod platform;
pub use platform::Platform;
pub mod errors;
pub use errors::*;
pub mod flavor;
pub use flavor::get_flavors;
pub mod minifest;
pub use minifest::{Minifest};
pub mod svn;
pub use svn::Svn;
pub mod git;
pub use git::Git;