# The clean command

The clean command is used to delete the generated book and any other build
artifacts.

```bash
mdbook clean
```

#### Specify a directory

The `clean` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
mdbook clean path/to/book
```

#### --dest-dir

The `--dest-dir` (`-d`) option allows you to override the book's output
directory, which will be deleted by this command. Relative paths are interpreted
relative to the book's root directory. If not specified it will default to the
value of the `build.build-dir` key in `book.toml`, or to `./book`.

```bash
mdbook clean --dest-dir=path/to/book
```

`path/to/book` could be absolute or relative.