//extern crate pkg_build_remote;
use pkg_build_remote::{ get_flavors, Svn, Git, get_minifest};
//use shellfn::shell;
use std::env;

fn main() -> Result<(), failure::Error>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "svn" {
            let remotes = Svn::get_url("1.2.3");
            println!("{:?}", remotes);
        } else if args[1] == "git" {
            let cwd = env::current_dir()?;
            let remotes = Git::get_remotes(cwd.to_str().unwrap());
            println!("{:?}", remotes);
        } else {
            println!("choose svn or git");
        }
    } else {
        println!("usage: pkg_build_remote svn|git")
    }
    /*
    let flavors = get_flavors()?;
    println!("flavors: {:?}", flavors);
    let mini = get_minifest()?;
    println!("{:?}", mini);
    */
    Ok(())
}

