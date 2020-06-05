use structopt::StructOpt;
use std::path::PathBuf;

// value defined as default in structopt structure
//const DEFAULT_PLATFORMS: &'static str = "cent7_64";

// waiting on cent6. this should end up going in a config...
//const DEFAULT_PLATFORMS: &'static str = "cent7_64,cent6_64";


#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
/// Trigger package builds on jenkins
///
/// There are broadly two use cases for pkg-build-remote. The first use case
/// is relevant when operating from within a checked out project. In this case, contextual information
/// is gleaned from the manifest and environment, and overrides are, in general
/// optional.
/// The second use case is for triggering a build without the source code checked
/// out. In this case, one must supply the following options: scnm, flavors, vcs_url, name, tag.
pub struct Opt {
    
    /// The source code management system in use for the particular
    /// project. This must be used in two cases: (1) if building outside of 
    /// the package source, or (2) if both .svn and .git are present in the 
    /// package source.  
    /// Current choices: git | svn
    #[structopt(short = "s", long = "scm")]
    pub vcs: Option<String>,

    /// The path to the repository on disk, if it is not the current working
    /// directory. This is where the command will look for your vcs and
    /// manifest if warranted.
    #[structopt(short = "r", long = "repo-path", parse(from_os_str))]
    pub project_path: Option<PathBuf>,

    /// Use information gleaned from the local manifest and vcs system rather
    /// than using the GPI. Why would you do this? Why indeed.
    #[structopt(short = "l", long="local")]
    pub use_local_project: bool,
    
    /// If using the gpi (via --gpi) look up flavors in gpi and use them
    #[structopt(short="a", long="all-flavors")]
    pub all_flavors: bool,

    /// Optionally suppiy one or more flavours as a comma separated
    /// list. By default, pkg-build-remote will attempt to build all
    /// of the flavors defined in the manifest, if supplied. Otherwise,
    /// the vanilla flavor will be used.
    #[structopt(short = "f", long = "flavours")]
    pub flavours: Option<String>,

    /// Optionally suppiy the vcs url
    #[structopt(short = "v", long = "vcs-url")]
    pub vcs_url: Option<String>,

    /// Optionally specify the name of the package instead of pulling it from
    /// the manifest.
    #[structopt(short = "n", long = "name")]
    pub name: Option<String>,

    /// Optionally specify the tag to build from instead of pulling it from the
    /// manifest. It is assumed that the tag and version are identical.
    #[structopt(short = "t", long = "tag")]
    pub tag: Option<String>,

    /// Not fond of the British spelling? Register your disatisfaction at
    /// taxation without representation by using the American spelling. For the actual
    /// description, take a look at `flavours`.
    #[structopt(long = "flavors")]
    pub flavors: Option<String>,

    /// Optionally supply a list of one or more, comma separated platforms to build for.
    /// This is case insensitive.
    #[structopt(short = "p", long = "platforms", default_value = "cent7")]
    pub platforms: String,

    /// Provide verbose feedback to stdout
    #[structopt(long = "verbose")]
    pub verbose: bool,

    /// When set to true, pkg-build-remote will report on its choices,
    /// but will not actually execute a remote build. May be used to
    /// verify input to the command.
    #[structopt(short = "d", long = "dry-run")]
    pub dry_run: bool,
    /// Present a prompt allowing the user to decide whether to submit the job
    /// after reviewing relevant information.
    #[structopt(short = "a", long = "prompt")]
    pub prompt: bool,
}