# The serve command

The serve command is used to preview a book by serving it via HTTP at
`localhost:3000` by default: 

```bash
mdbook serve
```

The `serve` command  watches the book's `src` directory for
changes, rebuilding the book and refreshing clients for each change; this includes
re-creating deleted files still mentioned in `SUMMARY.md`! A websocket
connection is used to trigger the client-side refresh.

***Note:*** *The `serve` command is for testing a book's HTML output, and is not
intended to be a complete HTTP server for a website.*

#### Specify a directory

The `serve` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
mdbook serve path/to/book
```

### Server options

The `serve` hostname defaults to `localhost`, and the port defaults to `3000`. Either option can be specified on the command line:

```bash
mdbook serve path/to/book -p 8000 -n 127.0.0.1 
```

#### --open

When you use the `--open` (`-o`) flag, mdbook will open the book in your
default web browser after starting the server.

#### --dest-dir

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of the `build.build-dir` key in
`book.toml`, or to `./book`.

#### Specify exclude patterns

The `serve` command will not automatically trigger a build for files listed in
the `.gitignore` file in the book root directory. The `.gitignore` file may
contain file patterns described in the [gitignore
documentation](https://git-scm.com/docs/gitignore). This can be useful for
ignoring temporary files created by some editors.

***Note:*** *Only the `.gitignore` from the book root directory is used. Global
`$HOME/.gitignore` or `.gitignore` files in parent directories are not used.*
