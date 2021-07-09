# The test command

When writing a book, you sometimes need to automate some tests. For example,
[The Rust Programming Book](https://doc.rust-lang.org/stable/book/) uses a lot
of code examples that could get outdated. Therefore it is very important for
them to be able to automatically test these code examples.

mdBook supports a `test` command that will run all available tests in a book. At
the moment, only rustdoc tests are supported, but this may be expanded upon in
the future.

#### Disable tests on a code block

rustdoc doesn't test code blocks which contain the `ignore` attribute:

    ```rust,ignore
    fn main() {}
    ```

rustdoc also doesn't test code blocks which specify a language other than Rust:

    ```markdown
    **Foo**: _bar_
    ```

rustdoc *does* test code blocks which have no language specified:

    ```
    This is going to cause an error!
    ```

#### Specify a directory

The `test` command can take a directory as an argument to use as the book's root
instead of the current working directory.

```bash
mdbook test path/to/book
```

#### --library-path

The `--library-path` (`-L`) option allows you to add directories to the library
search path used by `rustdoc` when it builds and tests the examples. Multiple
directories can be specified with multiple options (`-L foo -L bar`) or with a
comma-delimited list (`-L foo,bar`). The path should point to the Cargo 
[build cache](https://doc.rust-lang.org/cargo/guide/build-cache.html) `deps` directory that
contains the build output of your project. For example, if your Rust project's book is in a directory
named `my-book`, the following command would include the crate's dependencies when running `test`:

```shell
mdbook test my-book -L target/debug/deps/
```

See the `rustdoc` command-line [documentation](https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies)
for more information.

#### --dest-dir

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of the `build.build-dir` key in
`book.toml`, or to `./book`.
