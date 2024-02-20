use dharitri_sc_meta::template::{template_names_from_repo, RepoSource, TemplateDownloader};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const TEMPLATE_TEMP_DIR_NAME: &str = "template-test";
const BUILD_CONTRACTS: bool = false;

#[test]
fn test_template_list() {
    let workspace_path = find_workspace();
    let repo_source = RepoSource::from_local_path(workspace_path);
    let mut template_names = template_names_from_repo(&repo_source);
    template_names.sort();
    assert_eq!(
        template_names,
        [
            "adder".to_string(),
            "crypto-zombies".to_string(),
            "empty".to_string()
        ]
    );
}

#[test]
#[cfg_attr(not(feature = "template-test"), ignore)]
fn template_test_adder() {
    template_test("adder", "new-adder");
}

#[test]
#[cfg_attr(not(feature = "template-test"), ignore)]
fn template_test_crypto_zombies() {
    template_test("crypto-zombies", "new-crypto-zombies");
}

#[test]
#[cfg_attr(not(feature = "template-test"), ignore)]
fn template_test_empty() {
    template_test("empty", "new-empty");
}

fn template_test(template_name: &str, new_name: &str) {
    let workspace_path = find_workspace();
    let repo_source = RepoSource::from_local_path(workspace_path);

    let target_dir = prepare_target_dir(new_name);

    let downloader =
        TemplateDownloader::new(&repo_source, template_name.to_string(), target_dir.clone());
    downloader.copy_template(&downloader.template_source.metadata.files_include);
    downloader.update_dependencies();
    downloader.rename_template_to(new_name.to_string());
    if BUILD_CONTRACTS {
        build_contract(&target_dir);
    }
    cargo_test(&target_dir);
}

fn prepare_target_dir(new_name: &str) -> PathBuf {
    let template_temp_path = find_workspace().join(TEMPLATE_TEMP_DIR_NAME);
    fs::create_dir_all(&template_temp_path).unwrap();

    let target_dir = template_temp_path.join(new_name);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).unwrap();
    }

    target_dir
}

pub fn cargo_test(contract_location: &Path) {
    let workspace_target_dir = find_workspace().join("target");

    let mut args = vec![
        "test",
        "--target-dir",
        workspace_target_dir.to_str().unwrap(),
    ];
    if BUILD_CONTRACTS {
        args.push("--features");
        args.push("dharitri-sc-scenario/run-go-tests");
    }

    let exit_status = Command::new("cargo")
        .args(args)
        .current_dir(contract_location)
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract test process failed");
}

pub fn build_contract(contract_location: &Path) {
    let workspace_target_dir = find_workspace().join("target");

    let exit_status = Command::new("cargo")
        .args([
            "run",
            "build",
            "--target-dir",
            workspace_target_dir.to_str().unwrap(),
        ])
        .current_dir(contract_location.join("meta"))
        .spawn()
        .expect("failed to spawn contract clean process")
        .wait()
        .expect("contract test process was not running");

    assert!(exit_status.success(), "contract build process failed");
}

/// Finds the workspace by taking the `current_exe` and working its way up.
/// Works in debug mode too.
///
/// TODO: duplicated code from scenario_world. De-duplicate after dependencies are reorganized.
pub fn find_workspace() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    let mut path = current_exe.as_path();
    while !is_target(path) {
        path = path.parent().unwrap();
    }

    path.parent().unwrap().into()
}

fn is_target(path_buf: &Path) -> bool {
    path_buf.file_name().unwrap() == "target"
}
