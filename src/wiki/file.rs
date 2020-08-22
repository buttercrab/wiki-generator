use crate::util::path;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub struct File {
    pub(crate) from: PathBuf,
    pub(crate) to: PathBuf,
}

static IMG_LIST: &[&str] = &[
    "apng", "bmp", "gif", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "svg", "tif",
    "tiff", "webp",
];

static EXCLUDE: &[&str] = &["md"];

impl File {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
        from: P,
        src: Q,
        out: R,
        preserve: &Option<Vec<String>>,
    ) -> File {
        let from = from.as_ref();
        let out = out.as_ref();

        let (is_img, ext) = match from.extension() {
            Some(ext) => {
                let ext = path::os_to_str(ext);
                (IMG_LIST.contains(&&*ext.to_ascii_lowercase()), ext)
            }
            None => (false, "".to_string()),
        };

        debug_assert!(!EXCLUDE.contains(&&*ext));

        let preserve = match preserve {
            Some(preserve) => preserve.contains(
                &path::path_to_str(from.strip_prefix(&src).unwrap()).to_ascii_lowercase(),
            ),
            None => false,
        };

        let to = if preserve {
            Path::new(out).join(from.strip_prefix(src).unwrap())
        } else {
            let mut hasher = Sha256::new();
            hasher.update(fs::read(from).unwrap());
            let res = &hasher.finalize()[..];
            let mut hash = String::new();
            for i in 0..8 {
                hash.push_str(&*format!("{:02x}", res[i]));
            }

            Path::new(out)
                .join(if is_img { "r/img" } else { "r/file" })
                .join(format!(
                    "{name}-{hash}",
                    name = path::os_to_str(from.file_stem().unwrap()),
                    hash = hash
                ))
                .with_extension(ext)
        };

        File {
            from: from.clone().to_path_buf(),
            to,
        }
    }

    pub fn copy(&self) {
        path::make_dir_above(&self.to);
        fs::copy(&self.from, &self.to).expect(&*format!("failed to move {:?}", self.from));
    }
}
