# The serve command

The serve command is used to preview a book by serving it over HTTP at
`127.0.0.1:3000` by default. Additionally it watches the book's directory for
changes, rebuilding the book and refreshing clients for each change. A websocket
connection is used to trigger the client-side refresh.

***Note:*** *The `serve` command is for testing a book's HTML output, and is not
intended to be a complete HTTP server for a website.*

#### Specify a directory

The `serve` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
mdbook serve path/to/book
```

#### Server options

`serve` has four options: the HTTP port, the WebSocket port, the HTTP hostname
to listen on, and the hostname for the browser to connect to for WebSockets.

For example: suppose you have an nginx server for SSL termination which has a
public address of 192.168.1.100 on port 80 and proxied that to 127.0.0.1 on port
8000\. To run use the nginx proxy do:

```bash
mdbook serve path/to/book -p 8000 -n 127.0.0.1 --websocket-hostname 192.168.1.100
```

If you were to want live reloading for this you would need to proxy the
websocket calls through nginx as well from `192.168.1.100:<WS_PORT>` to
`127.0.0.1:<WS_PORT>`. The `-w` flag allows for the websocket port to be
configured.

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

_Note: Only `.gitignore` from book root directory is used. Global
`$HOME/.gitignore` or `.gitignore` files in parent directories are not used._
