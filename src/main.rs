extern crate nd_lib;

use nd_lib::package::Package;

fn main() {
    let pkg = Package::load("../nd_lib/fixtures/3-dep-not-installed");
    for issue in pkg.validate() {
        println!("{:?}", issue);
    }
}
