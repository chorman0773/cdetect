mod get_cc;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub use get_cc::find_cc;
pub use get_cc::prepare_cc;

pub fn try_compile<I: IntoIterator<Item = String>>(tag: &str, x: I) -> Option<String> {
    try_compile_in(
        tag,
        x,
        std::env::current_dir().ok()?,
        Path::new(&std::env::var_os("OUT_DIR")?),
    )
}

pub fn try_compile_in<I: IntoIterator<Item = String>, S: AsRef<Path>, B: AsRef<Path>>(
    tag: &str,
    x: I,
    src_dir: S,
    bin_dir: B,
) -> Option<String> {
    try_compile_in_with(tag, x, src_dir, bin_dir, |x| x)
}

pub fn try_compile_in_with<
    I: IntoIterator<Item = String>,
    S: AsRef<Path>,
    B: AsRef<Path>,
    F: FnMut(&mut Command) -> &mut Command,
>(
    tag: &str,
    x: I,
    src_dir: S,
    bin_dir: B,
    mut withfn: F,
) -> Option<String> {
    for mut s in x {
        let stem = s.clone();
        s += tag;
        let mut src_path = PathBuf::from(src_dir.as_ref());
        src_path.push(&s);
        src_path.set_extension(".c");
        let mut obj_path = PathBuf::from(bin_dir.as_ref());
        obj_path.push(&s);
        let mut cmd = prepare_cc()?;
        let cflags = std::env::var("CFLAGS").ok();
        let cflags = cflags
            .iter()
            .flat_map(|x| x.split(|c: char| c.is_whitespace()));
        let ldflags = std::env::var("LDFLAGS").ok();
        let ldflags = ldflags
            .iter()
            .flat_map(|x| x.split(|c: char| c.is_whitespace()));
        match (withfn)(&mut cmd)
            .arg(src_path)
            .arg("-o")
            .arg(obj_path)
            .args(cflags)
            .args(ldflags)
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()
            .ok()
        {
            Some(s) if s.success() => return Some(stem),
            _ => (),
        }
    }

    None
}
