//!   cli.rs
//! 
//! This module defines the cli interface for the command
//! using the popular structopt crate

use structopt::StructOpt;
//use std::path::PathBuf;

// value defined as default in structopt structure
//const DEFAULT_PLATFORMS: &'static str = "cent7_64";

// waiting on cent6. this should end up going in a config...
//const DEFAULT_PLATFORMS: &'static str = "cent7_64,cent6_64";


#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
/// Trigger package builds on jenkins
///
/// Just provide your package name and tag, ad we do the rest. You may optionally
/// set specific flavors, or platform(s) as well.
///
/// If you do not supply flavors or platforms, we use the Packages Jenkins Pipeline
/// which will automagically figure out what flavors and platforms to build for.
///
/// If you specify flavors or platforms explicitly, we use the Build Distribution
/// Pipeline, which the Packages Pipeline delegates work to. Of the two, the 
/// Packages pipeline should be preferred when possible.
pub struct Opt {
    
    /// Optionally suppiy one or more flavours as a comma separated
    /// list. By default, pkg-build-remote will attempt to build all
    /// of the flavors defined in the manifest, if supplied. Otherwise,
    /// the vanilla flavor will be used.
    #[structopt(short = "f", long = "flavours")]
    pub flavours: Option<String>,

    /// Not fond of the British spelling? Register your disatisfaction at
    /// taxation without representation by using the American spelling. For the actual
    /// description, take a look at `flavours`.
    #[structopt(long = "flavors")]
    pub flavors: Option<String>,

    /// Specify the name of the package
    #[structopt(name = "PACKAGE")]
    pub name: String,

    /// Specify the tag which you wish to build
    #[structopt(name = "TAG")]
    pub tag: String,

    /// Optionally supply a list of one or more, comma separated platforms to build for.
    /// This is case insensitive.
    #[structopt(short = "p", long = "platforms")]
    pub platforms: Option<String>,

    /// Provide verbose feedback to stdout
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// When set to true, pkg-build-remote will report on its choices,
    /// but will not actually execute a remote build. May be used to
    /// verify input to the command.
    #[structopt(short = "d", long = "dry-run")]
    pub dry_run: bool,

    /// Present a prompt allowing the user to decide whether to submit the job
    /// after reviewing relevant information.
    #[structopt(short = "a", long = "ask")]
    pub prompt: bool,
}