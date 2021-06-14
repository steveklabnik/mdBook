# Changelog

## mdBook 0.4.10
[2f7293a...dc2062a](https://github.com/rust-lang/mdBook/compare/2f7293a...dc2062a)

### Changed
- Reverted breaking change in 0.4.9 that removed the `__non_exhaustive` marker
  on the `Book` struct.
  [#1572](https://github.com/rust-lang/mdBook/pull/1572)
- Updated handlebars to 4.0.
  [#1550](https://github.com/rust-lang/mdBook/pull/1550)
- Removed the `chapter_begin` id on the print page's chapter separators.
  [#1541](https://github.com/rust-lang/mdBook/pull/1541)

## mdBook 0.4.9
[7e01cf9...d325c60](https://github.com/rust-lang/mdBook/compare/7e01cf9...d325c60)

### Changed
- Updated all dependencies and raised the minimum Rust version to 1.42.
  [#1528](https://github.com/rust-lang/mdBook/pull/1528)
- Added more detail to error message when a preprocessor fails.
  [#1526](https://github.com/rust-lang/mdBook/pull/1526)
- Set max-width of HTML video tags to 100% to match img tags.
  [#1542](https://github.com/rust-lang/mdBook/pull/1542)

### Fixed
- Type errors when parsing `book.toml` are no longer ignored.
  [#1539](https://github.com/rust-lang/mdBook/pull/1539)
- Better handling if `mdbook serve` fails to start the http server.
  [#1555](https://github.com/rust-lang/mdBook/pull/1555)
- Fixed the path for `edit-url-template` if the book used a source directory
  other than `src`.
  [#1554](https://github.com/rust-lang/mdBook/pull/1554)

## mdBook 0.4.8
[fcceee4...b592b10](https://github.com/rust-lang/mdBook/compare/fcceee4...b592b10)

### Added
- Added the option `output.html.edit-url-template` which can be a URL which is
  linked on each page to direct the user to a site (such as GitHub) where the
  user can directly suggest an edit for the page they are currently reading.
  [#1506](https://github.com/rust-lang/mdBook/pull/1506)

### Changed
- Printed output now includes a page break between chapters.
  [#1485](https://github.com/rust-lang/mdBook/pull/1485)

### Fixed
- HTML, such as HTML comments, is now ignored if it appears above the title line
  in `SUMMARY.md`.
  [#1437](https://github.com/rust-lang/mdBook/pull/1437)

## mdBook 0.4.7
[9a9eb01...c83bbd6](https://github.com/rust-lang/mdBook/compare/9a9eb01...c83bbd6)

### Changed
- Updated shlex parser to fix a minor parsing issue (used by the
  preprocessor/backend custom command config).
  [#1471](https://github.com/rust-lang/mdBook/pull/1471)
- Enhanced text contrast of `light` theme to improve accessibility.
  [#1470](https://github.com/rust-lang/mdBook/pull/1470)

### Fixed
- Fixed some issues with fragment scrolling and linking.
  [#1463](https://github.com/rust-lang/mdBook/pull/1463)

## mdBook 0.4.6
[eaa6914...1a0c296](https://github.com/rust-lang/mdBook/compare/eaa6914...1a0c296)

### Changed
- The chapter name is now included in the search breadcrumbs.
  [#1389](https://github.com/rust-lang/mdBook/pull/1389)
- Pressing Escape will remove the `?highlight` argument from the URL.
  [#1427](https://github.com/rust-lang/mdBook/pull/1427)
- `mdbook init --theme` will now place the theme in the root of the book
  directory instead of in the `src` directory.
  [#1432](https://github.com/rust-lang/mdBook/pull/1432)
- A custom renderer that sets the `command` to a relative path now interprets
  the relative path relative to the book root. Previously it was inconsistent
  based on the platform (either relative to the current directory, or relative
  to the renderer output directory). Paths relative to the output directory
  are still supported with a deprecation warning.
  [#1418](https://github.com/rust-lang/mdBook/pull/1418)
- The `theme` directory in the config is now interpreted as relative to the
  book root, instead of the current directory.
  [#1405](https://github.com/rust-lang/mdBook/pull/1405)
- Handle UTF-8 BOM for chapter sources.
  [#1285](https://github.com/rust-lang/mdBook/pull/1285)
- Removed extra whitespace added to `{{#playground}}` snippets.
  [#1375](https://github.com/rust-lang/mdBook/pull/1375)

### Fixed
- Clicking on a search result with multiple search words will now correctly
  highlight all of the words.
  [#1426](https://github.com/rust-lang/mdBook/pull/1426)
- Properly handle `<` and `>` characters in the table of contents.
  [#1376](https://github.com/rust-lang/mdBook/pull/1376)
- Fixed to properly serialize the `build` table in the config, which prevented
  setting it in the API.
  [#1378](https://github.com/rust-lang/mdBook/pull/1378)

## mdBook 0.4.5
[eaa6914...f66df09](https://github.com/rust-lang/mdBook/compare/eaa6914...f66df09)

### Fixed

- Fixed XSS in the search page.
  [CVE-2020-26297](https://groups.google.com/g/rustlang-security-announcements/c/3-sO6of29O0)
  [648c9ae](https://github.com/rust-lang/mdBook/commit/648c9ae772bec83f0a5954d17b4287d5bb1d6606)

## mdBook 0.4.4
[4df9ec9...01836ba](https://github.com/rust-lang/mdBook/compare/4df9ec9...01836ba)

### Added
- Added the `output.html.print.enable` configuration value to disable the
  "print" page.
  [#1169](https://github.com/rust-lang/mdBook/pull/1169)
- Added a list of supported languages for syntax-highlighting to the
  documentation.
  [#1345](https://github.com/rust-lang/mdBook/pull/1345)

### Fixed
- Now supports symbolic links for files in the `src` directory.
  [#1323](https://github.com/rust-lang/mdBook/pull/1323)

## mdBook 0.4.3
[9278b83...4df9ec9](https://github.com/rust-lang/mdBook/compare/9278b83...4df9ec9)

### Added
- Added `output.html.cname` option to emit a `CNAME` file which is used by
  GitHub Pages to know which domain is being used.
  [#1311](https://github.com/rust-lang/mdBook/pull/1311)

### Changed
- `mdbook test` no longer stops on the first test failure, but instead will
  run all the tests.
  [#1313](https://github.com/rust-lang/mdBook/pull/1313)
- Removed the `local` font source for Source Code Pro, as the locally
  installed font may not render properly on FireFox on macOS.
  [#1307](https://github.com/rust-lang/mdBook/pull/1307)

### Fixed
- Added newline to end of `.nojekyll` file.
  [#1310](https://github.com/rust-lang/mdBook/pull/1310)
- Fixed missing space before draft chapter titles.
  [#1309](https://github.com/rust-lang/mdBook/pull/1309)

## mdBook 0.4.2
[649f355...9278b83](https://github.com/rust-lang/mdBook/compare/649f355...9278b83)

### Changed
- The "show hidden lines" icon has changed from the "expand" icon to an "eye".
  [#1281](https://github.com/rust-lang/mdBook/pull/1281)
- Updated highlight.js. This adds several languages: c, c-like (effectively
  cpp), csharp (replaces cs), kotlin, less, lua, php-template, plaintext,
  python-repl, r, scss, typescript.
  [#1277](https://github.com/rust-lang/mdBook/pull/1277)

### Fixed
- Fixed SUMMARY links that contained newlines.
  [#1291](https://github.com/rust-lang/mdBook/pull/1291)
- Fixed SUMMARY links that contain `%20` spaces.
  [#1293](https://github.com/rust-lang/mdBook/pull/1293)
- Fixed favicon so that if only the png or svg is overridden, the other is not
  automatically included in the `<link>` tag.
  [#1272](https://github.com/rust-lang/mdBook/pull/1272)

## mdBook 0.4.1
[d4df7e7...649f355](https://github.com/rust-lang/mdBook/compare/d4df7e7...649f355)

### Changed
- Removed several outdated dev-dependencies.
  [#1267](https://github.com/rust-lang/mdBook/pull/1267)

### Fixed
- Fixed sidebar scrolling if the book includes part titles.
  [#1265](https://github.com/rust-lang/mdBook/pull/1265)
- Don't include the default favicon if only one of the PNG or SVG is overridden.
  [#1266](https://github.com/rust-lang/mdBook/pull/1266)

## mdBook 0.4.0
[99ecd4f...d4df7e7](https://github.com/rust-lang/mdBook/compare/99ecd4f...d4df7e7)

### Breaking Changes
- Several of the changes in the release have altered the public API of the
  mdbook library.
- Many dependencies have been updated or replaced.
  This also removes the `--websocket-hostname` and `--websocket-port` from
  the `serve` command.
  [#1211](https://github.com/rust-lang/mdBook/pull/1211)
- A new "404" page is now automatically rendered. This requires knowledge of
  the base URL of your site to work properly. If you decide to use this as
  your 404 page, you should set the `site-url` setting in the book
  configuration so mdbook can generate the links correctly. Alternatively you
  can disable the 404 page generation, or set up your own 404 handling in your
  web server.
  [#1221](https://github.com/rust-lang/mdBook/pull/1221)
- The `debug` and `output` features have been removed as they were unused.
  [#1211](https://github.com/rust-lang/mdBook/pull/1211)
- If you are using customized themes, you may want to consider setting the
  `preferred-dark-theme` config setting, as it now defaults to "navy".
  [#1199](https://github.com/rust-lang/mdBook/pull/1199)
- "Playpen" has been renamed to "playground". This is generally backwards
  compatible for users, but `{{#playpen}}` will now display warnings. This may
  impact books that have modified the "playpen" elements in the theme.
  [#1241](https://github.com/rust-lang/mdBook/pull/1241)
- If a renderer is not installed, it is now treated as an error. If you want
  the old behavior of ignoring missing renderers, set the `optional` setting
  for that renderer.
  [#1122](https://github.com/rust-lang/mdBook/pull/1122)
- If you have a custom favicon, you may need to look into adding an SVG
  version, otherwise the default SVG icon will be displayed.
  [#1230](https://github.com/rust-lang/mdBook/pull/1230)

### Added
- Added a new `[rust]` configuration section to `book.toml`, which allows
  setting the default edition with `edition = "2018"`.
  [#1163](https://github.com/rust-lang/mdBook/pull/1163)
- Renderers can now be marked as `optional`, so that they will be ignored if
  the renderer is not installed.
  [#1122](https://github.com/rust-lang/mdBook/pull/1122)
- Added `head.hbs` to allow adding content to the `<head>` section in HTML.
  [#1206](https://github.com/rust-lang/mdBook/pull/1206)
- Added "draft chapters". These are chapters listed without a link to indicate
  content yet to be written.
  [#1153](https://github.com/rust-lang/mdBook/pull/1153)
- Added "parts" to split a book into different sections. Headers can be added
  to `SUMMARY.md` to signify different sections.
  [#1171](https://github.com/rust-lang/mdBook/pull/1171)
- Added generation of a "404" page for handling missing pages and broken links.
  [#1221](https://github.com/rust-lang/mdBook/pull/1221)
- Added configuration section for specifying URL redirects.
  [#1237](https://github.com/rust-lang/mdBook/pull/1237)
- Added an SVG favicon that works with light and dark colors schemes.
  [#1230](https://github.com/rust-lang/mdBook/pull/1230)

### Changed
- Changed default Rust attribute of `allow(unused_variables)` to `allow(unused)`.
  [#1195](https://github.com/rust-lang/mdBook/pull/1195)
- Fonts are now served locally instead of from the Google Fonts CDN. The
  `copy-fonts` option was added to disable this if you want to supply your own
  fonts.
  [#1188](https://github.com/rust-lang/mdBook/pull/1188)
- Switched the built-in webserver for the `serve` command to a new
  implementation. This results in some internal differences in how websockets
  are handled, which removes the separate websocket options. This should also
  make it easier to serve multiple books at once.
  [#1211](https://github.com/rust-lang/mdBook/pull/1211)
- The default dark theme is now "navy".
  [#1199](https://github.com/rust-lang/mdBook/pull/1199)
- "Playpen" has been renamed to "playground", matching the actual name of the
  service which was renamed many years ago.
  [#1241](https://github.com/rust-lang/mdBook/pull/1241)

### Fixed
- Links with the `+` symbol should now work.
  [#1208](https://github.com/rust-lang/mdBook/pull/1208)
- The `MDBOOK_BOOK` environment variable now correctly allows overriding the
  entire book configuration.
  [#1207](https://github.com/rust-lang/mdBook/pull/1207)
- The sidebar can no longer be dragged outside of the window.
  [#1229](https://github.com/rust-lang/mdBook/pull/1229)
- Hide the Rust Playground "play" button for `no_run` code samples.
  [#1249](https://github.com/rust-lang/mdBook/pull/1249)
- Fixed the `--dest-dir` command-line option for the `serve` and `watch`
  commands.
  [#1228](https://github.com/rust-lang/mdBook/pull/1228)
- Hotkey handlers are now disabled in `text` input fields (for example, typing
  `S` in a custom text input field).
  [#1244](https://github.com/rust-lang/mdBook/pull/1244)

## mdBook 0.3.7
[88684d8...99ecd4f](https://github.com/rust-lang/mdBook/compare/88684d8...99ecd4f)

### Changed
- Code spans in headers are no longer highlighted as code.
  [#1162](https://github.com/rust-lang/mdBook/pull/1162)
- The sidebar will now scroll the activate page to the middle instead of the top.
  [#1161](https://github.com/rust-lang/mdBook/pull/1161)
- Reverted change to reject build output within the `src` directory, and
  instead add a check that prevents infinite copies.
  [#1181](https://github.com/rust-lang/mdBook/pull/1181)
  [#1026](https://github.com/rust-lang/mdBook/pull/1026)

### Fixed
- Fixed sidebar line-height jumping for collapsed chapters.
  [#1182](https://github.com/rust-lang/mdBook/pull/1182)
- Fixed theme selector focus.
  [#1170](https://github.com/rust-lang/mdBook/pull/1170)

## mdBook 0.3.6
[efdb832...88684d8](https://github.com/rust-lang/mdBook/compare/efdb832...88684d8)

### Added
- `MDBook::execute_build_process` is now publicly accessible in the API so
  that plugins can more easily initiate the build process.
  [#1099](https://github.com/rust-lang/mdBook/pull/1099)

### Changed
- Use a different color for Ayu theme's highlighting for Rust attributes (uses
  a bright color instead of the comment color).
  [#1133](https://github.com/rust-lang/mdBook/pull/1133)
- Adjusted spacing of sidebar entries.
  [#1137](https://github.com/rust-lang/mdBook/pull/1137)
- Slightly adjusted line-height of `<p>`, `<ul>`, and `<ol>`.
  [#1136](https://github.com/rust-lang/mdBook/pull/1136)
- Handlebars updated to 3.0.
  [#1130](https://github.com/rust-lang/mdBook/pull/1130)

### Fixed
- Fix an issue with sidebar scroll position on reload.
  [#1108](https://github.com/rust-lang/mdBook/pull/1108)
- `mdbook serve` will retain the current scroll position when the page is reloaded.
  [#1097](https://github.com/rust-lang/mdBook/pull/1097)
- Fixed the page name if the book didn't have a title to not be prefixed with ` - `.
  [#1145](https://github.com/rust-lang/mdBook/pull/1145)
- HTML attributes `rel=next` and `rel=previous` are now supported in "wide"
  mode (previously they were only set in narrow mode).
  [#1150](https://github.com/rust-lang/mdBook/pull/1150)
- Prevent recursive copies when the destination directory is contained in the
  source directory.
  [#1135](https://github.com/rust-lang/mdBook/pull/1135)
- Adjusted the menu bar animation to not immediately obscure the top content.
  [#989](https://github.com/rust-lang/mdBook/pull/989)
- Fix for comments in SUMMARY.md that appear between items.
  [#1167](https://github.com/rust-lang/mdBook/pull/1167)

## mdBook 0.3.5
[6e0d0fa...efdb832](https://github.com/rust-lang/mdBook/compare/6e0d0fa...efdb832)

### Changed
- The `default-theme` config setting is now case-insensitive.
  [#1079](https://github.com/rust-lang/mdBook/pull/1079)

### Fixed
- Fixed `#` hidden Rust code lines not rendering properly.
  [#1088](https://github.com/rust-lang/mdBook/pull/1088)
- Updated pulldown-cmark to 0.6.1, fixing several issues.
  [#1021](https://github.com/rust-lang/mdBook/pull/1021)

## mdBook 0.3.4
[e5f77aa...6e0d0fa](https://github.com/rust-lang/mdBook/compare/e5f77aa...6e0d0fa)

### Changed
- Switch to relative `rem` font sizes from `px`.
  [#894](https://github.com/rust-lang/mdBook/pull/894)
- Migrated repository to https://github.com/rust-lang/mdBook/
  [#1083](https://github.com/rust-lang/mdBook/pull/1083)

## mdBook 0.3.3
[2b649fe...e5f77aa](https://github.com/rust-lang/mdBook/compare/2b649fe...e5f77aa)

### Changed
- Improvements to the automatic dark theme selection.
  [#1069](https://github.com/rust-lang/mdBook/pull/1069)
- Fragment links now prevent scrolling the header behind the menu bar.
  [#1077](https://github.com/rust-lang/mdBook/pull/1077)

### Fixed
- Fixed error when building a book that has a spacer immediately after the
  first chapter.
  [#1075](https://github.com/rust-lang/mdBook/pull/1075)

## mdBook 0.3.2
[9cd47eb...2b649fe](https://github.com/rust-lang/mdBook/compare/9cd47eb...2b649fe)

### Added
- Added a markdown renderer, which is off by default. This may be useful for
  debugging preprocessors.
  [#1018](https://github.com/rust-lang/mdBook/pull/1018)
- Code samples may now include line numbers with the
  `output.html.playpen.line-numbers` configuration value.
  [#1035](https://github.com/rust-lang/mdBook/pull/1035)
- The `watch` and `serve` commands will now ignore files listed in
  `.gitignore`.
  [#1044](https://github.com/rust-lang/mdBook/pull/1044)
- Added automatic dark-theme detection based on the CSS `prefers-color-scheme`
  feature. This may be enabled by setting `output.html.preferred-dark-theme`
  to your preferred dark theme.
  [#1037](https://github.com/rust-lang/mdBook/pull/1037)
- Added `rustdoc_include` preprocessor. This makes it easier to include
  portions of an external Rust source file. The rest of the file is hidden,
  but the user may expand it to see the entire file, and will continue to work
  with `mdbook test`.
  [#1003](https://github.com/rust-lang/mdBook/pull/1003)
- Added Ctrl-Enter shortcut to the playpen editor to automatically run the
  sample.
  [#1066](https://github.com/rust-lang/mdBook/pull/1066)
- Added `output.html.playpen.copyable` configuration option to disable
  the copy button.
  [#1050](https://github.com/rust-lang/mdBook/pull/1050)
- Added ability to dynamically expand and fold sections within the sidebar.
  See the `output.html.fold` configuration to enable this feature.
  [#1027](https://github.com/rust-lang/mdBook/pull/1027)

### Changed
- Use standard `scrollbar-color` CSS along with webkit extension
  [#816](https://github.com/rust-lang/mdBook/pull/816)
- The renderer build directory is no longer deleted before the renderer is
  run. This allows a backend to cache results between runs.
  [#985](https://github.com/rust-lang/mdBook/pull/985)
- Next/prev links now highlight on hover to indicate it is clickable.
  [#994](https://github.com/rust-lang/mdBook/pull/994)
- Increase padding of table headers.
  [#824](https://github.com/rust-lang/mdBook/pull/824)
- Errors in `[output.html]` config are no longer ignored.
  [#1033](https://github.com/rust-lang/mdBook/pull/1033)
- Updated highlight.js for syntax highlighting updates (primarily to add
  async/await to Rust highlighting).
  [#1041](https://github.com/rust-lang/mdBook/pull/1041)
- Raised minimum supported rust version to 1.35.
  [#1003](https://github.com/rust-lang/mdBook/pull/1003)
- Hidden code lines are no longer dynamically removed via JavaScript, but
  instead managed with CSS.
  [#846](https://github.com/rust-lang/mdBook/pull/846)
  [#1065](https://github.com/rust-lang/mdBook/pull/1065)
- Changed the default font set for the ACE editor, giving preference to
  "Source Code Pro".
  [#1062](https://github.com/rust-lang/mdBook/pull/1062)
- Windows 32-bit releases are no longer published.
  [#1071](https://github.com/rust-lang/mdBook/pull/1071)

### Fixed
- Fixed sidebar auto-scrolling.
  [#1052](https://github.com/rust-lang/mdBook/pull/1052)
- Fixed error message when running `clean` multiple times.
  [#1055](https://github.com/rust-lang/mdBook/pull/1055)
- Actually fix the "next" link on index.html. The previous fix didn't work.
  [#1005](https://github.com/rust-lang/mdBook/pull/1005)
- Stop using `inline-block` for `inline code`, fixing selection highlighting
  and some rendering issues.
  [#1058](https://github.com/rust-lang/mdBook/pull/1058)
- Fix header auto-hide on browsers with momentum scrolling that allows
  negative `scrollTop`.
  [#1070](https://github.com/rust-lang/mdBook/pull/1070)

## mdBook 0.3.1
[69a08ef...9cd47eb](https://github.com/rust-lang/mdBook/compare/69a08ef...9cd47eb)

### Added
- 🔥 Added ability to include files using anchor points instead of line numbers.
  [#851](https://github.com/rust-lang/mdBook/pull/851)
- Added `language` configuration value to set the language of the book, which
  will affect things like the `<html lang="en">` tag.
  [#941](https://github.com/rust-lang/mdBook/pull/941)

### Changed
- Updated to handlebars 2.0.
  [#977](https://github.com/rust-lang/mdBook/pull/977)

### Fixed
- Fixed memory leak warning.
  [#967](https://github.com/rust-lang/mdBook/pull/967)
- Fix more print.html links.
  [#963](https://github.com/rust-lang/mdBook/pull/963)
- Fixed crash on some unicode input.
  [#978](https://github.com/rust-lang/mdBook/pull/978)

## mdBook 0.3.0
[6cbc41d...69a08ef](https://github.com/rust-lang/mdBook/compare/6cbc41d...69a08ef)

### Added
- Added ability to resize the sidebar.
  [#849](https://github.com/rust-lang/mdBook/pull/849)
- Added `load_with_config_and_summary` function to `MDBook` to be able to
  build a book with a custom `Summary`.
  [#883](https://github.com/rust-lang/mdBook/pull/883)
- Set `noindex` on `print.html` page to prevent robots from indexing it.
  [#844](https://github.com/rust-lang/mdBook/pull/844)
- Added support for ~~strikethrough~~ and GitHub-style tasklists.
  [#952](https://github.com/rust-lang/mdBook/pull/952)

### Changed
- Command-line help output is now colored.
  [#861](https://github.com/rust-lang/mdBook/pull/861)
- The build directory is now deleted before rendering starts, instead of after
  if finishes.
  [#878](https://github.com/rust-lang/mdBook/pull/878)
- Removed dependency on `same-file` crate.
  [#903](https://github.com/rust-lang/mdBook/pull/903)
- 💥 Renamed `with_preprecessor` to `with_preprocessor`.
  [#906](https://github.com/rust-lang/mdBook/pull/906)
- Updated ACE editor to 1.4.4, should remove a JavaScript console warning.
  [#935](https://github.com/rust-lang/mdBook/pull/935)
- Dependencies have been updated.
  [#934](https://github.com/rust-lang/mdBook/pull/934)
  [#945](https://github.com/rust-lang/mdBook/pull/945)
- Highlight.js has been updated. This fixes some TOML highlighting, and adds
  Julia support.
  [#942](https://github.com/rust-lang/mdBook/pull/942)
- 🔥 Updated to pulldown-cmark 0.5. This may have significant changes to the
  formatting of existing books, as the newer version has more accurate
  interpretation of the CommonMark spec and a large number of bug fixes and
  changes.
  [#898](https://github.com/rust-lang/mdBook/pull/898)
- The `diff` language should now highlight correctly.
  [#943](https://github.com/rust-lang/mdBook/pull/943)
- Make the blank region of a header not clickable.
  [#948](https://github.com/rust-lang/mdBook/pull/948)
- Rustdoc tests now use the preprocessed content instead of the raw,
  unpreprocessed content.
  [#891](https://github.com/rust-lang/mdBook/pull/891)

### Fixed
- Fixed file change detection so that `mdbook serve` only reloads once when
  multiple files are changed at once.
  [#870](https://github.com/rust-lang/mdBook/pull/870)
- Fixed on-hover color highlighting for links in sidebar.
  [#834](https://github.com/rust-lang/mdBook/pull/834)
- Fixed loss of focus when clicking the "Copy" button in code blocks.
  [#867](https://github.com/rust-lang/mdBook/pull/867)
- Fixed incorrectly stripping the path for `additional-js` files.
  [#796](https://github.com/rust-lang/mdBook/pull/796)
- Fixed color of `code spans` that are links.
  [#905](https://github.com/rust-lang/mdBook/pull/905)
- Fixed "next" navigation on index.html.
  [#916](https://github.com/rust-lang/mdBook/pull/916)
- Fixed keyboard chapter navigation for `file` urls.
  [#915](https://github.com/rust-lang/mdBook/pull/915)
- Fixed bad wrapping for inline code on some browsers.
  [#818](https://github.com/rust-lang/mdBook/pull/818)
- Properly load an existing `SUMMARY.md` in `mdbook init`.
  [#841](https://github.com/rust-lang/mdBook/pull/841)
- Fixed some broken links in `print.html`.
  [#871](https://github.com/rust-lang/mdBook/pull/871)
- The Rust Playground link now supports the 2018 edition.
  [#946](https://github.com/rust-lang/mdBook/pull/946)

## mdBook 0.2.3 (2018-01-18)
[2c20c99...6cbc41d](https://github.com/rust-lang/mdBook/compare/2c20c99...6cbc41d)

### Added
- Added an optional button to the top of the page which will link to a git
  repository. Use the `git-repository-url` and `git-repository-icon` options
  in the `[output.html]` section to enable it and set its appearance.
  [#802](https://github.com/rust-lang/mdBook/pull/802)
- Added a `default-theme` option to the `[output.html]` section.
  [#804](https://github.com/rust-lang/mdBook/pull/804)

### Changed
- 💥 Header ID anchors no longer add an arbitrary `a` character for headers
  that start with a non-ascii-alphabetic character.
  [#788](https://github.com/rust-lang/mdBook/pull/788)

### Fixed
- Fix websocket hostname usage
  [#865](https://github.com/rust-lang/mdBook/pull/865)
- Fixing links in print.html
  [#866](https://github.com/rust-lang/mdBook/pull/866)

## mdBook 0.2.2 (2018-10-19)
[7e2e095...2c20c99](https://github.com/rust-lang/mdBook/compare/7e2e095...2c20c99)

### Added
- 🎉 Process-based custom preprocessors. See [the
  docs](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html)
  for more.
  [#792](https://github.com/rust-lang/mdBook/pull/792)

- 🎉 Configurable preprocessors.

  Added `build.use-default-preprocessors` boolean TOML key to allow disabling
  the built-in `links` and `index` preprocessors.

  Added `[preprocessor]` TOML tables to configure each preprocessor.

  Specifying `[preprocessor.links]` or `[preprocessor.index]` will enable the
  respective built-in preprocessor if `build.use-default-preprocessors` is
  `false`.

  Added `fn supports_renderer(&self, renderer: &str) -> bool` to the
  `Preprocessor` trait to specify if the preprocessor supports the given
  renderer. The default implementation always returns `true`.

  `Preprocessor::run` now takes a book by value instead of a mutable
  reference. It should return a `Book` value with the intended modifications.

  Added `PreprocessorContext::renderer` to indicate the renderer being used.

  [#658](https://github.com/rust-lang/mdBook/pull/658)
  [#787](https://github.com/rust-lang/mdBook/pull/787)

### Fixed
- Fix paths to additional CSS and JavaScript files
  [#777](https://github.com/rust-lang/mdBook/pull/777)
- Ensure section numbers are correctly incremented after a horizontal
  separator
  [#790](https://github.com/rust-lang/mdBook/pull/790)

## mdBook 0.2.1 (2018-08-22)
[91ffca1...7e2e095](https://github.com/rust-lang/mdBook/compare/91ffca1...7e2e095)

### Changed
- Update to handlebars-rs 1.0
  [#761](https://github.com/rust-lang/mdBook/pull/761)

### Fixed
- Fix table colors, broken by Stylus -> CSS transition
  [#765](https://github.com/rust-lang/mdBook/pull/765)

## mdBook 0.2.0 (2018-08-02)

### Changed
- 💥 This release changes how links are handled in mdBook. Previously, relative
  links were interpreted relative to the book's root. In `0.2.0`+ links are
  relative to the page they are in, and use the `.md` extension. This has [several
  advantages](https://github.com/rust-lang/mdBook/pull/603#issue-166701447),
  such as making links work in other markdown viewers like GitHub. You will
  likely have to change links in your book to accommodate this change. For
  example, a book with this layout:

  ```
  chapter_1/
      section_1.md
      section_2.md
  SUMMARY.md
  ```

  Previously a link in `section_1.md` to `section_2.md` would look like this:
  ```markdown
  [section_2](chapter_1/section_2.html)
  ```

  Now it must be changed to this:
  ```markdown
  [section_2](section_2.md)
  ```

- 💥 `mdbook test --library-path` now accepts a comma-delimited list of
  arguments rather than taking all following arguments. This makes it easier
  to handle the trailing book directory argument without always needing to put
  ` -- ` before it. Multiple instances of the option continue to be accepted:
  `mdbook test -L foo -L bar`.

- 💥 `mdbook serve` has some of its options renamed for clarity. See `mdbook
  help serve` for details.

- Embedded rust playpens now use the "stable" playground API.
  [#754](https://github.com/rust-lang/mdBook/pull/754)

### Fixed
- Escaped includes (`\{{#include file.rs}}`) will now render correctly.
  [f30ce01](https://github.com/rust-lang/mdBook/commit/f30ce0184d71e342141145472bf816419d30a2c5)
- `index.html` will now render correctly when the book's first section is
  inside a subdirectory.
  [#756](https://github.com/rust-lang/mdBook/pull/756)
