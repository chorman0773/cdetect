use std::{
    ffi::OsString,
    fs::File,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use target_tuples::Target;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
/// Describes the CLI of the C (or C++) compiler, if known
pub enum CCFlavour {
    /// The CLI of the C compiler isn't known, or could not be detected
    Unknown,
    /// The CLI of the C compiler matches POSIX cc (such as gcc or clang)
    Cc,
    /// The CLI of the C compiler matches Microsoft cl.exe
    Cl,
}

impl Default for CCFlavour {
    fn default() -> CCFlavour {
        CCFlavour::Unknown
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
/// Represents Versions of the C (or C++) standards, support by a C compiler
pub enum CStandard {
    /// ANSI C (C89)
    C89,
    /// ISO C 95
    C95,
    /// ISO C 99
    C99,
    /// ISO C 11
    C11,
    /// ISO C 18
    C18,
    /// ISO C 2x
    C2x,
    /// ISO C++98
    Cxx98,
    /// ISO C++03
    Cxx03,
    /// ISO C++11
    Cxx11,
    /// ISO C++14
    Cxx14,
    /// ISO C++17
    Cxx17,
    /// ISO C++20
    Cxx20,
    /// ISO C++23
    Cxx2x,
    /// GNU C 89
    Gnu89,
    /// GNU C 95
    Gnu95,
    /// GNU C 99
    Gnu99,
    /// GNU C 11
    Gnu11,
    /// GNU C 18
    Gnu18,
    /// GNU C 2x
    Gnu2x,
    /// GNU C++ 98
    Gxx98,
    /// GNU C++ 03
    Gxx03,
    /// GNU C++ 11
    Gxx11,
    /// GNU C++ 14
    Gxx14,
    /// GNU C++ 17
    Gxx17,
    /// GNU C++ 20
    Gxx20,
    /// GNU C++ 23
    Gxx2x,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
/// Structure of properties for the C (or C++) compiler.
/// The type is non-exhausitve, as more properties may be added in the future. To construct, add `..Default::default()` after each field that you wish to initialize.
pub struct CCompilerProperties {
    /// Absolute path to the Found C (or C++) compiler
    pub path: PathBuf,
    /// Options which should immediately follow the path when invoking the compiler
    pub extra_opts: Vec<OsString>,
    /// Flags that should be passed to the C Compiler for the compile step
    pub compile_flags: Vec<OsString>,
    /// Flags that should be passed to the C compiler for the link step, or an empty list of the C compiler is not used for linking
    pub ld_flags: Vec<OsString>,
    /// The exact target string that is compiled for, or None if it is unknown
    pub target: Option<Target>,
    /// The flavour of the C compiler
    pub flavour: CCFlavour,

    /// Standards Supported by the C Compiler
    pub standard: Vec<CStandard>,

    #[doc(hidden)]
    pub __nonexhaustive: (),
}

/// Low-level function to populate the properties of a C compiler
pub fn populate_properties(
    properties: &mut CCompilerProperties,
    tmpdir: &Path,
) -> std::io::Result<()> {
    let cc = &properties.path;
    properties.flavour = CCFlavour::Cc; // Assume it's cc for now
    {
        let mut test_file = tmpdir.to_owned();
        test_file.push("test.c");
        let mut file = loop {
            match File::options()
                .create_new(true)
                .write(true)
                .read(true)
                .open(&test_file)
            {
                Ok(f) => break f,
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                    std::thread::sleep(Duration::from_millis(1))
                }
                Err(e) => return Err(e),
            }
        };
        writeln!(file, "int main(){{}}")?;
        drop(file);
        // Check for Standards versions
        use CStandard::*;
        for (std, flag) in [
            (C89, "c89"),
            (C95, "c95"),
            (C99, "c99"),
            (C11, "c11"),
            (C18, "c18"),
            (C2x, "c2x"),
            (Cxx98, "c++98"),
            (Cxx03, "c++03"),
            (Cxx11, "c++11"),
            (Cxx14, "c++14"),
            (Cxx17, "c++17"),
            (Cxx20, "c++20"),
            (Cxx2x, "c++2x"),
            (Gnu89, "gnu89"),
            (Gnu95, "gnu95"),
            (Gnu99, "gnu99"),
            (Gnu11, "gnu11"),
            (Gnu18, "gnu18"),
            (Gnu2x, "gnu2x"),
            (Gxx98, "g++98"),
            (Gxx03, "g++03"),
            (Gxx11, "g++11"),
            (Gxx14, "g++14"),
            (Gxx17, "g++17"),
            (Gxx20, "g++20"),
            (Gxx2x, "g++2x"),
        ] {
            if Command::new(cc)
                .args(&properties.extra_opts)
                .args(&properties.compile_flags)
                .arg("-c")
                .arg(format!("-std={}", flag))
                .arg(&test_file)
                .status()?
                .success()
            {
                properties.standard.push(std);
            }
        }
    }

    Ok(())
}
