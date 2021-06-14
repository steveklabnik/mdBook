# The watch command

The `watch` command is useful when you want your book to be rendered on every
file change. You could repeatedly issue `mdbook build` every time a file is
changed. But using `mdbook watch` once will watch your files and will trigger a
build automatically whenever you modify a file; this includes re-creating
deleted files still mentioned in `SUMMARY.md`!

#### Specify a directory

The `watch` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
mdbook watch path/to/book
```

#### --open

When you use the `--open` (`-o`) option, mdbook will open the rendered book in
your default web browser.

#### --dest-dir

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of the `build.build-dir` key in
`book.toml`, or to `./book`.


#### Specify exclude patterns

The `watch` command will not automatically trigger a build for files listed in
the `.gitignore` file in the book root directory. The `.gitignore` file may
contain file patterns described in the [gitignore
documentation](https://git-scm.com/docs/gitignore). This can be useful for
ignoring temporary files created by some editors.

_Note: Only `.gitignore` from book root directory is used. Global
`$HOME/.gitignore` or `.gitignore` files in parent directories are not used._
