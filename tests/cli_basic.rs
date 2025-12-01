use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
#[allow(deprecated)]
fn cli_deletes_direct_child_directory_on_confirmed_y() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let child = temp.child("music");
    child.create_dir_all()?;

    let mut cmd = Command::cargo_bin("srmd")?;
    cmd.current_dir(temp.path())
        .arg("music")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child_process = cmd.spawn()?;
    {
        let stdin = child_process
            .stdin
            .as_mut()
            .expect("failed to open stdin for srmd");
        stdin.write_all(b"y\n")?;
    }

    let output = child_process.wait_with_output()?;
    assert!(
        output.status.success(),
        "srmd did not exit successfully: {:?}",
        output.status
    );
    assert!(
        !child.path().exists(),
        "directory should have been removed by srmd"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("delete "));
    assert!(stdout.contains("[y/N]"));

    Ok(())
}

#[test]
#[allow(deprecated)]
fn cli_does_not_delete_when_user_declines() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let child = temp.child("to_keep");
    child.create_dir_all()?;

    let mut cmd = Command::cargo_bin("srmd")?;
    cmd.current_dir(temp.path())
        .arg("to_keep")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child_process = cmd.spawn()?;
    {
        let stdin = child_process
            .stdin
            .as_mut()
            .expect("failed to open stdin for srmd");
        stdin.write_all(b"n\n")?;
    }

    let output = child_process.wait_with_output()?;
    assert!(
        output.status.success(),
        "srmd should still exit successfully when user declines: {:?}",
        output.status
    );
    assert!(
        child.path().exists(),
        "directory should not have been removed when user declined"
    );

    Ok(())
}

#[test]
#[allow(deprecated)]
fn cli_fails_for_non_direct_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let parent = temp.child("music");
    parent.create_dir_all()?;
    let child = parent.child("old");
    child.create_dir_all()?;

    let mut cmd = Command::cargo_bin("srmd")?;
    cmd.current_dir(temp.path())
        .arg("music/old")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child_process = cmd.spawn()?;
    {
        let stdin = child_process
            .stdin
            .as_mut()
            .expect("failed to open stdin for srmd");
        stdin.write_all(b"y\n")?;
    }

    let output = child_process.wait_with_output()?;
    assert!(
        !output.status.success(),
        "srmd should fail for non-direct subdirectory: {:?}",
        output.status
    );
    assert!(
        child.path().exists(),
        "non-direct subdirectory should not have been removed"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("This is not a subdirectory"));

    Ok(())
}

#[test]
#[allow(deprecated)]
fn cli_fails_for_directory_outside_current_dir() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let other = TempDir::new()?;
    let other_child = other.child("other_dir");
    other_child.create_dir_all()?;

    let mut cmd = Command::cargo_bin("srmd")?;
    cmd.current_dir(temp.path())
        .arg(other_child.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child_process = cmd.spawn()?;
    {
        let stdin = child_process
            .stdin
            .as_mut()
            .expect("failed to open stdin for srmd");
        stdin.write_all(b"y\n")?;
    }

    let output = child_process.wait_with_output()?;
    assert!(
        !output.status.success(),
        "srmd should fail for directory outside current dir: {:?}",
        output.status
    );
    assert!(
        other_child.path().exists(),
        "directory outside current dir should not have been removed"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("This is not a subdirectory"));

    Ok(())
}


