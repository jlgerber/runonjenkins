
use log::{debug, error};
use crate::{
    traits::*, BuildServer, Git, Minifest, RemoteBuildError,
    Svn, VcsSystem, utils, Flavors, utils::request_build_for,
     cli::Opt
};
use std::env;

/// set up the build using local information gleaned from the manifest and the local vcs repo
pub fn do_local( opts: Opt) ->  Result<(), RemoteBuildError> {

    debug!("retrieving project path");
    let project_path = opts.project_path.unwrap_or(env::current_dir()?);
    debug!("project_path: {:?}", project_path);

    debug!("retrieving flavors");
    let flavors = Flavors::resolve_flavors(opts.flavors, opts.flavours, Some(&project_path));
   
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



/// execute a build using a particular vcs's information
pub fn build_from_vcs(
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
