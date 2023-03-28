// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub(crate) fn compile_blueprint(blueprint: &[u8]) -> Result<String> {
    let mut compiler = Command::new("blueprint-compiler")
        .args(["compile", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("blueprint-compiler not found"));

    let mut stdin = compiler.stdin.take().unwrap();
    stdin.write_all(b"using Gtk 4.0;\n")?;
    stdin.write_all(blueprint)?;
    drop(stdin);

    let mut buf = String::new();
    compiler.stdout.unwrap().read_to_string(&mut buf)?;

    if !buf.starts_with('<') {
        bail!(buf);
    }

    Ok(buf)
}

pub(crate) fn blueprint_files(root: PathBuf) -> Vec<PathBuf> {
    let mut results = vec![];
    let entries = std::fs::read_dir(root)
        .unwrap()
        .filter_map(|entry| entry.ok());

    for entry in entries {
        let path = entry.path();
        match std::fs::metadata(&path) {
            Ok(metadata) if metadata.is_dir() => {
                results.append(&mut blueprint_files(path));
            }
            _ => {
                if let Some(ext) = path.extension() {
                    if ext == "blp" {
                        results.push(path)
                    }
                }
            }
        }
    }
    results
}

pub(crate) fn find_blueprint(name: &str) -> Result<PathBuf> {
    let root = &std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let direct_path = Path::new(root).join(name);
    if direct_path.exists() {
        return Ok(direct_path);
    } else {
        let src_path = Path::new(root).join("src");
        let blueprints = blueprint_files(src_path);

        let found: Vec<_> = blueprints
            .into_iter()
            .filter(|path| path.ends_with(name))
            .collect();

        match found.len() {
            0 => bail!("No files found for name {name}"),
            1 => Ok(found[0].to_owned()),
            _ => bail!("Found too many files for name {name}: {found:?}"),
        }
    }
}

#[test]
fn find_files() {
    let correct_paths = ["my_widget.blp"];

    for name in correct_paths {
        dbg!(find_blueprint(name).unwrap());
    }
}
