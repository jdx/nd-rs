#[macro_use]
extern crate clap;
extern crate nd_lib;

use clap::{App, Arg, SubCommand};
use std::process::{exit, Command};

use nd_lib::package::Package;

fn exec(args: Vec<&str>) {
    let pkg = Package::load("../nd_lib/fixtures/3-dep-not-installed");
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
    let mut app = App::new("nd")
        .version(crate_version!())
        // .author(crate_authors!())
        .about("run/manage node.js projects")
        .subcommand(
            SubCommand::with_name("exec")
                .aliases(&["x"])
                .about("run node script directly (equivalent to running node directly)")
                .arg(Arg::with_name("args")
                     .multiple(true))
        );
    let matches = app.clone().get_matches();
    // println!("{:?}", matches);
    match matches.subcommand_name() {
        Some("exec") => {
            let matches = matches.subcommand_matches("exec").unwrap();
            exec(matches.values_of("args").unwrap().collect());
        }
        None => {
            app.print_long_help().unwrap();
            println!("");
        }
        cmd => panic!("invalid command {:?}", cmd),
    }
}
