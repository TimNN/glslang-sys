#![feature(path_ext)]

use std::{env, fs};
use std::fs::PathExt;
use std::path::{Path, PathBuf};

extern crate gcc;


#[cfg(unix)]
const OS_DEPENDENT: &'static [&'static str] = &["glslang", "OSDependent", "Linux"];
#[cfg(win)]
const OS_DEPENDENT: &'static [&'static str] = &["glslang", "OSDependent", "Windows"];

const SEARCH_DIRS: &'static [&'static [&'static str]] = &[
    &["glslang"],
    &["glslang", "gen"],
    &["glslang", "GenericCodeGen"],
    &["glslang", "Include"],
    &["glslang", "MachineIndependent"],
    &["glslang", "MachineIndependent", "preprocessor"],
    &["glslang", "Public"],
    OS_DEPENDENT,
    &["OGLCompilersDLL"],
];

fn main() {
    let src_root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("extern").join("glslang");


    let mut config = gcc::Config::new();
    config.cpp(true);

    for file in find_sources(&src_root) {
        config.file(file);
    }

    for dir in resolve_search_dirs(&src_root) {
        config.include(dir);
    }

    config.compile("libglslang.a");
}

fn find_sources<P: AsRef<Path>>(root: &P) -> Vec<PathBuf> {
    let mut sources = Vec::new();

    for search_dir in resolve_search_dirs(root) {

        for file in fs::read_dir(search_dir).unwrap().filter_map(|e| {
            let path = e.unwrap().path();

            let mut ok = false;

            if let Some(ext) = path.extension() {
                if path.is_file() && ext == "cpp" {
                    ok = true;
                }
            }

            if ok { Some(path) } else { None }
        }) {
            sources.push(file)
        }
    }

    sources
}

fn resolve_search_dirs<P: AsRef<Path>>(root: &P) -> Vec<PathBuf> {
    SEARCH_DIRS.iter().map(|dir| {
        let mut buf = root.as_ref().to_owned();

        for component in *dir {
            buf.push(component)
        }

        buf
    }).collect()
}
