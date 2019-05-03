use pkg_build_remote::{ get_flavors, Svn, Git, Minifest, VcsSystem};
use std::env;
use structopt::StructOpt;
use std::path::Path;
use std::env::current_dir;

#[derive(StructOpt, Debug)]
#[structopt(name = "pkg-build-remote")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Vcs choice git | svn
    #[structopt(short = "v", long = "vcs")]
    vcs: Option<String>,

    /// Set speed
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


fn main() -> Result<(), failure::Error>{
    let mut opts = Opt::from_args();

    let vcs = identify_vcs(&opts.vcs);

    if vcs.is_none() {
        println!("Error: No VCS system idemtified");
        std::process::exit(1);
    }
    let vcs = vcs.unwrap();

    // get minifest
    let minifest = Minifest::from_disk()?;
    println!("{:?}", minifest);
    match vcs {

        VcsSystem::Svn => {
            // identify version and name ursl
            // minifest

            let remotes = Svn::get_url("1.2.3");
            println!("{:?}", remotes);
        },
         VcsSystem::Git => {
            let cwd = env::current_dir()?;
            let remotes = Git::get_remotes(cwd.to_str().unwrap());
            println!("{:?}", remotes);
        }
        _ => {
            println!("choose svn or git");
            std::process::exit(1);
        }
    }




    /*
    let flavors = get_flavors()?;
    println!("flavors: {:?}", flavors);
    let mini = get_minifest()?;
    println!("{:?}", mini);
    */
    Ok(())
}

