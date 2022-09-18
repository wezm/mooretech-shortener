extern crate core;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs, io};

use url::Url;

#[path = "src/rewrite_map.rs"]
mod rewrite_map;

fn main() {
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("map.rs");
    let mappings = fs::read_to_string("mappings.txt").expect("unable to read mappings.txt");
    let mappings = rewrite_map::parse(&mappings).expect("error parsing mappings");
    write_output(&path, &mappings).expect("error writing output file");
}

fn write_output(path: &Path, mappings: &[(&str, Url)]) -> Result<(), io::Error> {
    let mut f = File::create(&path).expect("unable to create output file");
    writeln!(f, "pub const MAPPINGS: &[(&str, &str)] = &[")?;

    for (key, val) in mappings.iter() {
        writeln!(f, r#"    ("{}", "{}"),"#, key, val)?;
    }

    writeln!(f, "];")
}
