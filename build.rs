extern crate pkg_config;

fn main() {
    pkg_config::find_library("libftdi1").unwrap();
}