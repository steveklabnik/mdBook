# Configuring Renderers

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
- **mathjax-support:** Adds support for [MathJax](../mathjax.md). Defaults to
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
- **edit-url-template:** Edit url template, when provided shows a
  "Suggest an edit" button for directly jumping to editing the currently
  viewed page. For e.g. GitHub projects set this to
  `https://github.com/<owner>/<repo>/edit/master/{path}` or for
  Bitbucket projects set it to
  `https://bitbucket.org/<owner>/<repo>/src/master/{path}?mode=edit`
  where {path} will be replaced with the full path of the file in the
  repository.
- **redirect:** A subtable used for generating redirects when a page is moved.
  The table contains key-value pairs where the key is where the redirect file
  needs to be created, as an absolute path from the build directory, (e.g.
  `/appendices/bibliography.html`). The value can be any valid URI the
  browser should navigate to (e.g. `https://rust-lang.org/`,
  `/overview.html`, or `../bibliography.html`).
- **input-404:** The name of the markdown file used for missing files.
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
edit-url-template = "https://github.com/rust-lang/mdBook/edit/master/guide/{path}"
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

See [the preprocessors documentation](preprocessors.md) for how to
specify which preprocessors should run before the Markdown renderer.

### Custom Renderers

A custom renderer can be enabled by adding a `[output.foo]` table to your
`book.toml`. Similar to [preprocessors](preprocessors.md) this will
instruct `mdbook` to pass a representation of the book to `mdbook-foo` for
rendering. See the [alternative backends] chapter for more detail.

The custom renderer has access to all the fields within its table (i.e.
anything under `[output.foo]`). mdBook checks for two common fields:

- **command:** The command to execute for this custom renderer. Defaults to
  the name of the renderer with the `mdbook-` prefix (such as `mdbook-foo`).
- **optional:** If `true`, then the command will be ignored if it is not
  installed, otherwise mdBook will fail with an error. Defaults to `false`.

[alternative backends]: ../../for_developers/backends.md
