//! The CLI's dependency on envault-core carries an explicit `version` (required to publish);
//! it must track the workspace version or a published CLI would pull an older core from crates.io.
#[test]
fn core_dependency_version_matches_workspace() {
    let manifest: toml::Value = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    let dep = &manifest["dependencies"]["envault-core"]["version"];
    assert_eq!(dep.as_str().unwrap(), env!("CARGO_PKG_VERSION"), "bump crates/cli/Cargo.toml envault-core.version with the workspace version");
}

/// npm/envault/package.json pins its own version and the four platform packages; the
/// release script rewrites them, but the checked-in file must not drift either.
#[test]
fn npm_launcher_versions_match_workspace() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../npm/envault/package.json");
    let Ok(text) = std::fs::read_to_string(path) else { return }; // not present in a crates.io tarball
    let pkg: serde_json::Value = serde_json::from_str(&text).unwrap();
    let v = env!("CARGO_PKG_VERSION");
    assert_eq!(pkg["version"].as_str().unwrap(), v, "npm/envault/package.json version");
    for (name, dep) in pkg["optionalDependencies"].as_object().unwrap() {
        assert_eq!(dep.as_str().unwrap(), v, "{name} pin in npm/envault/package.json");
    }
}
