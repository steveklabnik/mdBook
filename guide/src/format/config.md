# Configuration

You can configure the parameters for your book in the ***book.toml*** file.

Here is an example of what a ***book.toml*** file might look like:

```toml
[book]
title = "Example book"
author = "John Doe"
description = "The example book covers examples."

[rust]
edition = "2018"

[build]
build-dir = "my-example-book"
create-missing = false

[preprocessor.index]

[preprocessor.links]

[output.html]
additional-css = ["custom.css"]

[output.html.search]
limit-results = 15
```

## Supported configuration options

It is important to note that **any** relative path specified in the
configuration will always be taken relative from the root of the book where the
configuration file is located.

### General metadata

This is general information about your book.

- **title:** The title of the book
- **authors:** The author(s) of the book
- **description:** A description for the book, which is added as meta
  information in the html `<head>` of each page
- **src:** By default, the source directory is found in the directory named
  `src` directly under the root folder. But this is configurable with the `src`
  key in the configuration file.
- **language:** The main language of the book, which is used as a language attribute `<html lang="en">` for example.

**book.toml**
```toml
[book]
title = "Example book"
authors = ["John Doe", "Jane Doe"]
description = "The example book covers examples."
src = "my-src"  # the source files will be found in `root/my-src` instead of `root/src`
language = "en"
```

### Rust options

Options for the Rust language, relevant to running tests and playground
integration.

- **edition**: Rust edition to use by default for the code snippets. Default
  is "2015". Individual code blocks can be controlled with the `edition2015`
  or `edition2018` annotations, such as:

  ~~~text
  ```rust,edition2015
  // This only works in 2015.
  let try = true;
  ```
  ~~~

### Build options

This controls the build process of your book.

- **build-dir:** The directory to put the rendered book in. By default this is
  `book/` in the book's root directory.
- **create-missing:** By default, any missing files specified in `SUMMARY.md`
  will be created when the book is built (i.e. `create-missing = true`). If this
  is `false` then the build process will instead exit with an error if any files
  do not exist.
- **use-default-preprocessors:** Disable the default preprocessors of (`links` &
  `index`) by setting this option to `false`.

  If you have the same, and/or other preprocessors declared via their table
  of configuration, they will run instead.

  - For clarity, with no preprocessor configuration, the default `links` and
    `index` will run.
  - Setting `use-default-preprocessors = false` will disable these
    default preprocessors from running.
  - Adding `[preprocessor.links]`, for example, will ensure, regardless of
    `use-default-preprocessors` that `links` it will run.

## Configuring Preprocessors

The following preprocessors are available and included by default:

- `links`: Expand the `{{ #playground }}`, `{{ #include }}`, and `{{ #rustdoc_include }}` handlebars
  helpers in a chapter to include the contents of a file.
- `index`: Convert all chapter files named `README.md` into `index.md`. That is
  to say, all `README.md` would be rendered to an index file `index.html` in the
  rendered book.


**book.toml**
```toml
[build]
build-dir = "build"
create-missing = false

[preprocessor.links]

[preprocessor.index]
```

### Custom Preprocessor Configuration

Like renderers, preprocessor will need to be given its own table (e.g.
`[preprocessor.mathjax]`). In the section, you may then pass extra
configuration to the preprocessor by adding key-value pairs to the table.

For example

```toml
[preprocessor.links]
# set the renderers this preprocessor will run for
renderers = ["html"]
some_extra_feature = true
```

#### Locking a Preprocessor dependency to a renderer

You can explicitly specify that a preprocessor should run for a renderer by
binding the two together.

```toml
[preprocessor.mathjax]
renderers = ["html"]  # mathjax only makes sense with the HTML renderer
```

### Provide Your Own Command

By default when you add a `[preprocessor.foo]` table to your `book.toml` file,
`mdbook` will try to invoke the `mdbook-foo` executable. If you want to use a
different program name or pass in command-line arguments, this behaviour can
be overridden by adding a `command` field.

