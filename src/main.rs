use pkg_build_remote::{ get_flavors, Svn, Git, Minifest, BuildRequest, Platform, VcsSystem};
use std::env;
use structopt::StructOpt;
use std::env::current_dir;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Vcs choice git | svn
    #[structopt(short = "v", long = "vcs")]
    vcs: Option<String>,

    #[structopt(short = "f", long = "flavours", default_value="^")]
    flavors: String,

    /// Set dry run
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,

}

// am i in a git repo
fn is_git() -> bool{
    let mut cwd = current_dir().unwrap();
    cwd.push(".git");
    cwd.exists()
}

fn is_svn() -> bool {
    let mut cwd = current_dir().unwrap();
    cwd.push(".svn");
    cwd.exists()
}


fn identify_vcs(selection: &Option<String>) -> Option<VcsSystem> {
    match selection {
        Some(val) => {
            return Some(VcsSystem::from(val.as_str()))
        }
        None => {
            if is_git() {
                return Some(VcsSystem::from("git"));
            }
            if is_svn() {
                return Some(VcsSystem::from("svn"))
            }
        }
    }
    None
}

fn parse_flavors(flavor: &str) -> Vec<&str> {
    flavor.split(",").collect::<Vec<&str>>()
}

/*
pub fn new<'a,T,P>(
        project: T,
        version: T,
        flavor:  T,
        repo:   &'a str,
        scm_type: impl Into<VcsSystem>,
        platform: P
    )-> Result<Self, ParseError>
*/

fn build_requests(minifest: &Minifest, repo: &str, scm_type: &VcsSystem, platform: &Platform, flavors: &Vec<&str>) ->Vec<BuildRequest> {
    let mut build_reqs = Vec::with_capacity(flavors.len());
    for flav in flavors{
        build_reqs.push(BuildRequest::new(minifest.name.as_str(), minifest.version.as_str(), flav, repo, scm_type, platform).unwrap());
    }

    build_reqs
}

fn main() -> Result<(), failure::Error>{
    let opts = Opt::from_args();

    let vcs = identify_vcs(&opts.vcs);

    if vcs.is_none() {
        println!("Error: No VCS system idemtified");
        std::process::exit(1);
    }
    let vcs = vcs.unwrap();

    let flavors = parse_flavors(&opts.flavors);

    // get minifest
    let minifest = Minifest::from_disk()?;
    println!("{:?}", minifest);
    match vcs {
        VcsSystem::Svn => {
            let remotes = Svn::get_url(minifest.version.as_str());
            println!("{:?}", remotes);
        },
         VcsSystem::Git => {
            let cwd = env::current_dir()?;
            let remotes = Git::get_remotes(cwd.to_str().unwrap())?;
            println!("{:?}", remotes);
            let build_reqs = build_requests(&minifest, remotes[0].as_str(), &VcsSystem::Git, &Platform::Cent7,&flavors);
            for br in build_reqs {
                println!("{:?}", br);
            }

        }
        _ => {
            println!("choose svn or git");
            std::process::exit(1);
        }
    }


    Ok(())
}

