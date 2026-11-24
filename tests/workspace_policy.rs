//! Rules over the whole workspace that no single crate can check for itself.
//!
//! - No crate in the dependency graph is `reqwest`, `openssl` or `native-tls`: one HTTP client
//!   (the broker's) and one TLS stack (rustls). `Cargo.lock` is the graph, with every optional and
//!   platform-specific dependency in it.
//! - No file under `crates/` is longer than 300 lines.
//! - Every action a workflow uses is pinned by a full commit id. A tag or a branch can be moved to
//!   new code after it was reviewed.
//!
//! Each rule is a function, tested on a planted case before it is run on the workspace: a rule
//! that stopped matching would otherwise pass everything.

use std::fs;
use std::path::{Path, PathBuf};

const BANNED: &[&str] = &["reqwest", "openssl", "openssl-sys", "native-tls"];
const MAX_LINES: usize = 300;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The package names in a `Cargo.lock`.
fn locked_packages(lock: &str) -> Vec<&str> {
    let mut names = Vec::new();
    let mut in_package = false;
    for line in lock.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[[package]]";
        } else if in_package && let Some(rest) = line.strip_prefix("name = \"") {
            names.push(rest.trim_end_matches('"'));
        }
    }
    names
}

/// The banned crates a `Cargo.lock` holds.
fn banned_in(lock: &str) -> Vec<&str> {
    locked_packages(lock)
        .into_iter()
        .filter(|name| BANNED.contains(name))
        .collect()
}

/// Every file under `dir`, skipping build output.
fn files_under(dir: &Path, found: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
            .path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name != "target") {
                files_under(&path, found);
            }
        } else {
            found.push(path);
        }
    }
}

/// The files under `dir` that are longer than `max` lines, with their line counts.
fn too_long(dir: &Path, max: usize) -> Vec<(PathBuf, usize)> {
    let mut files = Vec::new();
    files_under(dir, &mut files);
    files.sort();
    files
        .into_iter()
        .filter_map(|path| {
            // A file that is not UTF-8 text has no lines to count.
            let lines = fs::read_to_string(&path).ok()?.lines().count();
            (lines > max).then_some((path, lines))
        })
        .collect()
}

/// The `uses:` references in a workflow that are not `<owner>/<repo>[/path]@<40 hex digits>`.
/// A local action (`./path`) is this repository's own code.
fn unpinned_actions(workflow: &str) -> Vec<&str> {
    workflow
        .lines()
        .filter_map(|line| {
            let line = line.trim().trim_start_matches("- ").trim_start();
            let reference = line.strip_prefix("uses:")?.split('#').next()?.trim();
            let reference = reference.trim_matches(|quote| quote == '"' || quote == '\'');
            let pinned = reference.rsplit_once('@').is_some_and(|(action, commit)| {
                action.contains('/')
                    && commit.len() == 40
                    && commit
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            });
            (!reference.starts_with("./") && !pinned).then_some(reference)
        })
        .collect()
}

#[test]
fn a_planted_unpinned_action_is_found() {
    let workflow = "jobs:\n  a:\n    steps:\n      \
        - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n      \
        - uses: actions/cache@v6\n      \
        - name: named\n        uses: \"owner/repo/path@main\"\n      \
        - uses: ./local/action\n      \
        - uses: owner/repo@3D3C42E5AAC5BA805825DA76410C181273BA90B1\n      \
        - uses: owner/repo@3d3c42e\n      \
        - uses: docker://alpine:3\n      \
        - run: echo uses: nothing\n";
    assert_eq!(
        unpinned_actions(workflow),
        [
            "actions/cache@v6",
            "owner/repo/path@main",
            "owner/repo@3D3C42E5AAC5BA805825DA76410C181273BA90B1",
            "owner/repo@3d3c42e",
            "docker://alpine:3",
        ]
    );
}

#[test]
fn every_workflow_action_is_pinned_by_commit() {
    let mut files = Vec::new();
    files_under(&root().join(".github/workflows"), &mut files);
    // The read is not empty: the gate's own workflow is in it.
    assert!(files.iter().any(|path| path.ends_with("ci.yml")));
    for path in files {
        let workflow = fs::read_to_string(&path).unwrap();
        assert_eq!(unpinned_actions(&workflow), [""; 0], "{}", path.display());
    }
}

#[test]
fn a_planted_banned_crate_is_found() {
    let lock = "version = 4\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\n\n\
                [[package]]\nname = \"reqwest\"\nversion = \"0.13.0\"\ndependencies = [\n \"native-tls\",\n]\n\n\
                [[package]]\nname = \"native-tls\"\nversion = \"0.2.0\"\n\n\
                [[package]]\nname = \"openssl-sys\"\nversion = \"0.9.0\"\n\n\
                [metadata]\nname = \"openssl\"\n";
    assert_eq!(banned_in(lock), ["reqwest", "native-tls", "openssl-sys"]);
    // A crate whose name only contains a banned one is a different crate.
    assert_eq!(
        banned_in("[[package]]\nname = \"reqwest-middleware-types\"\n"),
        [""; 0]
    );
    assert_eq!(banned_in("[[package]]\nname = \"openssl\"\n"), ["openssl"]);
}

#[test]
fn no_crate_depends_on_reqwest_openssl_or_native_tls() {
    let lock = fs::read_to_string(root().join("Cargo.lock")).unwrap();
    let packages = locked_packages(&lock);
    // The read is not empty: the workspace's own crates are in it.
    assert!(
        packages.contains(&"bma-types"),
        "Cargo.lock lists no workspace crate"
    );
    assert_eq!(banned_in(&lock), [""; 0], "banned crates in Cargo.lock");
}

#[test]
fn a_planted_long_file_is_found() {
    let dir = std::env::temp_dir().join(format!("bma-policy-{}", std::process::id()));
    fs::create_dir_all(dir.join("nested/target")).unwrap();
    fs::write(
        dir.join("nested/long.rs"),
        "// line\n".repeat(MAX_LINES + 1),
    )
    .unwrap();
    fs::write(dir.join("nested/edge.rs"), "// line\n".repeat(MAX_LINES)).unwrap();
    fs::write(
        dir.join("nested/target/built.rs"),
        "// line\n".repeat(MAX_LINES * 2),
    )
    .unwrap();
    let found = too_long(&dir, MAX_LINES);
    fs::remove_dir_all(&dir).unwrap();
    assert_eq!(found, [(dir.join("nested/long.rs"), MAX_LINES + 1)]);
}

#[test]
fn no_file_under_crates_is_longer_than_300_lines() {
    let crates = root().join("crates");
    let mut files = Vec::new();
    files_under(&crates, &mut files);
    // The read is not empty: the walk found the crate this workspace starts with.
    assert!(files.contains(&crates.join("bma-types/src/lib.rs")));
    assert_eq!(
        too_long(&crates, MAX_LINES),
        [],
        "files over {MAX_LINES} lines"
    );
}
