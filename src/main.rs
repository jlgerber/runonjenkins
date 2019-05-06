use pkg_build_remote::{
    get_flavors, traits::*, BuildRequest, BuildServer, Git, Minifest, Platform, RemoteBuildError,
    Svn, VcsSystem,
};
use prettytable::{table, row, cell, format};
use pretty_env_logger;
use log::{debug, info, error};
use std::{env, io::{stdout, stdin, Write}, path::{Path, PathBuf}};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// The source code management system in use for the particular
    /// project. This is optional, as pkg-build-remote will attempt to
    /// identify the system by scanning the current working directory, or
    /// the directory supplied via the --repo-path flag in
    /// order to identify the correct version control system.
    /// Current choices: git | svn
    #[structopt(short = "s", long = "scm")]
    vcs: Option<String>,

    /// The path to the repository on disk, if it is not the current working
    /// directory. This is where the command will look for your vcs and
    /// manifest if warranted.
    #[structopt(short = "r", long = "repo-path", parse(from_os_str))]
    project_path: Option<PathBuf>,

    /// Optionally suppiy one or more flavours as a comma separated
    /// list. By default, pkg-build-remote will attempt to build all
    /// of the flavors defined in the manifest.
    #[structopt(short = "f", long = "flavours")]
    flavours: Option<String>,

    /// Not fond of the British spelling? Register your disatisfaction at
    /// taxation without representation by using the American spelling.
    #[structopt(long = "flavors")]
    flavors: Option<String>,

    /// Optionally supply a list of one or more, comma separated platforms to build for.
    /// This is case insensitive.
    #[structopt(short = "p", long = "platforms", default_value = "cent7")]
    platforms: String,

    /// Provide verbose feedback to stdout
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// When set to true, pkg-build-remote will report on its choices,
    /// but will not actually execute a remote build. May be used to
    /// verify input to the command.
    #[structopt(short = "n", long = "dry-run")]
    dry_run: bool,
    /// Present a prompt allowing the user to decide whether to submit the job
    /// after reviewing relevant information.
    #[structopt(short = "a", long = "prompt")]
    prompt: bool,
}

// Given a reference to an Option<String> where the String is the vcs choice,
// and a path to the location where we are to look in the event that the selection is None,
// return a VcsSystem instance based either on supplied name (selection), or identification
// from the path. In the event that the user has supplied an invalid VcsSystem or path,
// this function will report an error to stderr and exit the process.
fn identify_vcs(selection: &Option<String>, path: &Path, verbose: bool) -> VcsSystem {
    match selection {
        Some(val) => {
            debug!("vcs predefined");
            let vcs_val = VcsSystem::from(val.as_str());
            if let VcsSystem::Unknown(v) = vcs_val {
                error!("Unknown vcs system: {}", v);
                std::process::exit(1);
            }
            return vcs_val;
        }
        None => {
            if Git::is_repo(path) {
                debug!("git found");
                return VcsSystem::from("git");
            }
            if Svn::is_repo(path) {
                debug!("svn found");
                return VcsSystem::from("svn");
            }
        }
    }
    error!("Error: No VCS system idemtified");
    std::process::exit(1);
}

// Convert a &str of comma separated flavor names into a
// vector of flavor name `&str`s
fn parse_flavors(flavor: &str) -> Vec<&str> {
    flavor.split(",").map(|x| x.trim()).collect::<Vec<&str>>()
}

// Given a &str of potentially comma separated platform names,
// convert them to Platform instances, filtering out Platform::Unknowns
fn parse_platforms(platforms: &str) -> Vec<Platform> {
    platforms
        .split(",")
        .map(|x| x.trim())
        .map(|x| Platform::from(x))
        // filter out any platforms that are unknown
        .filter(|x| {
            if let Platform::Unknown(_) = x {
                false
            } else {
                true
            }
        })
        .collect::<Vec<Platform>>()
}

// Construct a Vector of BuildRequest instances, one per flavor.
// The BuildRequest provides a method that produces a struct
// which is serializable into json in the form that Jenkins
// is looking for
fn build_requests(
    minifest: &Minifest,
    repo: &str,
    scm_type: &VcsSystem,
    platform: &Platform,
    flavors: &Vec<&str>,
) -> Vec<BuildRequest> {
    let mut build_reqs = Vec::with_capacity(flavors.len());
    for flav in flavors {
        build_reqs.push(
            BuildRequest::new(
                minifest.name.as_str(),
                minifest.version.as_str(),
                flav,
                repo,
                scm_type,
                platform,
            )
            .unwrap(),
        );
    }
    build_reqs
}

