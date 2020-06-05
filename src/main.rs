use log::{debug, error};
use pkg_build_remote::{
    traits::*, BuildRequest, BuildServer, Git, Minifest, RemoteBuildError,
    Svn, VcsSystem, utils, Platform, Flavours, packalaka_tags::PackageTagList
};
use pretty_env_logger;
use prettytable::{cell, format, row, table};
use std::{
    env,
    io::{stdin, stdout, Write},
    path::PathBuf,
};
use structopt::StructOpt;

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
struct Opt {
    
    /// The source code management system in use for the particular
    /// project. This must be used in two cases: (1) if building outside of 
    /// the package source, or (2) if both .svn and .git are present in the 
    /// package source.  
    /// Current choices: git | svn
    #[structopt(short = "s", long = "scm")]
    vcs: Option<String>,

    /// The path to the repository on disk, if it is not the current working
    /// directory. This is where the command will look for your vcs and
    /// manifest if warranted.
    #[structopt(short = "r", long = "repo-path", parse(from_os_str))]
    project_path: Option<PathBuf>,

    /// Use the GPI to get project information rather than scraping disk. This 
    /// affords the user the luxury of running this code from anywhere as opposed
    /// to from within project directory
    #[structopt(short = "g", long="gpi")]
    use_gpi: bool,
    
    /// If using the gpi (via --gpi) look up flavors in gpi and use them
    #[structopt(short="a", long="all-flavors")]
    all_flavors: bool,

    /// Optionally suppiy one or more flavours as a comma separated
    /// list. By default, pkg-build-remote will attempt to build all
    /// of the flavors defined in the manifest, if supplied. Otherwise,
    /// the vanilla flavor will be used.
    #[structopt(short = "f", long = "flavours")]
    flavours: Option<String>,

    /// Optionally suppiy the vcs url
    #[structopt(short = "v", long = "vcs-url")]
    vcs_url: Option<String>,

    /// Optionally specify the name of the package instead of pulling it from
    /// the manifest.
    #[structopt(short = "n", long = "name")]
    name: Option<String>,

    /// Optionally specify the tag to build from instead of pulling it from the
    /// manifest. It is assumed that the tag and version are identical.
    #[structopt(short = "t", long = "tag")]
    tag: Option<String>,

    /// Not fond of the British spelling? Register your disatisfaction at
    /// taxation without representation by using the American spelling. For the actual
    /// description, take a look at `flavours`.
    #[structopt(long = "flavors")]
    flavors: Option<String>,

    /// Optionally supply a list of one or more, comma separated platforms to build for.
    /// This is case insensitive.
    #[structopt(short = "p", long = "platforms", default_value = "cent7")]
    platforms: String,

