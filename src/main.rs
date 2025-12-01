use std::env;
use std::fs;
use std::io::{stdin, Error, ErrorKind};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir()?;
    run_once(&current_dir, &args, input)
}

fn run_once<F>(current_dir: &Path, args: &[String], read_line: F) -> std::io::Result<()>
where
    F: Fn(&str) -> String,
{
    let current_dir = fs::canonicalize(current_dir)?;
    let dirname = get_dirname_from_args(args)?;
    let target = current_dir.join(&dirname);
    let to_delete = fs::canonicalize(&target)?;
    ensure_is_dir(&to_delete)?;
    ensure_is_subdirectory(&current_dir, &to_delete)?;
    let message = format!("delete {}? [y/N]", to_delete.display());
    let answer = read_line(&message);
    if answer.eq_ignore_ascii_case("y") {
        fs::remove_dir_all(to_delete)?;
    }
    Ok(())
}

fn get_dirname_from_args(args: &[String]) -> std::io::Result<String> {
    match args.get(1) {
        Some(filename) => Ok(filename.clone()),
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            "Please give directory to delete safely as argument",
        )),
    }
}

fn ensure_is_dir(path: &Path) -> std::io::Result<()> {
    match path.is_dir() {
        true => Ok(()),
        false => Err(Error::new(ErrorKind::InvalidInput, "Not a directory")),
    }
}

fn ensure_is_subdirectory(parent: &Path, child: &Path) -> std::io::Result<()> {
    let not_a_subdir = Err(Error::new(
        ErrorKind::InvalidInput,
        "This is not a subdirectory",
    ));
    let mut cc = child.iter();
    for p in parent.iter() {
        if let Some(c) = cc.next() {
            if p != c {
                return not_a_subdir;
            }
        } else {
            return not_a_subdir;
        }
    }
    // child must be exactly one level below parent: it needs exactly one more path component
    match (cc.next(), cc.next()) {
        (Some(_), None) => Ok(()),
        _ => not_a_subdir,
    }
}

fn input(message: &str) -> String {
    println!("{}", message);
    let mut ret = String::new();
    stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{ensure_is_dir, ensure_is_subdirectory, get_dirname_from_args, run_once};
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use std::io::ErrorKind;
    use std::path::Path;

    #[test]
    fn get_dirname_from_args_returns_first_argument_after_program_name(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args = vec![
            "srmd".to_string(),
            "target-dir".to_string(),
            "ignored".to_string(),
        ];
        let dirname = get_dirname_from_args(&args)?;
        assert_eq!(dirname, "target-dir");
        Ok(())
    }

    #[test]
    fn get_dirname_from_args_errors_when_missing_argument(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args = vec!["srmd".to_string()];
        let err = get_dirname_from_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "Please give directory to delete safely as argument"
        );
        Ok(())
    }

    #[test]
    fn ensure_is_dir_accepts_existing_directory() -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        assert!(ensure_is_dir(temp.path()).is_ok());
        Ok(())
    }

    #[test]
    fn ensure_is_dir_rejects_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let file = temp.child("file.txt");
        file.touch()?;

        let err = ensure_is_dir(file.path()).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        Ok(())
    }

    #[test]
    fn ensure_is_dir_rejects_nonexistent_path() -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let missing = temp.path().join("missing");
        let err = ensure_is_dir(&missing).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        Ok(())
    }

    #[test]
    fn direct_subdirectory_is_ok() -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b/c");
        assert!(ensure_is_subdirectory(parent, child).is_ok());
        Ok(())
    }

    #[test]
    fn same_directory_is_not_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b");
        assert!(ensure_is_subdirectory(parent, child).is_err());
        Ok(())
    }

    #[test]
    fn parent_directory_is_not_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/a");
        assert!(ensure_is_subdirectory(parent, child).is_err());
        Ok(())
    }

    #[test]
    fn unrelated_directory_is_not_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/x/y");
        assert!(ensure_is_subdirectory(parent, child).is_err());
        Ok(())
    }

    #[test]
    fn deeper_nested_directory_is_not_direct_subdirectory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b/c/d");
        assert!(ensure_is_subdirectory(parent, child).is_err());
        Ok(())
    }

    #[test]
    fn trailing_slash_does_not_make_same_directory_a_subdirectory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b/");
        assert!(ensure_is_subdirectory(parent, child).is_err());
        Ok(())
    }

    #[test]
    fn root_and_direct_child_are_ok() -> Result<(), Box<dyn std::error::Error>> {
        let parent = Path::new("/");
        let child = Path::new("/tmp");
        assert!(ensure_is_subdirectory(parent, child).is_ok());
        Ok(())
    }

    #[test]
    fn run_once_deletes_direct_subdirectory_when_user_confirms(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let child = temp.child("to_delete");
        child.create_dir_all()?;
        assert!(child.path().exists());

        let args = vec!["srmd".to_string(), "to_delete".to_string()];
        run_once(temp.path(), &args, |_| "y".to_string())?;

        assert!(!child.path().exists());
        Ok(())
    }

    #[test]
    fn run_once_does_not_delete_when_user_declines(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let child = temp.child("to_keep");
        child.create_dir_all()?;

        let args = vec!["srmd".to_string(), "to_keep".to_string()];
        run_once(temp.path(), &args, |_| "n".to_string())?;

        assert!(child.path().exists());
        Ok(())
    }

    #[test]
    fn run_once_does_not_delete_on_unrecognized_answer(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let child = temp.child("to_keep");
        child.create_dir_all()?;

        let args = vec!["srmd".to_string(), "to_keep".to_string()];
        run_once(temp.path(), &args, |_| "maybe".to_string())?;

        assert!(child.path().exists());
        Ok(())
    }

    #[test]
    fn run_once_errors_for_deeper_nested_directory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let parent = temp.child("parent");
        parent.create_dir_all()?;
        let child = parent.child("grandchild");
        child.create_dir_all()?;

        let args = vec!["srmd".to_string(), "parent/grandchild".to_string()];
        let result = run_once(temp.path(), &args, |_| "y".to_string());

        assert!(result.is_err());
        assert!(child.path().exists());
        Ok(())
    }

    #[test]
    fn run_once_errors_for_directory_outside_current_dir(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp = TempDir::new()?;
        let other = TempDir::new()?;
        let other_child = other.child("other_dir");
        other_child.create_dir_all()?;

        let args = vec![
            "srmd".to_string(),
            other_child.path().to_string_lossy().to_string(),
        ];
        let result = run_once(temp.path(), &args, |_| "y".to_string());

        assert!(result.is_err());
        assert!(other_child.path().exists());
        Ok(())
    }
}
