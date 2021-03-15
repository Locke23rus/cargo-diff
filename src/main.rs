extern crate clap;
extern crate reqwest;

use clap::{crate_version, App, Arg};
use reqwest::header::CONTENT_LENGTH;
use std::io::{self, Read};
use std::path::PathBuf;
use std::{env, error::Error};
use std::{fs, process::exit};

fn main() {
    let matches = App::new("cargo-diff")
        .version(&crate_version!()[..])
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
    let crate_dir1 = download_and_extract_crate(
        temp_dir.clone(),
        package_name.to_string(),
        version1.to_string(),
    )
    .unwrap();

    let crate_dir2 = download_and_extract_crate(
        temp_dir.clone(),
        package_name.to_string(),
        version2.to_string(),
    )
    .unwrap();

    println!("{}", crate_dir1.display());
    println!("{}", crate_dir2.display());
}

fn create_temp_dir() -> io::Result<PathBuf> {
    let dir = env::temp_dir().join("qwe");
    // FIXME: generate unique name
    fs::create_dir_all(dir.clone())?;
    println!("Created temp_dir: {}", dir.display());
    Ok(dir)
}

fn download_and_extract_crate(
    dir: PathBuf,
    name: String,
    version: String,
) -> Result<PathBuf, String> {
    let crate_bytes = download_crate(name.clone(), version.clone()).unwrap_or_else(|e| {
        println!(
            "Failed to download crate `{}={}`: {}",
            name.clone(),
            version.clone(),
            e
        );
        exit(1);
    });

    println!("Extracting crate archive to {}/", dir.display());
    let gzip = flate2::read::GzDecoder::new(&crate_bytes[..]);
    let mut archive = tar::Archive::new(gzip);
    match archive.unpack(dir.clone()) {
        Ok(_) => {
            println!("Crate content extracted to {}/", dir.display());
        }
        Err(e) => {
            println!("Couldn't extract crate to {}/: {}", dir.display(), e);
            exit(1)
        }
    }

    let crate_dir = dir.join(format!("{}-{}", name, version));
    Ok(crate_dir)
}

/// Download given crate and return it as a vector of gzipped bytes.
fn download_crate(name: String, version: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let download_url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        name, version
    );
    println!(
        "Downloading crate `{}={}` from {}",
        name, version, download_url
    );
    let mut response = reqwest::blocking::get(&download_url)?;

    let content_length: Option<usize> = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok());
    println!(
        "Download size: {}",
        content_length.map_or("<unknown>".into(), |cl| format!("{} bytes", cl))
    );
    let mut bytes = match content_length {
        Some(cl) => Vec::with_capacity(cl),
        None => Vec::new(),
    };
    response.read_to_end(&mut bytes)?;

    println!("Crate `{}={}` downloaded successfully", name, version);
    Ok(bytes)
}
