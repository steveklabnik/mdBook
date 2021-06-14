# Environment Variables

All configuration values can be overridden from the command line by setting the
corresponding environment variable. Because many operating systems restrict
environment variables to be alphanumeric characters or `_`, the configuration
key needs to be formatted slightly differently to the normal `foo.bar.baz` form.

Variables starting with `MDBOOK_` are used for configuration. The key is created
by removing the `MDBOOK_` prefix and turning the resulting string into
`kebab-case`. Double underscores (`__`) separate nested keys, while a single
underscore (`_`) is replaced with a dash (`-`).

For example:

- `MDBOOK_foo` -> `foo`
- `MDBOOK_FOO` -> `foo`
- `MDBOOK_FOO__BAR` -> `foo.bar`
- `MDBOOK_FOO_BAR` -> `foo-bar`
- `MDBOOK_FOO_bar__baz` -> `foo-bar.baz`

So by setting the `MDBOOK_BOOK__TITLE` environment variable you can override the
book's title without needing to touch your `book.toml`.

> **Note:** To facilitate setting more complex config items, the value of an
> environment variable is first parsed as JSON, falling back to a string if the
> parse fails.
>
> This means, if you so desired, you could override all book metadata when
> building the book with something like
>
> ```shell
> $ export MDBOOK_BOOK="{'title': 'My Awesome Book', authors: ['Michael-F-Bryan']}"
> $ mdbook build
> ```

The latter case may be useful in situations where `mdbook` is invoked from a
script or CI, where it sometimes isn't possible to update the `book.toml` before
building.
