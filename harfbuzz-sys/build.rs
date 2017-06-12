extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), Error> {
    let package_name = "harfbuzz";
    let shared_libs = ["harfbuzz", "harfbuzz-gobject"];
    let version = if cfg!(feature = "v1_4_3") {
        "1.4.3"
    } else if cfg!(feature = "v1_4_2") {
        "1.4.2"
    } else if cfg!(feature = "v1_3_3") {
        "1.3.3"
    } else if cfg!(feature = "v1_2_3") {
        "1.2.3"
    } else if cfg!(feature = "v1_1_3") {
        "1.1.3"
    } else if cfg!(feature = "v1_1_2") {
        "1.1.2"
    } else if cfg!(feature = "v1_0_5") {
        "1.0.5"
    } else if cfg!(feature = "v0_9_42") {
        "0.9.42"
    } else if cfg!(feature = "v0_9_41") {
        "0.9.41"
    } else if cfg!(feature = "v0_9_39") {
        "0.9.39"
    } else if cfg!(feature = "v0_9_38") {
        "0.9.38"
    } else if cfg!(feature = "v0_9_31") {
        "0.9.31"
    } else if cfg!(feature = "v0_9_30") {
        "0.9.30"
    } else if cfg!(feature = "v0_9_28") {
        "0.9.28"
    } else if cfg!(feature = "v0_9_22") {
        "0.9.22"
    } else if cfg!(feature = "v0_9_20") {
        "0.9.20"
    } else if cfg!(feature = "v0_9_11") {
        "0.9.11"
    } else if cfg!(feature = "v0_9_10") {
        "0.9.10"
    } else if cfg!(feature = "v0_9_8") {
        "0.9.8"
    } else if cfg!(feature = "v0_9_7") {
        "0.9.7"
    } else if cfg!(feature = "v0_9_5") {
        "0.9.5"
    } else {
        "0.0"
    };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(())
    }

    let target = env::var("TARGET").expect("TARGET environment variable doesn't exist");
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }
    match config.probe(package_name) {
        Ok(library) => {
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!("cargo:rustc-link-search=native={}",
                             path.to_str().expect("library path doesn't exist"));
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

