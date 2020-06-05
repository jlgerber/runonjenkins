/*!
# Crate pkg_build-remote

This crate provides the implementation for the  `pkg-build-remote`
command.

*/
pub mod cli;

pub mod traits;

pub mod vcs_system;
pub use vcs_system::VcsSystem;

pub mod build_request;
pub use build_request::*;

pub mod build_param_type;
pub use build_param_type::*;

pub mod constants;

pub mod build_server;
pub use build_server::BuildServer;

pub mod platform;
pub use platform::Platform;

pub mod errors;
pub use errors::*;

pub mod flavor;
pub use flavor::Flavors;

pub mod from_manifest;
pub use from_manifest::minifest::Minifest;

pub mod svn;
pub use svn::Svn;

pub mod git;
pub use git::Git;

pub mod machine_os;
pub use machine_os::MachineOs;

pub mod from_gpi;
pub use from_gpi::gpi;
pub use from_gpi::packalaka_tags;

pub mod utils;

pub mod prelude {
    pub use super::traits::*;
}
