use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

// TODO use tempfile https://docs.rs/tempfile/3.2.0/tempfile/
// The current implementation does not work well since we have to commit symlinks: boo :(

fn setup_cmd_debug_recurse(cmd: &mut Command) {
    cmd.env("RUST_LOG", "debug")
        .arg("--debug")
        .arg("--recurse")
        .arg("tests/test_dir");
}

#[test]
fn it_gives_error_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.env("RUST_LOG", "debug")
        .arg("--debug")
        .arg("--recurse")
        .arg("tests/test_dir/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn it_gives_error_if_file_does_not_exist_with_recursion() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.env("RUST_LOG", "debug")
        .arg("tests/test_dir/does/not/exist")
        .arg("--debug")
        .arg("--recurse");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn it_finds_all_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("tests/test_dir/file0_1.txt"))
        .stderr(predicate::str::contains("tests/test_dir/file0_2.txt"));

    Ok(())
}

#[test]
fn it_finds_all_files_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("tests/test_dir/file0_1.txt"))
        .stderr(predicate::str::contains("tests/test_dir/file0_2.txt"))
        .stderr(predicate::str::contains("tests/test_dir/level1/file1.txt"))
        .stderr(predicate::str::contains(
            "tests/test_dir/level1/level2a/file2a_1.txt",
        ))
        .stderr(predicate::str::contains(
            "tests/test_dir/level1/level2a/file2a_2.txt",
        ))
        .stderr(predicate::str::contains(
            "tests/test_dir/level1/level2b/file2b.txt",
        ))
        .stderr(predicate::str::contains(
            "tests/test_dir/level1/level2a/level3/file3.txt",
        ));

    Ok(())
}

#[test]
fn it_does_not_output_dirs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("tests/test_dir/level2b/empty_dir").not());

    Ok(())
}

#[test]
fn it_does_not_output_dirs_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("empty_dir").not());

    Ok(())
}

#[test]
fn it_does_not_follow_symlinks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("symlink_to_file3.txt").not());

    Ok(())
}

#[test]
fn it_does_not_follow_symlinks_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("symlink_to_file0_1").not())
        .stderr(predicate::str::contains("symlink_to_file3").not());

    Ok(())
}

#[test]
fn it_does_not_output_hidden_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(".hidden").not());

    Ok(())
}

#[test]
fn it_does_not_output_hidden_files_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    setup_cmd_debug_recurse(&mut cmd);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(".hidden").not())
        .stderr(predicate::str::contains(".hidden2b").not());

    Ok(())
}