```toml
[preprocessor.random]
command = "python random.py"
```

## Configuring Renderers

### HTML renderer options

The HTML renderer has a couple of options as well. All the options for the
renderer need to be specified under the TOML table `[output.html]`.

The following configuration options are available:

- **theme:** mdBook comes with a default theme and all the resource files needed
  for it. But if this option is set, mdBook will selectively overwrite the theme
  files with the ones found in the specified folder.
- **default-theme:** The theme color scheme to select by default in the
  'Change Theme' dropdown. Defaults to `light`.
- **preferred-dark-theme:** The default dark theme. This theme will be used if
  the browser requests the dark version of the site via the
  ['prefers-color-scheme'](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme)
  CSS media query. Defaults to `navy`.
- **curly-quotes:** Convert straight quotes to curly quotes, except for those
  that occur in code blocks and code spans. Defaults to `false`.
- **mathjax-support:** Adds support for [MathJax](mathjax.md). Defaults to
  `false`.
- **copy-fonts:** Copies fonts.css and respective font files to the output directory and use them in the default theme. Defaults to `true`.
- **google-analytics:** If you use Google Analytics, this option lets you enable
  it by simply specifying your ID in the configuration file.
- **additional-css:** If you need to slightly change the appearance of your book
  without overwriting the whole style, you can specify a set of stylesheets that
  will be loaded after the default ones where you can surgically change the
  style.
- **additional-js:** If you need to add some behaviour to your book without
  removing the current behaviour, you can specify a set of JavaScript files that
  will be loaded alongside the default one.
- **print:** A subtable for configuration print settings. mdBook by default adds
  support for printing out the book as a single page. This is accessed using the
  print icon on the top right of the book.
- **no-section-label:** mdBook by defaults adds section label in table of
  contents column. For example, "1.", "2.1". Set this option to true to disable
  those labels. Defaults to `false`.
- **fold:** A subtable for configuring sidebar section-folding behavior.
- **playground:** A subtable for configuring various playground settings.
- **search:** A subtable for configuring the in-browser search functionality.
  mdBook must be compiled with the `search` feature enabled (on by default).
- **git-repository-url:**  A url to the git repository for the book. If provided
  an icon link will be output in the menu bar of the book.
- **git-repository-icon:** The FontAwesome icon class to use for the git
  repository link. Defaults to `fab-github`.
- **redirect:** A subtable used for generating redirects when a page is moved.
  The table contains key-value pairs where the key is where the redirect file
  needs to be created, as an absolute path from the build directory, (e.g.
  `/appendices/bibliography.html`). The value can be any valid URI the
  browser should navigate to (e.g. `https://rust-lang.org/`,
  `/overview.html`, or `../bibliography.html`).
- **input-404:** The name of the markdown file used for misssing files.
  The corresponding output file will be the same, with the extension replaced with `html`.
  Defaults to `404.md`.
- **site-url:** The url where the book will be hosted. This is required to ensure
  navigation links and script/css imports in the 404 file work correctly, even when accessing
  urls in subdirectories. Defaults to `/`.
- **cname:** The DNS subdomain or apex domain at which your book will be hosted.
  This string will be written to a file named CNAME in the root of your site, as
  required by GitHub Pages (see [*Managing a custom domain for your GitHub Pages
  site*][custom domain]).

[custom domain]: https://docs.github.com/en/github/working-with-github-pages/managing-a-custom-domain-for-your-github-pages-site

Available configuration options for the `[output.html.print]` table:

- **enable:** Enable print support. When `false`, all print support will not be
  rendered. Defaults to `true`.

Available configuration options for the `[output.html.fold]` table:

- **enable:** Enable section-folding. When off, all folds are open.
  Defaults to `false`.
- **level:** The higher the more folded regions are open. When level is 0, all
  folds are closed. Defaults to `0`.

Available configuration options for the `[output.html.playground]` table:

- **editable:** Allow editing the source code. Defaults to `false`.
- **copyable:** Display the copy button on code snippets. Defaults to `true`.
- **copy-js:** Copy JavaScript files for the editor to the output directory.
  Defaults to `true`.
