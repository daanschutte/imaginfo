use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn it_gives_error_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn it_finds_all_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tests/test_dir/file0_1.txt"))
        .stdout(predicate::str::contains("tests/test_dir/file0_2.txt"));

    Ok(())
}

#[test]
fn it_does_not_follow_symlinks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("symlink_to_file3.txt").not());

    Ok(())
}

#[test]
fn it_does_not_output_dirs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tests/test_dir/level1").not());

    Ok(())
}

#[test]
fn it_does_not_output_hidden_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".hidden").not());

    Ok(())
}

#[test]
fn it_gives_error_if_file_does_not_exist_with_recursion() -> Result<(), Box<dyn std::error::Error>>
{
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir/does/not/exist").arg("-r");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn it_finds_all_files_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir").arg("-r");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tests/test_dir/file0_1.txt"))
        .stdout(predicate::str::contains("tests/test_dir/file0_2.txt"))
        .stdout(predicate::str::contains("tests/test_dir/level1/file1.txt"))
        .stdout(predicate::str::contains(
            "tests/test_dir/level1/level2a/file2a_1.txt",
        ))
        .stdout(predicate::str::contains(
            "tests/test_dir/level1/level2a/file2a_2.txt",
        ))
        .stdout(predicate::str::contains(
            "tests/test_dir/level1/level2b/file2b.txt",
        ))
        .stdout(predicate::str::contains(
            "tests/test_dir/level1/level2a/level3/file3.txt",
        ));

    Ok(())
}

#[test]
fn it_does_not_follow_symlinks_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir").arg("-r");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("symlink_to_level1").not())
        .stdout(predicate::str::contains("symlink_to_file0_1.txt").not())
        .stdout(predicate::str::contains("symlink_to_file3.txt").not());

    Ok(())
}

#[test]
fn it_does_not_output_dirs_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir").arg("-r");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("empty_dir").not());

    Ok(())
}

#[test]
fn it_does_not_output_hidden_files_with_recursion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imaginfo")?;

    cmd.arg("tests/test_dir").arg("-r");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(".hidden").not())
        .stdout(predicate::str::contains(".hidden2b").not());

    Ok(())
}
