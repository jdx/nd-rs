extern crate nd_lib;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

use clap::{App, Arg, SubCommand};
use std::process::{exit, Command};

use nd_lib::package::Package;

fn exec(root: &str, args: Vec<&str>) {
    let pkg = Package::load(root);
    for issue in pkg.validate() {
        println!("{:?}", issue);
    }
    let status = Command::new("node")
        .args(args)
        .status()
        .expect("failed to execute node");
    exit(status.code().unwrap());
}

fn main() {
    env_logger::init();
    let mut app = App::new("nd")
        .version(crate_version!())
        // .author(crate_authors!())
        .about("run/manage node.js projects")
        .subcommand(
            SubCommand::with_name("exec")
                .aliases(&["x"])
                .about("run node script directly (equivalent to running node directly)")
                .arg(Arg::with_name("args")
                     .required(true)
                     .multiple(true))
        )
        .arg(Arg::with_name("root")
             .short("r")
             .long("root")
             .value_name("DIR")
             .help("Set project root")
             .takes_value(true)
        );
    let matches = app.clone().get_matches();
    let root = matches.value_of("root").unwrap_or("");
    info!("{:?}", matches);
    match matches.subcommand_name() {
        Some("exec") => {
            let matches = matches.subcommand_matches("exec").unwrap();
            exec(root, matches.values_of("args").unwrap().collect());
        }
        None => {
            app.print_long_help().unwrap();
            println!("");
        }
        cmd => panic!("invalid command {:?}", cmd),
    }
}
