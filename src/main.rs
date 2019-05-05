use pkg_build_remote::{get_flavors, Svn, Git, Minifest, RemoteBuildError, BuildRequest, BuildServer, Platform, VcsSystem, traits::*};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// The source code management system in use for the particular
    /// project. This is optional, as pkg-build-remote will attempt to
    /// identify the system by scanning the current working directory in
    /// order to identify the correct version control system.
    /// Current choices: git | svn
    #[structopt(short = "s", long = "scm")]
    vcs: Option<String>,

    /// Optionally suppiy one or more flavours to build as a comma separated
    /// list. By default, pkg-build-remote will attempt to build the
    /// vanilla flavour.
    #[structopt(short = "f", long = "flavours")]
    flavours: Option<String>,

    /// Not fond of the British spelling? Register your disatisfaction at
    /// taxation without representation by using the American spelling.
    #[structopt(long = "flavors")]
    flavors: Option<String>,

    /// Optionally supply a list of one or more, comma separated platforms to build for.
    /// This is case insensitive.
    #[structopt(short = "p", long = "platforms", default_value="cent7")]
    platforms: String,

    /// Provide verbose feedback to stdout
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// When set to true, pkg-build-remote will report on its choices,
    /// but will not actually execute a remote build. May be used to
    /// verify input to the command.
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,

}


fn identify_vcs(selection: &Option<String>) -> VcsSystem {
    match selection {
        Some(val) => {
            println!("vcs predefined");
            return VcsSystem::from(val.as_str())
        }
        None => {
            if Git::is_cwd_repo() {
                println!("git found");
                return VcsSystem::from("git");
            }
            if Svn::is_cwd_repo() {
                println!("svn found");
                return VcsSystem::from("svn");
            }
        }
    }
    println!("Error: No VCS system idemtified");
    std::process::exit(1);
}

fn parse_flavors(flavor: &str) -> Vec<&str> {
    flavor
    .split(",")
    .map(|x| x.trim())
    .collect::<Vec<&str>>()
}

fn parse_platforms(platforms: &str) -> Vec<Platform> {
    platforms
    .split(",")
    .map(|x| x.trim())
    .map(|x| Platform::from(x))
    // filter out any platforms that are unknown
    .filter(|x| if let Platform::Unknown(_) = x { false } else { true } )
    .collect::<Vec<Platform>>()
}

fn build_requests(
    minifest: &Minifest,
    repo: &str,
    scm_type: &VcsSystem,
    platform: &Platform,
    flavors: &Vec<&str>
) ->Vec<BuildRequest>
{
    let mut build_reqs = Vec::with_capacity(flavors.len());
    for flav in flavors{
        build_reqs.push(
            BuildRequest::new(
                minifest.name.as_str(),
                minifest.version.as_str(),
                flav,
                repo,
                scm_type,
                platform
            ).unwrap()
        );
    }
    build_reqs
}


fn request_build_for (
    build_server:  &BuildServer,
    minifest:      &Minifest,
    remote_server: &url::Url,
    vcs:           &VcsSystem,
    platforms:     &str,
    flavors:       &str,
    dry_run:       bool,
    verbose:       bool )
-> Result<(), failure::Error> {

    let platforms = parse_platforms(platforms);
    let flavors   = parse_flavors(flavors);

    if verbose{ println!("{:#?}", remote_server); }
    for platform in platforms {
        let build_reqs = build_requests(&minifest, remote_server.as_str(), vcs, &platform, &flavors);
        for br in build_reqs {
            if verbose{ println!("{:#?}", br); }
            if dry_run {
                println!("dry_run mode");
                println!("route {:?}", build_server.request_route());
                println!("build params: {:#?}", br.to_build_params());
            } else {
            let _results = build_server.request_build(&br, verbose, dry_run)?;
            }
        }
    }
    Ok(())
}

// Given flavors and flavours options from the command line, reconcile the two and identify
// the requested flavors. This function will guard against specifying both flavors and flavours,
// exiting the process if neither is None.
// Furthermore, if both flavors and `flavours` are None, `resolve_flavors` will retrieve the
// full list of flavors from the manifest.
//
// # Parameters
//
// * `flavors`  - an Option<String> populated via the --flavors flag of the cli.
// * `flavours` - an Option<String> populated via the --flavours flag of the cli.
// * `path`     - an Option<&Path> representing the path to the root of the vcs, where
//                we expect to find the manifest.
// # Returns
//
// A
fn resolve_flavors(
    flavors: Option<String>,
    flavours: Option<String>,
    path: Option<&std::path::Path>
) -> Result<String, RemoteBuildError>
{
    if flavours.is_some() && flavors.is_some() {
        eprintln!("Using --falvours and --flavors? You cheeky monkey. Pick one or the other");
        std::process::exit(1);
    }

    let flavors = if flavours.is_none() && flavours.is_none() {
        get_flavors(path)?.join(".")
    } else if flavours.is_some() {
        flavours.unwrap()
    } else {
        flavors.unwrap()
    };
    Ok(flavors)
}

fn main() -> Result<(), failure::Error> {
    let opts = Opt::from_args();
    let cwd = env::current_dir()?;
    let flavors      = resolve_flavors(opts.flavors, opts.flavours, Some(&cwd))?;
    let vcs          = identify_vcs(&opts.vcs);
    let build_server = BuildServer::default();
    let minifest     = Minifest::from_disk(None)?;

    if opts.verbose { println!("{:?}", minifest) };

    match vcs {
        VcsSystem::Svn => {
            let remote_server = Svn::get_url(minifest.version.as_str())?;

            let _ = request_build_for(
                &build_server,
                &minifest,
                &remote_server,
                &vcs,
                &opts.platforms,
                &flavors,
                opts.dry_run,
                opts.verbose
            )?;
        },
        VcsSystem::Git => {
            let remote_server =  Git::get_server_urls(&cwd)?;
            let remote_server = &remote_server[0];

            let _ = request_build_for(
                &build_server,
                &minifest,
                &remote_server,
                &vcs,
                &opts.platforms,
                &flavors,
                opts.dry_run,
                opts.verbose
            )?;
        },
        _ => {
            eprintln!("SCM must either be svn or git");
            std::process::exit(1);
        }
    }

    Ok(())
}

