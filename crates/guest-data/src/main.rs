//! Reads the player's own guest-game install and writes what it finds to `local/guest/`.
//!
//! Current stage: checks the install path and writes a file inventory. It never writes to the
//! install. Parsing the game's formats comes later.
//!
//! Usage: cargo run -p guest-data -- "<path to the guest game install>"

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, u64)>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        if meta.is_dir() {
            walk(&path, root, out)?;
        } else {
            let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
            out.push((rel, meta.len()));
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let Some(install) = std::env::args_os().nth(1).map(PathBuf::from) else {
        eprintln!("usage: guest-data <guest game install folder>");
        return ExitCode::FAILURE;
    };
    if !install.is_dir() {
        eprintln!("not a folder: {}", install.display());
        return ExitCode::FAILURE;
    }

    let mut files = Vec::new();
    if let Err(e) = walk(&install, &install, &mut files) {
        eprintln!("could not read {}: {e}", install.display());
        return ExitCode::FAILURE;
    }
    files.sort();

    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../local/guest");
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("could not create {}: {e}", out_dir.display());
        return ExitCode::FAILURE;
    }
    let out_path = out_dir.join("inventory.txt");
    let mut f = match fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not write {}: {e}", out_path.display());
            return ExitCode::FAILURE;
        }
    };
    for (name, size) in &files {
        let _ = writeln!(f, "{size}\t{name}");
    }

    println!("{} files listed in {}", files.len(), out_path.display());
    ExitCode::SUCCESS
}
