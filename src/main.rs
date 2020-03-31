use failure::AsFail;
use log::{debug, error};
use pkg_build_remote::{
    traits::*, BuildRequest, BuildServer, Git, Minifest, RemoteBuildError,
    Svn, VcsSystem, utils,
};
use pretty_env_logger;
use prettytable::{cell, format, row, table};
use std::{
    env,
    io::{stdin, stdout, Write},
    path::PathBuf,
};
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

    /// Optionally specify the name of the package instead of pulling it from
    /// the manifest.
    #[structopt(short = "n", long = "name")]
    name: Option<String>,

    /// Optionally specify the tag to build from instead of pulling it from the
    /// manifest (in the form of the version)
    #[structopt(short = "t", long = "tag")]
    tag: Option<String>,

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
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,
    /// Present a prompt allowing the user to decide whether to submit the job
    /// after reviewing relevant information.
    #[structopt(short = "a", long = "prompt")]
    prompt: bool,
}

// Trigger a build on the given build server, with the project identified
// using the supplied parameters. Of course, if dry_run is true, then simply
// pretend to do a build.
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
    let platforms = utils::parse_platforms(platforms);
    let flavors = utils::parse_flavors(flavors);

    debug!("{:?}", vcs_project_url);

    if dry_run || verbose || prompt {
        let platform_str: Vec<String> = platforms.iter().map(|x| x.to_string()).collect();
        let platform_str = platform_str.join(" , ");

        let mut table = table!(
            [FYbH2c -> "Remote Build Request Information"],
            [FYb -> "Route",     Fwb -> build_server.request_route().ok_or(RemoteBuildError::EmptyError("unable to unwrap request_route".into()))?],
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
        let _ = reader
            .read_line(&mut result)
            .ok()
            .expect("Failed to read line");
        result = result.to_lowercase();
        if result != "y" && result != "yes" {
            println!("User cancelled build request: {}", result);
            std::process::exit(0);
        }
    }
    for platform in platforms {
        let build_reqs = BuildRequest::build_requests(
            &minifest,
            vcs_project_url.as_str(),
            vcs,
            &platform,
            &flavors,
        )?;
        for br in build_reqs {
            debug!("{:?}", br);
            let _results = build_server.request_build(&br, verbose, dry_run)?;
        }
    }
    Ok(())
}


fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();

    let opts = Opt::from_args();
    let project_path = opts.project_path.unwrap_or(env::current_dir()?);
    let flavors = utils::resolve_flavors(opts.flavors, opts.flavours, Some(&project_path));
    if flavors.is_err() {
        let e = flavors.unwrap_err();
        error!("Unable to resolve flavors: {}.", e.as_fail());
        std::process::exit(1);
    }
    let flavors = flavors.unwrap();
    let vcs = VcsSystem::identify_vcs(&opts.vcs, &project_path);
    let build_server = BuildServer::default();

    let minifest = utils::get_minifest(&project_path, &opts.name, &opts.tag); //Minifest::from_disk(Some(&project_path));
    if minifest.is_err() {
        let e = minifest.unwrap_err();
        error!("Problem with manifest. {}", e.as_fail());
        std::process::exit(1);
    }
    let minifest = minifest.unwrap();
    debug!("{:?}", minifest);

    let result = match vcs {
        VcsSystem::Svn => {
            //let vcs_project_url = Svn::get_url(&project_path, minifest.version.as_str())?;
            let vcs_project_url = Svn::get_server_urls(&project_path)?;
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
            error!("{}", e.as_fail());
            std::process::exit(1);
        }
        _ => (),
    };
    Ok(())
}
