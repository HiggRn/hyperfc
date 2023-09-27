use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use clap::arg;
use clap::builder::ValueParser;
use clap::command;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::ValueHint;

mod llvm;
mod shell;

fn main() {
    let default_triple_cstring = llvm::get_default_target_triple();
    let default_triple = default_triple_cstring.to_str().unwrap();

    let matches = command!()
        .args(&[
            arg!(path: <SOURCE_FILE> "The path to the hyperfuck program to compile")
                .value_hint(ValueHint::FilePath)
                .value_parser(ValueParser::path_buf()),
            arg!(-O --opt <LEVEL> "Optimization level")
                .value_parser(["0", "1", "2"])
                .default_value("2"),
            arg!(--"llvm-opt" <LEVEL> "LLVM optimization level")
                .value_parser(["0", "1", "2", "3"])
                .default_value("3"),
            arg!(--passes <"PASS-SPECIFICATION"> "Limit hyperfc optimizations to those specified"),
            arg!(strip: -S "Strip symbols from the binary").action(ArgAction::SetTrue),
            arg!(--target <TARGET> "LLVM target triple").default_value(default_triple.to_owned()),
            arg!(--"dump-llvm" "Print the LLVM IR generated").action(ArgAction::SetTrue),
            arg!(--"dump-ir" "Print the HF IR generated").action(ArgAction::SetTrue),
        ])
        .get_matches();

    match compile_file(&matches) {
        Ok(_) => (),
        Err(()) => std::process::exit(2),
    }
}

/// Read the contents of the file at path, and return a string of its
/// contents. Return a diagnostic if we can't open or read the file.
fn slurp(path: &Path) -> Result<String, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(message) => return Err(format!("{}: {message}", path.display())),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(message) => Err(format!("{}: {message}", path.display())),
    }
}

/// Convert "foo.hf" to "foo".
fn executable_name(hf_path: &Path) -> String {
    let hf_file_name = hf_path.file_name().unwrap().to_str().unwrap();

    let mut name_parts: Vec<_> = hf_file_name.split('.').collect();
    let parts_len = name_parts.len();
    if parts_len > 1 {
        name_parts.pop();
    }

    name_parts.join(".")
}

/// Compile source files and link.
fn compile_file(matches: &ArgMatches) -> Result<(), ()> {
    let path: &PathBuf = matches.get_one("path").expect("Required argument");

    let src = slurp(path).map_err(|e| eprintln!("{e}"));
    todo!()
}

/// Link the object file.
fn link_object_file(
    object_file_path: &str,
    executable_path: &str,
    target_triple: Option<String>,
    strip: bool,
) -> Result<(), String> {
    let mut clang_args = vec![object_file_path, "-o", executable_path];

    if let Some(ref target_triple) = target_triple {
        clang_args.push("-target");
        clang_args.push(target_triple);
    }

    if strip {
        clang_args.push("-s");
    }

    shell::run_shell_command("clang", &clang_args[..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn executable_name_bf() {
        assert_eq!(executable_name(&PathBuf::from("foo.bf")), "foo");
    }

    #[test]
    fn executable_name_b() {
        assert_eq!(executable_name(&PathBuf::from("foo_bar.b")), "foo_bar");
    }

    #[test]
    fn executable_name_relative_path() {
        assert_eq!(executable_name(&PathBuf::from("bar/baz.bf")), "baz");
    }
}
