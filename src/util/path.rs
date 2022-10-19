use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_files_all<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    WalkDir::new(path.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|i| i.metadata().unwrap().is_file())
        .map(|i| i.path().to_path_buf())
        .collect::<Vec<_>>()
}

pub fn make_dir_above<P: AsRef<Path>>(path: P) {
    fs::create_dir_all(path.as_ref().parent().unwrap())
        .unwrap_or_else(|_| panic!("make dir above {:?} failed", path.as_ref()));
}

pub fn os_to_str<O: AsRef<OsStr>>(os: O) -> String {
    os.as_ref().to_os_string().into_string().unwrap()
}

pub fn path_to_str<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .as_os_str()
        .to_os_string()
        .into_string()
        .unwrap()
}

pub fn simplify<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut res = PathBuf::new();

    for i in path.iter() {
        res = if i == OsStr::new("..") {
            match res.parent() {
                Some(p) => p.to_path_buf(),
                None => res,
            }
        } else if i != OsStr::new(".") {
            res.join(i)
        } else {
            res
        }
    }

    res
}
