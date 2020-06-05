
use log::debug;
use crate::{
    BuildServer, 
    RemoteBuildError,
    packalaka_tags::PackageTagList, 
    utils::request_build_for,
    cli::Opt
};


// set up and execute the build using information gleaned from the gpi
pub fn do_gpi(opts: Opt) ->  Result<(), RemoteBuildError> {
    
    let name = opts.name.ok_or(RemoteBuildError::EmptyError("Missing name. Must be supplied".into()))?;
    let tag = opts.tag.ok_or(RemoteBuildError::EmptyError("Missing tag. Must be supplied".into()))?;


    let tags = PackageTagList::from_service(&name, &tag)?;
    debug!("PackageTag {:#?}", tags);

    if tags.len() == 0 {
        return Err(RemoteBuildError::EmptyError(format!("No Records exist for {}-{}", &name, &tag)));
    }

    let build_server = BuildServer::default();

    let distribution = tags.get(0).unwrap(); // already testing that tags.len() > 0 above
    
    // if the user supplies flavors either via the flavor or flavour flag, go ahead and 
    // use them. Otherwise, pull them from the gpi
    let flavors = if opts.flavors.is_some() {
        opts.flavors.unwrap()
    } else if opts.flavours.is_some() {
        opts.flavours.unwrap()
    } else {
       tags.get(0).unwrap().flavors().join(",")
    };
    debug!("flavors selected: {}", flavors);
    debug!("request_build_for(...)");
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
