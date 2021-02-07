extern crate clap;
use clap::{App, Arg};
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let matches = App::new("cargo-diff")
        .version("v0.0.1")
        .arg(
            Arg::with_name("PACKAGE_NAME")
                .help("Sets package name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("VERSION_1")
                .help("Sets version1")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("VERSION_2")
                .help("Sets version2")
                .required(true)
                .index(3),
        )
        .get_matches();

    let package_name = matches.value_of("PACKAGE_NAME").unwrap();
    let version1 = matches.value_of("VERSION_1").unwrap();
    let version2 = matches.value_of("VERSION_2").unwrap();

    let temp_dir = create_temp_dir().unwrap();
    let crate1 = download_crate(temp_dir.clone(), package_name, version1).unwrap();
    let crate2 = download_crate(temp_dir.clone(), package_name, version2).unwrap();

    println!("{}", crate1.display());
    println!("{}", crate2.display());
}

fn create_temp_dir() -> io::Result<PathBuf> {
    let mut dir = env::temp_dir();
    // FIXME: generate unique name
    dir.push("qwe");
    fs::create_dir_all(dir.clone())?;
    println!("Created temp_dir: {}", dir.display());
    Ok(dir)
}

fn download_crate(dir: PathBuf, name: &str, version: &str) -> Result<PathBuf, String> {
    let mut crate_path = dir.clone();
    crate_path.push(format!("{}-{}.crate", name, version));

    let url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        name, version
    );
    println!("{}", url);
    Ok(crate_path)
}
