use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

pub fn find_cc() -> Option<PathBuf> {
    if let Some(x) = std::env::var_os("CC") {
        let buf = PathBuf::from(x);
        if buf.is_absolute() {
            Some(buf)
        } else {
            which::which(buf).ok()
        }
    } else {
        let host = std::env::var("HOST").ok();
        let target = std::env::var("TARGET").ok();
        let cc_name = if target.is_some() && target != host {
            let mut target = target.clone().unwrap();
            target += "-cc";
            PathBuf::from(target)
        } else {
            PathBuf::from("cc")
        };
        let gcc_name = if target.is_some() && target != host {
            let mut target = target.clone().unwrap();
            target += "-gcc";
            PathBuf::from(target)
        } else {
            PathBuf::from("gcc")
        };

        let names = vec![
            cc_name,
            PathBuf::from("clang"),
            PathBuf::from("lccc"),
            gcc_name,
        ];
        // Support only gcc-like CLIs for now
        // if let Some(target) = target {
        //     if target.ends_with("msvc") {
        //         names.push(PathBuf::from("cl"));
        //     }
        // }

        for n in names {
            if let Ok(name) = which::which(n) {
                return Some(name);
            }
        }
        None
    }
}

pub fn prepare_cc() -> Option<Command> {
    let path = find_cc()?;
    let mut cmd = Command::new(&path);
    add_cc_target(&mut cmd, &path);

    Some(cmd)
}

pub fn add_cc_target<'a>(x: &'a mut Command, name: &Path) -> &'a mut Command {
    let target = std::env::var_os("TARGET");
    let host = std::env::var_os("HOST");

    if target.is_some() && target != host {
        let target = target.unwrap();

        if !name
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with(&*target.to_string_lossy())
        {
            x.args([OsStr::new("--target"), &target]);
        }
    }

    x
}
