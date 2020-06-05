use log::{debug, error};
use pkg_build_remote::{
    RemoteBuildError,
    from_gpi, cli::Opt,
    from_manifest,
};
use pretty_env_logger;

use structopt::StructOpt;


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
    
    let result = if opts.use_local_project {
        from_manifest::request::do_local(opts)
    } else {
        from_gpi::request::do_gpi(opts)
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
