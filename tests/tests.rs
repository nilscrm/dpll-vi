use insta_cmd::assert_cmd_snapshot;
use rstest::rstest;
use std::{fs::File, path::{Path, PathBuf}, process::Command};

fn test_cnf_files(group_name: &str, path: &Path) {
  insta::with_settings!({
    snapshot_path => format!("snapshots/{}", group_name),
    filters => [(r"Performance\s+Time\s+.*\sms\s+Speed.*IPS", "[Performance...]")] },
  {
    let stdin = File::open(path).unwrap();
    assert_cmd_snapshot!(path.file_stem().unwrap().to_str().unwrap(), Command::new("vine").args(["run", "dpll/main.vi"]).arg("--breadth-first").stdin(stdin));
  });
}

#[rstest]
fn test_minimal(#[files("tests/minimal/*.cnf")] path: PathBuf) {
  test_cnf_files("minimal", &path);
}

#[rstest]
fn test_medium(#[files("tests/medium/*.cnf")] path: PathBuf) {
  test_cnf_files("medium", &path);
}

#[rstest]
fn test_satlib_sat(#[files("tests/uf5-218_first_10/*.cnf")] path: PathBuf) {
  test_cnf_files("uf5-218_first_10", &path);
}

#[rstest]
fn test_satlib_unsat(#[files("tests/uuf50-218_first_10/*.cnf")] path: PathBuf) {
  test_cnf_files("uuf50-218_first_10", &path);
}