// Trigger a build on the given build server, with the project identified
// using the supplied parameters. Of course, if dry_run is true, then simply
// pretend to do a build.
//
// # Parameters
//
// * `build_server` - The build server instance.
// * `minifest`     - The minifest instance, which supplies the name and version from the manifest
// * `vcs_project_url` - The url of the project on the vcs server (eg svn or gitlab)
// * `vcs`             - The version control system (eg git, svn)
// * `platforms`       - A comma separated list of platforms to build for (eg cent6,cent7)
// * `flavors`         - A comma separated list of flavors to build. (eg ^,maya)
// * `dry_run`         - Whether or not we are in dry run mode
// * `verbose`         - Whether or not we are in verbose mode
// * `prompt`          - prompt the user as to whether the user wishes to execute a remote build after
//                       being presented with relevant details.
// # Returns
// Result wrapping () if successful, or Failure if unsuccessful
fn request_build_for(
    build_server: &BuildServer,
    minifest: &Minifest,
    vcs_project_url: &url::Url,
    vcs: &VcsSystem,
    platforms: &str,
    flavors: &str,
    dry_run: bool,
    verbose: bool,
    prompt: bool,
) -> Result<(), failure::Error> {
    let platforms = parse_platforms(platforms);
    let flavors = parse_flavors(flavors);

    debug!("{:?}", vcs_project_url);

    if dry_run || verbose || prompt {
        let platform_str: Vec<String> = platforms.iter().map(|x| x.to_string()).collect();
        let platform_str = platform_str.join(" , ");

        let mut table = table!(
            [FYbH2c -> "Remote Build Request Information"],
            [FYb -> "Route",     Fwb -> build_server.request_route().unwrap()],
            [FYb -> "Project",   Fwb ->  minifest.name],
            [FYb -> "VCS Tag",   Fwb -> minifest.version],
            [FYb -> "Flavors",   Fwb -> flavors.join(" , ").as_str()],
            [FYb -> "VCS Repo",  Fwb -> vcs_project_url.as_str()],
            [FYb -> "Platforms", Fwb -> platform_str.as_str()]
        );
        // FORMAT_CLEAN
        // FORMAT_NO_COLSEP
        // FORMAT_BORDERS_ONLY
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
        println!("");
        table.printstd();
        println!("");
    }
    if prompt {
        print!("Do you wish to submit a build request? (y/n) ");
        stdout().flush().ok().expect("unable to flush stdout");
        let reader = stdin();
        let mut result = String::new();
        let _ = reader.read_line(&mut result).ok().expect("Failed to read line");
        result = result.to_lowercase();
        if result != "y" && result != "yes" {
            println!("User cancelled build request: {}", result);
            std::process::exit(0);
        }
    }
    for platform in platforms {
        let build_reqs =
            build_requests(&minifest, vcs_project_url.as_str(), vcs, &platform, &flavors);
        for br in build_reqs {
            debug!("{:?}", br);
            let _results = build_server.request_build(&br, verbose, dry_run)?;
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
// A String if successful. Otherwise, a RemoteBuildError
fn resolve_flavors(
    flavors: Option<String>,
    flavours: Option<String>,
    path: Option<&std::path::Path>,
) -> Result<String, RemoteBuildError> {
    if flavours.is_some() && flavors.is_some() {
        error!("Using --falvours and --flavors? You cheeky monkey. Pick one or the other");
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
    pretty_env_logger::init();

    let opts = Opt::from_args();
    let project_path = opts.project_path.unwrap_or(env::current_dir()?);
    let flavors = resolve_flavors(opts.flavors, opts.flavours, Some(&project_path))?;
    let vcs = identify_vcs(&opts.vcs, &project_path, opts.verbose);
    let build_server = BuildServer::default();
    let minifest = Minifest::from_disk(Some(&project_path))?;

    debug!("{:?}", minifest);

    let result = match vcs {
        VcsSystem::Svn => {
            let vcs_project_url = Svn::get_url(minifest.version.as_str())?;

            request_build_for(
                &build_server,
                &minifest,
                &vcs_project_url,
                &vcs,
                &opts.platforms,
                &flavors,
                opts.dry_run,
                opts.verbose,
                opts.prompt,
            )
        }
        VcsSystem::Git => {
            let vcs_project_url = Git::get_server_urls(&project_path)?;
            let vcs_project_url = &vcs_project_url[0];

            request_build_for(
                &build_server,
                &minifest,
                &vcs_project_url,
                &vcs,
                &opts.platforms,
                &flavors,
                opts.dry_run,
                opts.verbose,
                opts.prompt,
            )
        }
        _ => {
            error!("SCM must either be svn or git");
            std::process::exit(1);
        }
    };
    match result {
        Err(e) => {
            error!("{}",e.cause());
            std::process::exit(1);
        }
        _ => ()
    };
    Ok(())
}
