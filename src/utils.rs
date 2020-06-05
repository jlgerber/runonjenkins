
use log::{debug};
use crate::{
   BuildRequest, BuildServer, Minifest, RemoteBuildError,
    VcsSystem, Platform, Flavours
};
use prettytable::{cell, format, row, table};
use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

// get the minifest from the path, unless both the name and tag are passed in as Some. Then
// in that case, build the minifest out of them
pub fn get_minifest(
    project_path: &Path,
    name: &Option<String>,
    tag: &Option<String>,
) -> Result<Minifest, failure::Error> {
    if name.is_some() && tag.is_some() {
        let name = name.as_ref().unwrap();
        let tag = tag.as_ref().unwrap();
        Ok(Minifest::new(name.clone(), tag.clone()))
    } else {
        Minifest::from_disk(Some(&project_path))
    }
}


// Trigger a build on the given build server, with the project identified
// using the supplied parameters. Of course, if dry_run is true, then simply
// pretend to do a build.
pub fn request_build_for(
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