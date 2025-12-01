# rmd

Safety‑first command line utility to delete a **single directory**. `rmd` is a deliberately limited, safer alternative to `rm -rf` when you only want to remove one subdirectory of your current working directory.

## Why?

The standard `rm` command can be surprisingly dangerous, especially when combined with `-rf` and shell completion. For example, imagine you are in `/home/user` and want to delete the folder `music`:

```bash
rm -rf music/
```

With tab completion, shells often add a trailing slash. One stray space turns this into a disaster:

```bash
rm -rf music /
```

Now `/` is also an argument, and you are recursively deleting the root filesystem. `rmd` exists to _not_ let you do that kind of thing.

## Safety guarantees

`rmd` intentionally has a very small, strict behavior surface:

- Only **direct subdirectories** of the current working directory can be deleted.
- It **never** traverses to the parent directory and will not delete the current directory itself.
- It only allows you to delete **one directory at a time**.
- It always asks for confirmation and only proceeds on explicit `y` / `Y` (default is **no**).
- Paths are resolved using `canonicalize`, so it behaves safely even when symlinks are involved.

If any of these conditions are not met, `rmd` fails with an error instead of guessing what you meant.

## Usage

Basic usage:

```bash
rmd path/to/subdir
```

Examples:

```bash
$ pwd
/home/user/projects

$ ls
music  notes  tmp

$ rmd music
delete /home/user/projects/music? [y/N] y
# directory removed
```

Attempts that will **not** be allowed:

```bash
# Not a direct subdirectory (too high up)
$ rmd /home/user
Error: This is not a subdirectory

# Same directory as CWD
$ rmd .
Error: Not a directory   # or another InvalidInput error, depending on your shell expansion

# Deeper nested path (not a direct child)
$ rmd music/old
Error: This is not a subdirectory
```

The tool is designed to err on the side of refusing to act rather than risk deleting the wrong thing.

## Installation

You need the [Rust](https://www.rust-lang.org/) toolchain installed (`cargo`, `rustc`). Then build from source:

```bash
cargo build --release
sudo cp ./target/release/rmd /usr/local/bin/
```

Now `rmd` should be available in your `PATH`:

```bash
rmd --help  # once CLI flags are added
```

## Contributing

Issues and pull requests are welcome. Some possible areas to help with:

- Improving error messages and CLI UX.
- Adding more tests and platform coverage.
- Packaging for various systems (Homebrew, Nix, AUR, etc.).

Before opening a PR, please:

- Run the test suite: `cargo test`
- Format the code: `cargo fmt`

## License

`rmd` is licensed under the MIT License. See the `LICENSE` file for details.
