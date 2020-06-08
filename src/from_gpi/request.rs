
use log::{debug,info};
use crate::{
    BuildServer, 
    RemoteBuildError,
    packalaka_tags::PackageTagList, 
    utils::request_build_for,
    utils::request_package_build_for,
    cli::Opt,
    constants::DEFAULT_PLATFORM
};


// set up and execute the build using information gleaned from the gpi
pub fn do_gpi(opts: Opt) ->  Result<(), RemoteBuildError> {
    
   

    let build_server = BuildServer::default();

   
    
    if opts.flavours.is_none() && opts.flavors.is_none() && opts.platforms.is_none() {
        info!("using package build route");
        request_package_build_for(
            &build_server,
            &opts.name,
            &opts.tag,
            opts.dry_run,
            opts.verbose,
            opts.prompt
        )
    } else {
        info!("using per-tag build route");
        let tags = PackageTagList::from_service(&opts.name, &opts.tag)?;
        debug!("PackageTag {:#?}", tags);

        if tags.len() == 0 {
            return Err(RemoteBuildError::EmptyError(format!("No Records exist for {}-{}", &opts.name, &opts.tag)));
        }
        
        let distribution = tags.get(0).unwrap(); // already testing that tags.len() > 0 above
        
        let platforms = opts.platforms.unwrap_or(DEFAULT_PLATFORM.to_string());
        // if the user supplies flavors either via the flavor or flavour flag, go ahead and 
        // use them. Otherwise, pull them from the gpi
        let flavors = if opts.flavors.is_some() {
            opts.flavors.unwrap()
        } else if opts.flavours.is_some() {
            opts.flavours.unwrap()
        } else {
            tags.get(0).unwrap().flavors().join(",")
        };
        
        debug!("platforms selected: {}", &platforms);
        debug!("flavors selected:   {}", &flavors);
        debug!("request_build_for(...)");

        request_build_for(
            &build_server,
            &opts.name,
            &opts.tag,
            &distribution.link()?,
            &distribution.uses, //vcs
            &platforms,
            &flavors,
            opts.dry_run,
            opts.verbose,
            opts.prompt,
        )
    }
}
