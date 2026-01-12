use anyhow::{Context, Result, anyhow};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[cfg(target_os = "windows")]
pub fn strip_extension(name: &str) -> String {
    println!("stripping extension from {name}");
    Some(name)
        .and_then(|n| Path::new(name).file_stem())
        .and_then(|n| n.to_str())
        .and_then(|s| Some(String::from(s)))
        .unwrap_or_else(|| String::from(name))
}

#[derive(PartialEq)]
enum Kind {
    File,
    Dir,
}

pub fn count_components(p: &Path) -> u32 {
    p.components().fold(0u32, |acc, _| acc + 1)
}

pub fn cur_dir() -> Result<PathBuf> {
    std::env::current_dir().context("Finding the current directory")
}

fn find_thing(dir: &Path, name: &str, kind: Kind) -> Result<PathBuf> {
    let mut cur = dir;

    loop {
        let candidate = cur.join(name);

        match fs::metadata(&candidate) {
            Ok(metadata) => {
                if (kind == Kind::File && metadata.is_file())
                    || (kind == Kind::Dir && metadata.is_dir())
                {
                    return Ok(candidate);
                }
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    return Err(e.into());
                }
            }
        }

        if let Some(parent) = cur.parent() {
            if cur == parent {
                return Err(std::io::Error::new(
                    io::ErrorKind::NotFound,
                    "Reached the root without finding what you want",
                )
                .into());
            }
            cur = parent;
        } else {
            return Err(std::io::Error::new(
                io::ErrorKind::NotFound,
                "Reached the root without finding what you want",
            )
            .into());
        }
    }
}

pub fn find(dir: &Path, name: &str) -> Result<PathBuf> {
    find_thing(dir, name, Kind::File)
}

pub fn find_dir(dir: &Path, name: &str) -> Result<PathBuf> {
    find_thing(dir, name, Kind::Dir)
}

pub fn find_root_of_repo() -> Result<PathBuf> {
    let here = cur_dir()?;
    // look for the .git directory
    match find_dir(&here, ".git") {
        Ok(d) => match d.parent() {
            Some(x) => Ok(x.to_path_buf()),
            None => Err(anyhow!("Failed to get a path with a parent ({d:?})")),
        },
        Err(e) => Err(e),
    }
}

/*
fn main() {
            match find_root(vec_args) {
                Ok(d) => {
                    println!("{}", d.display());
                    Ok(0)
                }
                _ => Ok(-1),
            }
}

fn find_root(_vec_args: &[String]) -> Result<std::path::PathBuf> {
    find_root_of_repo()
}

*/
