use assert_fs::prelude::*;
use assert_fs::TempDir;

fn run_srmd_with_input(
    current_dir: &TempDir,
    arg: &str,
    input: &str,
) -> assert_cmd::assert::Assert {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("srmd");
    cmd.current_dir(current_dir.path()).arg(arg).write_stdin(input);
    cmd.assert()
}

#[test]
fn cli_deletes_direct_child_directory_on_confirmed_y() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let child = temp.child("music");
    child.create_dir_all()?;

    let assert = run_srmd_with_input(&temp, "music", "y\n").success();

    assert!(
        !child.path().exists(),
        "directory should have been removed by srmd"
    );

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("delete "));
    assert!(stdout.contains("[y/N]"));

    Ok(())
}

#[test]
fn cli_does_not_delete_when_user_declines() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let child = temp.child("to_keep");
    child.create_dir_all()?;

    run_srmd_with_input(&temp, "to_keep", "n\n").success();

    assert!(
        child.path().exists(),
        "directory should not have been removed when user declined"
    );

    Ok(())
}

#[test]
fn cli_does_not_delete_on_unrecognized_answer() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let child = temp.child("to_keep");
    child.create_dir_all()?;

    run_srmd_with_input(&temp, "to_keep", "maybe\n").success();

    assert!(
        child.path().exists(),
        "directory should not have been removed on unrecognized answer"
    );

    Ok(())
}

#[test]
fn cli_fails_for_non_direct_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let parent = temp.child("music");
    parent.create_dir_all()?;
    let child = parent.child("old");
    child.create_dir_all()?;

    let assert = run_srmd_with_input(&temp, "music/old", "y\n").failure();

    assert!(
        child.path().exists(),
        "non-direct subdirectory should not have been removed"
    );

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("This is not a subdirectory"));

    Ok(())
}

#[test]
fn cli_fails_for_directory_outside_current_dir() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let other = TempDir::new()?;
    let other_child = other.child("other_dir");
    other_child.create_dir_all()?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("srmd");
    cmd.current_dir(temp.path())
        .arg(other_child.path())
        .write_stdin("y\n");

    let assert = cmd.assert().failure();

    assert!(
        other_child.path().exists(),
        "directory outside current dir should not have been removed"
    );

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("This is not a subdirectory"));

    Ok(())
}

