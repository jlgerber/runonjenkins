use log::{debug, error};
use pkg_build_remote::{
    RemoteBuildError,
    from_gpi, cli::Opt,
};
use pretty_env_logger;

use structopt::StructOpt;
use std::env;

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
    let opts = Opt::from_args();
    if opts.verbose {
        env::set_var("RUST_LOG","debug");
    }
    pretty_env_logger::init();
    debug!("Initialized");
    
    let result = from_gpi::request::do_gpi(opts);

    match result {
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
        _ => (),
    };
    Ok(())
}