    /// Provide verbose feedback to stdout
    #[structopt(long = "verbose")]
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

// Main captures and presents results
fn main() {
    match main_() {
        Ok(_) => (),
        Err(e) => {
            println!("\nERROR");
            println!("{}\n", e);
        }
    }
}

// Inner main function returning a result
fn main_() -> Result<(), RemoteBuildError> {
    pretty_env_logger::init();
    debug!("Initialized");
    let opts = Opt::from_args();
    
    let result = if opts.use_gpi {
        do_gpi(opts)
    } else {
        do_local(opts)
    };

    match result {
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
        _ => (),
    };
    Ok(())
}


// set up and execute the build using information gleaned from the gpi
fn do_gpi(opts: Opt) ->  Result<(), RemoteBuildError> {
    
    let name = opts.name.ok_or(RemoteBuildError::EmptyError("Missing name. Must be supplied".into()))?;
    let tag = opts.tag.ok_or(RemoteBuildError::EmptyError("Missing tag. Must be supplied".into()))?;


    let tags = PackageTagList::from_service(&name, &tag)?;
    debug!("PackageTag {:#?}", tags);

    if tags.len() == 0 {
        return Err(RemoteBuildError::EmptyError(format!("No Records exist for {}-{}", &name, &tag)));
    }

    let build_server = BuildServer::default();

    let distribution = tags.get(0).unwrap(); // already testing that tags.len() > 0 above
    
    let flavors = if opts.all_flavors {
        tags.get(0).unwrap().flavors().join(",")
    } else if opts.flavors.is_some() {
        opts.flavors.unwrap()
    } else if opts.flavours.is_some() {
        opts.flavours.unwrap()
    } else {
        "^".into()
    };
    println!("flavors: {}", flavors);


    request_build_for(
        &build_server,
        &name,
        &tag,
        &distribution.link()?,
        &distribution.uses, //vcs
        &opts.platforms,
        &flavors,
        opts.dry_run,
        opts.verbose,
        opts.prompt,
    )

}

// set up the build using local information gleaned from the manifest and the local vcs repo
fn do_local( opts: Opt) ->  Result<(), RemoteBuildError> {

    debug!("retrieving project path");
    let project_path = opts.project_path.unwrap_or(env::current_dir()?);
    debug!("project_path: {:?}", project_path);

    debug!("retrieving flavors");
    let flavors = Flavours::resolve_flavors(opts.flavors, opts.flavours, Some(&project_path));
   
    if flavors.is_err() {
        let e = flavors.unwrap_err();
        error!("Unable to resolve flavors: {}.", e);
        std::process::exit(1);
    }

    let flavors = flavors.unwrap();
    debug!("flavors retrieved: {:?}", flavors);

    debug!("identifying vcs system");
    let vcs = VcsSystem::identify_vcs(&opts.vcs, &project_path);
    debug!("VCS system {:?}", vcs);

    let build_server = BuildServer::default();

    debug!("retrieving name and version");
    let (name, version) = if opts.name.is_some() && opts.tag.is_some() {
        (opts.name.unwrap(), opts.tag.unwrap())
    } else {
        let minifest = utils::get_minifest(&project_path, &opts.name, &opts.tag); //Minifest::from_disk(Some(&project_path));
        
        if  let Ok(Minifest{name, version}) = minifest {
            (name,version)

        }   else {
            let e = minifest.unwrap_err();
            error!("Problem with manifest. {}", e);
            std::process::exit(1);
        }    
    };

    debug!("name: {:?} version: {:?}", name, version);
   
    build_from_vcs(
        &project_path, 
        &opts.vcs_url,
        &flavors, 
        &vcs, 
        &opts.platforms,
        &build_server, 
        &name, 
        &version,
         opts.dry_run,
        opts.verbose,
        opts.prompt,
    )
}



// execute a build using a particular vcs's information
fn build_from_vcs(
    project_path: &std::path::Path, 
    vcs_url: &Option<String>,
    flavors: &str, 
    vcs: &VcsSystem, 
    platforms: &str,
    build_server: &BuildServer, 
    name: &str, 
    version: &str,
    dry_run: bool,
    verbose: bool,
    prompt:bool,
) ->  Result<(), RemoteBuildError>  {

    debug!("invoking request_build_for based on vcs system");
    match vcs {
        VcsSystem::Svn => {
            debug!("Svn system...");
            //let vcs_project_url = Svn::get_url(&project_path, minifest.version.as_str())?;
            let vcs_project_url = vcs_url.as_ref().map(|u|{
                debug!("parsing Url");
                url::Url::parse(&u).unwrap_or_else(|_e| {
                    error!("unable to construct url from path provided");
                    std::process::exit(1);
                })
            
            }).unwrap_or_else(|| {
                let url = Svn::get_server_urls(&project_path).unwrap_or_else(|_|{
                    error!("Unable to get svn server url from project path");
                    std::process::exit(1);
                });
                url[0].clone()
            });

            debug!("vcs_project_url: {:?}", &vcs_project_url);
            debug!("calling request_build_for");
            request_build_for(
                &build_server,
                &name,
                &version,
                &vcs_project_url,
                &vcs,
                &platforms,
                &flavors,
                dry_run,
                verbose,
                prompt,
            )
        }
        VcsSystem::Git => {
            debug!("Git system...");
            let vcs_project_url = vcs_url.as_ref().map(|u|{
                debug!("parsing url");
                url::Url::parse(&u).unwrap_or_else(|e| {
                    error!("unable to construct url from {}. error {}", u, e);
                    std::process::exit(1);
                })
            
            }).unwrap_or_else(|| {
                debug!("vcs_url empty. Retrieving url from Git::get_server_urls({:?})", &project_path);
                let url = Git::get_server_urls(&project_path).unwrap_or_else(|_|{
                    error!("Unable to get git server url from project path");
                    std::process::exit(1);
                });
                assert!(url.len() > 0);
                url[0].clone()
            });
            debug!("vcs_project_url: {:?}", &vcs_project_url);
            request_build_for(
                &build_server,
                &name,
                &version,
                &vcs_project_url,
                &vcs,
                &platforms,
                &flavors,
                dry_run,
                verbose,
                prompt,
            )
        }
        _ => {
            error!("SCM must either be svn or git");
            std::process::exit(1);
        }
    }
}


// Trigger a build on the given build server, with the project identified
// using the supplied parameters. Of course, if dry_run is true, then simply
// pretend to do a build.
fn request_build_for(
    build_server: &BuildServer,
    name: &str,
    version: &str,
    vcs_project_url: &url::Url,
    vcs: &VcsSystem,
    platforms: &str,
    flavors: &str,
    dry_run: bool,
    verbose: bool,
    prompt: bool,
) -> Result<(), RemoteBuildError> {
    let platforms = Platform::parse_platforms(platforms);
    let flavors = Flavours::parse_flavors(flavors);

    debug!("{:?}", vcs_project_url);

    if dry_run || verbose || prompt {
        let platform_str: Vec<String> = platforms.iter().map(|x| x.to_string()).collect();
        let platform_str = platform_str.join(" , ");

        let mut table = table!(
            [FYbH2c -> "Remote Build Request Information"],
            [FYb -> "Route",     Fwb -> build_server.request_route().ok_or(RemoteBuildError::EmptyError("unable to unwrap request_route".into()))?],
            [FYb -> "Project",   Fwb ->  name],
            [FYb -> "VCS Tag",   Fwb -> version],
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
            name,
            version,
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