- **line-numbers** Display line numbers on editable sections of code. Requires both `editable` and `copy-js` to be `true`. Defaults to `false`.

[Ace]: https://ace.c9.io/

Available configuration options for the `[output.html.search]` table:

- **enable:** Enables the search feature. Defaults to `true`.
- **limit-results:** The maximum number of search results. Defaults to `30`.
- **teaser-word-count:** The number of words used for a search result teaser.
  Defaults to `30`.
- **use-boolean-and:** Define the logical link between multiple search words. If
  true, all search words must appear in each result. Defaults to `false`.
- **boost-title:** Boost factor for the search result score if a search word
  appears in the header. Defaults to `2`.
- **boost-hierarchy:** Boost factor for the search result score if a search word
  appears in the hierarchy. The hierarchy contains all titles of the parent
  documents and all parent headings. Defaults to `1`.
- **boost-paragraph:** Boost factor for the search result score if a search word
  appears in the text. Defaults to `1`.
- **expand:** True if search should match longer results e.g. search `micro`
  should match `microwave`. Defaults to `true`.
- **heading-split-level:** Search results will link to a section of the document
  which contains the result. Documents are split into sections by headings this
  level or less. Defaults to `3`. (`### This is a level 3 heading`)
- **copy-js:** Copy JavaScript files for the search implementation to the output
  directory. Defaults to `true`.

This shows all available HTML output options in the **book.toml**:

```toml
[book]
title = "Example book"
authors = ["John Doe", "Jane Doe"]
description = "The example book covers examples."

[output.html]
theme = "my-theme"
default-theme = "light"
preferred-dark-theme = "navy"
curly-quotes = true
mathjax-support = false
copy-fonts = true
google-analytics = "UA-123456-7"
additional-css = ["custom.css", "custom2.css"]
additional-js = ["custom.js"]
no-section-label = false
git-repository-url = "https://github.com/rust-lang/mdBook"
git-repository-icon = "fab-github"
site-url = "/example-book/"
cname = "myproject.rs"
input-404 = "not-found.md"

[output.html.print]
enable = true

[output.html.fold]
enable = false
level = 0

[output.html.playground]
editable = false
copy-js = true
line-numbers = false

[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-boolean-and = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3
copy-js = true

[output.html.redirect]
"/appendices/bibliography.html" = "https://rustc-dev-guide.rust-lang.org/appendix/bibliography.html"
"/other-installation-methods.html" = "../infra/other-installation-methods.html"
```

### Markdown Renderer

The Markdown renderer will run preprocessors and then output the resulting
Markdown. This is mostly useful for debugging preprocessors, especially in
conjunction with `mdbook test` to see the Markdown that `mdbook` is passing
to `rustdoc`.

The Markdown renderer is included with `mdbook` but disabled by default.
Enable it by adding an empty table to your `book.toml` as follows:

```toml
[output.markdown]
```

There are no configuration options for the Markdown renderer at this time;
only whether it is enabled or disabled.

See [the preprocessors documentation](#configuring-preprocessors) for how to
specify which preprocessors should run before the Markdown renderer.

### Custom Renderers

A custom renderer can be enabled by adding a `[output.foo]` table to your
`book.toml`. Similar to [preprocessors](#configuring-preprocessors) this will
instruct `mdbook` to pass a representation of the book to `mdbook-foo` for
rendering. See the [alternative backends] chapter for more detail.

The custom renderer has access to all the fields within its table (i.e.
anything under `[output.foo]`). mdBook checks for two common fields:

- **command:** The command to execute for this custom renderer. Defaults to
  the name of the renderer with the `mdbook-` prefix (such as `mdbook-foo`).
- **optional:** If `true`, then the command will be ignored if it is not
  installed, otherwise mdBook will fail with an error. Defaults to `false`.

[alternative backends]: ../for_developers/backends.md

## Environment Variables

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
