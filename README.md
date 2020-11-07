# rmd

Command line utility to safely delete a directory. This is a safer alternative to `rm -rf`.

## Advantages
* This utility will only delete subdirectories of the current working directory. 
* It will never traverse to the parent directory.
* It only allows you to delete one directory at a time
* It asks for confirmation before deletion

The standard utility `rm` has the following typical issue. Let's say you're in the directory `/home/user/` and you want to delete the folder `music`. If you're using tab completion it will usually append a slash to the folder you're trying to delete like so: `rm -rf music/`. This is quite dangerous because one additional space will mean you're deleting the root folder: `rm -rf music /`
 
 ## Usage
```
rmd dir_to_delete
```
 
 ## Installation
 You need to have the [rust language](https://www.rust-lang.org/) compiler, rustc installed, then: 
 ```
 make
 sudo make install
 ```
 This will copy the `rmd` binary to `/usr/local/bin/`. 
 