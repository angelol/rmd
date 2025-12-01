use std::env;
use std::fs;
use std::io::{stdin, Error, ErrorKind};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dirname = get_dirname()?;
    let to_delete = fs::canonicalize(dirname)?;
    ensure_is_dir(&to_delete)?;
    let current_dir = env::current_dir()?;
    ensure_is_subdirectory(&current_dir, &to_delete)?;
    let message = format!("delete {}? [y/N]", to_delete.display());
    let answer = input(&message);
    if answer.eq_ignore_ascii_case("y") {
        fs::remove_dir_all(to_delete)?;
    }
    Ok(())
}

fn get_dirname() -> std::io::Result<String> {
    match env::args().nth(1) {
        Some(filename) => Ok(filename),
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
    use super::ensure_is_subdirectory;
    use std::path::Path;

    #[test]
    fn direct_subdirectory_is_ok() {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b/c");
        assert!(ensure_is_subdirectory(parent, child).is_ok());
    }

    #[test]
    fn same_directory_is_not_subdirectory() {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b");
        assert!(ensure_is_subdirectory(parent, child).is_err());
    }

    #[test]
    fn parent_directory_is_not_subdirectory() {
        let parent = Path::new("/a/b");
        let child = Path::new("/a");
        assert!(ensure_is_subdirectory(parent, child).is_err());
    }

    #[test]
    fn unrelated_directory_is_not_subdirectory() {
        let parent = Path::new("/a/b");
        let child = Path::new("/x/y");
        assert!(ensure_is_subdirectory(parent, child).is_err());
    }

    #[test]
    fn deeper_nested_directory_is_not_direct_subdirectory() {
        let parent = Path::new("/a/b");
        let child = Path::new("/a/b/c/d");
        assert!(ensure_is_subdirectory(parent, child).is_err());
    }
}
