use pkg_build_remote::{Svn, Git, Minifest, BuildRequest,BuildServer, Platform, VcsSystem};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// The source code management system in use for the particular
    /// project. This is optional, as pkg-build-remote will attempt to
    /// identify the system by inspecting disk. Current choices: git | svn

    #[structopt(short = "s", long = "scm")]
    vcs: Option<String>,
    /// Suppiy one or more flavours to build as a comma separated
    /// list. By default, pkg-build-remote will attempt to build the
    /// vanilla flavour. Ths option is case insensitive.

    #[structopt(short = "f", long = "flavours", default_value="^")]
    flavors: String,
    /// Optionally supply a list of one or more, comma separated, platforms.
    /// Valid choices include cent6 | cent6_64 | cent7 | cent7_64. This
    /// is case insensitive
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


fn identify_vcs(selection: &Option<String>) -> Option<VcsSystem> {
    match selection {
        Some(val) => {
            println!("vcs predefined");
            return Some(VcsSystem::from(val.as_str()))
        }
        None => {
            if Git::is_cwd_repo() {
                println!("git found");
                return Some(VcsSystem::from("git"));
            }
            if Svn::is_cwd_repo() {
                println!("svn found");
                return Some(VcsSystem::from("svn"))
            }
        }
    }
    None
}

fn parse_flavors(flavor: &str) -> Vec<&str> {
    flavor.split(",").map(|x| x.trim()).collect::<Vec<&str>>()
}

fn parse_platforms(platforms: &str) -> Vec<Platform> {
    platforms
    .split(",")
    .map(|x| x.trim())
    .map(|x| Platform::from(x))
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

//fn trigger_requests(minifest: &Minifest, vcs: &VcsSystem, platforms: &str, )
fn main() -> Result<(), failure::Error> {
    let opts = Opt::from_args();

    let vcs = identify_vcs(&opts.vcs);

    if vcs.is_none() {
        println!("Error: No VCS system idemtified");
        std::process::exit(1);
    }
    let vcs = vcs.unwrap();

    let flavors = parse_flavors(&opts.flavors);

    let build_server = BuildServer::default();

    // get minifest
    let minifest = Minifest::from_disk(None)?;

    let platforms = parse_platforms(&opts.platforms);

    println!("{:?}", minifest);
    match vcs {
        VcsSystem::Svn => {
            let remotes = Svn::get_url(minifest.version.as_str())?;

            if opts.verbose{ println!("{:#?}", remotes); }
            for platform in platforms {
                let build_reqs = build_requests(&minifest, remotes.as_str(), &VcsSystem::Svn, &platform, &flavors);
                for br in build_reqs {
                    if opts.verbose{ println!("{:#?}", br); }
                    if opts.dry_run {
                        println!("dry_run mode");
                        println!("route {:?}", build_server.request_route());
                        println!("build params: {:#?}", br.to_build_params());
                    } else {
                    let _results = build_server.request_build(&br, opts.verbose, opts.dry_run)?;
                    }
                }
            }
        },
         VcsSystem::Git => {
            let cwd = env::current_dir()?;
            let remotes = Git::get_remotes_strings(cwd.to_str().unwrap())?;
            if opts.verbose{ println!("{:#?}", remotes); }
            for platform in platforms {
                let build_reqs = build_requests( &minifest, remotes[0].as_str(), &VcsSystem::Git, &platform, &flavors );
                for br in build_reqs {
                    if opts.verbose { println!("{:#?}", br) };
                    if opts.dry_run {
                        println!("dry_run mode");
                        println!("route {:?}", build_server.request_route());
                        println!("build params: {:#?}", br.to_build_params());
                    } else {
                    let _results = build_server.request_build(&br, opts.verbose, opts.dry_run)?;
                    }
                }
            }
        }
        _ => {
            println!("choose svn or git");
            std::process::exit(1);
        }
    }

    Ok(())
}

