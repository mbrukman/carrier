extern crate carrier;
extern crate clap;
extern crate pbr;
extern crate prost;

use pbr::ProgressBar;
use std::env;

mod cli;

pub fn main() -> Result<(), std::io::Error>{
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::Builder::from_default_env().default_format_timestamp(false).init();


    let matches = cli::build_cli()
        .version(carrier::BUILD_ID)
        .get_matches();

    match matches.subcommand() {
        ("identity", Some(_submatches)) => {
            let mut e   = carrier::Error::new(carrier::ERR_TAIL);
            let mut ik  = carrier::identity_kit::IdentityKit::new();

            unsafe{ carrier::identity_kit::carriertoml(ik._self(), e._self(), e._tail()); };

            e.check()?;
            Ok(())
        }
        _ => unreachable!(),
    }
}
