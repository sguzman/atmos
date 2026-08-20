use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn every_checked_in_asset_toml_parses() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let mut files = Vec::new();
    collect_toml_files(&root, &mut files);
    files.sort();

    assert!(
        !files.is_empty(),
        "expected at least one TOML asset under assets/"
    );

    let mut failures = Vec::new();
    for path in files {
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(err) => {
                failures.push(format!("{}: could not read: {err}", path.display()));
                continue;
            }
        };

        if let Err(err) = source.parse::<toml::Value>() {
            failures.push(format!("{}: {err}", path.display()));
        }
    }

    assert!(
        failures.is_empty(),
        "TOML asset validation failed:\n{}",
        failures.join("\n\n")
    );
}

fn collect_toml_files(directory: &Path, output: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|err| panic!("could not read {}: {err}", directory.display()));

    for entry in entries {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_toml_files(&path, output);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "toml")
        {
            output.push(path);
        }
    }
}
