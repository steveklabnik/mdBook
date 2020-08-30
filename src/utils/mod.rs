#![allow(missing_docs)] // FIXME: Document this

pub mod fs;
mod string;
pub(crate) mod toml_ext;
use crate::errors::Error;
use regex::Regex;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};

use std::borrow::Cow;
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub use self::string::{
    take_anchored_lines, take_lines, take_rustdoc_include_anchored_lines,
    take_rustdoc_include_lines,
};

/// Context for rendering markdown. This is used for fixing up links in the
/// output if one is missing in a translation.
#[derive(Clone, Debug)]
pub struct RenderMarkdownContext {
    /// Directory of the file being rendered, relative to the language's directory.
    /// If the file is "src/en/chapter/README.md", it is "chapter".
    pub path: PathBuf,
    /// Absolute path to the source directory of the book being rendered, across
    /// all languages.
    /// If the file is "src/en/chapter/README.md", it is "src/".
    pub src_dir: PathBuf,
    /// Language of the book being rendered.
    /// If the file is "src/en/chapter/README.md", it is "en".
    /// If the book is not multilingual, it is `None`.
    pub language: Option<String>,
    /// Fallback language to use if a link is missing. This is configured in
    /// `book.language` in the config.
    /// If the book is not multilingual, it is `None`.
    pub fallback_language: Option<String>,
    /// If true, prepend the parent path to the link.
    pub prepend_parent: bool,
}

lazy_static! {
    static ref SCHEME_LINK: Regex = Regex::new(r"^[a-z][a-z0-9+.-]*:").unwrap();
    static ref MD_LINK: Regex = Regex::new(r"(?P<link>.*)\.md(?P<anchor>#.*)?").unwrap();
}

/// Replaces multiple consecutive whitespace characters with a single space character.
pub fn collapse_whitespace(text: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s\s+").unwrap();
    }
    RE.replace_all(text, " ")
}

/// Convert the given string to a valid HTML element ID.
/// The only restriction is that the ID must not contain any ASCII whitespace.
pub fn normalize_id(content: &str) -> String {
    content
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
}

/// Generate an ID for use with anchors which is derived from a "normalised"
/// string.
pub fn id_from_content(content: &str) -> String {
    let mut content = content.to_string();

    // Skip any tags or html-encoded stuff
    const REPL_SUB: &[&str] = &[
        "<em>",
        "</em>",
        "<code>",
        "</code>",
        "<strong>",
        "</strong>",
        "&lt;",
        "&gt;",
        "&amp;",
        "&#39;",
        "&quot;",
    ];
    for sub in REPL_SUB {
        content = content.replace(sub, "");
    }

    // Remove spaces and hashes indicating a header
    let trimmed = content.trim().trim_start_matches('#').trim();

    normalize_id(trimmed)
}

fn rewrite_if_missing<P: AsRef<Path>>(
    fixed_link: &mut String,
    path_to_dest: P,
    dest: &str,
    src_dir: &PathBuf,
    language: &str,
    fallback_language: &str,
) {
    // We are inside a multilingual book.
    //
    // `fixed_link` is a string relative to the current language directory, like
    // "cli/README.md". Prepend the language's source directory (like "src/ja") and see
    // if the file exists.
    let mut path_on_disk = src_dir.clone();
    path_on_disk.push(language);
    path_on_disk.push(path_to_dest.as_ref());
    path_on_disk.push(dest);

    debug!("Checking if {} exists", path_on_disk.display());
    if !path_on_disk.exists() {
        // Now see if the file exists in the fallback language directory (like "src/en").
        let mut fallback_path = src_dir.clone();
        fallback_path.push(fallback_language);
        fallback_path.push(path_to_dest.as_ref());
        fallback_path.push(dest);

        debug!(
            "Not found, checking if fallback {} exists",
            fallback_path.display()
        );
        if fallback_path.exists() {
            // We can fall back to this link. Get enough parent directories to
            // reach the root source directory, append the fallback language
            // directory to it, the prepend the whole thing to the link.
            let mut relative_path = PathBuf::from(path_to_dest.as_ref());
            relative_path.push(dest);

            let mut path_to_fallback_src = fs::path_to_root(&relative_path);
            // One more parent directory out of language folder ("en")
            write!(path_to_fallback_src, "../{}/", fallback_language).unwrap();

            debug!(
                "Rewriting link to be under fallback: {}",
                path_to_fallback_src
            );
            fixed_link.insert_str(0, &path_to_fallback_src);
        }
    }
}

fn fix<'a>(dest: CowStr<'a>, ctx: Option<&RenderMarkdownContext>) -> CowStr<'a> {
    if dest.starts_with('#') {
        // Fragment-only link.
        if let Some(ctx) = ctx {
            if ctx.prepend_parent {
                let mut base = ctx.path.display().to_string();
                if base.ends_with(".md") {
                    base.replace_range(base.len() - 3.., ".html");
                }
                return format!("{}{}", base, dest).into();
            } else {
                return dest;
            }
        } else {
            return dest;
        }
    }
    // Don't modify links with schemes like `https`.
    if !SCHEME_LINK.is_match(&dest) {
        // This is a relative link, adjust it as necessary.
        let mut fixed_link = String::new();

        if let Some(ctx) = ctx {
            let base = ctx.path.parent().expect("path can't be empty");

            // If the book is multilingual, check if the file actually
            // exists, and if not rewrite the link to the fallback
            // language's page.
            if let Some(language) = &ctx.language {
                if let Some(fallback_language) = &ctx.fallback_language {
                    rewrite_if_missing(
                        &mut fixed_link,
                        &base,
                        &dest,
                        &ctx.src_dir,
                        &language,
                        &fallback_language,
                    );
                }
            }

            if ctx.prepend_parent {
                let base = base.to_str().expect("utf-8 paths only");
                if !base.is_empty() {
                    write!(fixed_link, "{}/", base).unwrap();
                }
            }
        }

        if let Some(caps) = MD_LINK.captures(&dest) {
            fixed_link.push_str(&caps["link"]);
            fixed_link.push_str(".html");
            if let Some(anchor) = caps.name("anchor") {
                fixed_link.push_str(anchor.as_str());
            }
        } else {
            fixed_link.push_str(&dest);
        };

        debug!("Fixed link: {:?}, {:?} => {:?}", dest, ctx, fixed_link);
        return CowStr::from(fixed_link);
    }
    dest
}

fn fix_html<'a>(html: CowStr<'a>, ctx: Option<&RenderMarkdownContext>) -> CowStr<'a> {
    // This is a terrible hack, but should be reasonably reliable. Nobody
    // should ever parse a tag with a regex. However, there isn't anything
    // in Rust that I know of that is suitable for handling partial html
    // fragments like those generated by pulldown_cmark.
    //
    // There are dozens of HTML tags/attributes that contain paths, so
    // feel free to add more tags if desired; these are the only ones I
    // care about right now.
    lazy_static! {
        static ref HTML_LINK: Regex =
            Regex::new(r#"(<(?:a|img) [^>]*?(?:src|href)=")([^"]+?)""#).unwrap();
    }

    HTML_LINK
        .replace_all(&html, move |caps: &regex::Captures<'_>| {
            let fixed = fix(caps[2].into(), ctx);
            format!("{}{}\"", &caps[1], fixed)
        })
        .into_owned()
        .into()
}

/// Fix links to the correct location.
///
/// This adjusts links, such as turning `.md` extensions to `.html`.
///
/// `path` is the path to the page being rendered relative to the root of the
/// book. This is used for the `print.html` page so that links on the print
/// page go to the original location. Normal page rendering sets `path` to
/// None. Ideally, print page links would link to anchors on the print page,
/// but that is very difficult.
fn adjust_links<'a>(event: Event<'a>, ctx: Option<&RenderMarkdownContext>) -> Event<'a> {
    match event {
        Event::Start(Tag::Link(link_type, dest, title)) => {
            Event::Start(Tag::Link(link_type, fix(dest, ctx), title))
        }
        Event::Start(Tag::Image(link_type, dest, title)) => {
            Event::Start(Tag::Image(link_type, fix(dest, ctx), title))
        }
        Event::Html(html) => Event::Html(fix_html(html, ctx)),
        _ => event,
    }
}

/// Wrapper around the pulldown-cmark parser for rendering markdown to HTML.
pub fn render_markdown(text: &str, curly_quotes: bool) -> String {
    render_markdown_with_path(text, curly_quotes, None)
}

pub fn new_cmark_parser(text: &str) -> Parser<'_> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    Parser::new_ext(text, opts)
}

pub fn render_markdown_with_path(
    text: &str,
    curly_quotes: bool,
    ctx: Option<&RenderMarkdownContext>,
) -> String {
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = new_cmark_parser(text);
    let mut converter = EventQuoteConverter::new(curly_quotes);
    let events = p
        .map(clean_codeblock_headers)
        .map(|event| adjust_links(event, ctx))
        .map(|event| converter.convert(event));

    html::push_html(&mut s, events);
    s
}

struct EventQuoteConverter {
    enabled: bool,
    convert_text: bool,
}

impl EventQuoteConverter {
    fn new(enabled: bool) -> Self {
        EventQuoteConverter {
            enabled,
            convert_text: true,
        }
    }

    fn convert<'a>(&mut self, event: Event<'a>) -> Event<'a> {
        if !self.enabled {
            return event;
        }

        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                self.convert_text = false;
                event
            }
            Event::End(Tag::CodeBlock(_)) => {
                self.convert_text = true;
                event
            }
            Event::Text(ref text) if self.convert_text => {
                Event::Text(CowStr::from(convert_quotes_to_curly(text)))
            }
            _ => event,
        }
    }
}

fn clean_codeblock_headers(event: Event<'_>) -> Event<'_> {
    match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info))) => {
            let info: String = info.chars().filter(|ch| !ch.is_whitespace()).collect();

            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from(info))))
        }
        _ => event,
    }
}

fn convert_quotes_to_curly(original_text: &str) -> String {
    // We'll consider the start to be "whitespace".
    let mut preceded_by_whitespace = true;

    original_text
        .chars()
        .map(|original_char| {
            let converted_char = match original_char {
                '\'' => {
                    if preceded_by_whitespace {
                        '‘'
                    } else {
                        '’'
                    }
                }
                '"' => {
                    if preceded_by_whitespace {
                        '“'
                    } else {
                        '”'
                    }
                }
                _ => original_char,
            };

            preceded_by_whitespace = original_char.is_whitespace();

            converted_char
        })
        .collect()
}

/// Prints a "backtrace" of some `Error`.
pub fn log_backtrace(e: &Error) {
    error!("Error: {}", e);

    for cause in e.chain().skip(1) {
        error!("\tCaused By: {}", cause);
    }
}

#[cfg(test)]
mod tests {
    mod render_markdown {
        use super::super::{fix, render_markdown, RenderMarkdownContext};

        #[test]
        fn preserves_external_links() {
            assert_eq!(
                render_markdown("[example](https://www.rust-lang.org/)", false),
                "<p><a href=\"https://www.rust-lang.org/\">example</a></p>\n"
            );
        }

        #[test]
        fn it_can_adjust_markdown_links() {
            assert_eq!(
                render_markdown("[example](example.md)", false),
                "<p><a href=\"example.html\">example</a></p>\n"
            );
            assert_eq!(
                render_markdown("[example_anchor](example.md#anchor)", false),
                "<p><a href=\"example.html#anchor\">example_anchor</a></p>\n"
            );

            // this anchor contains 'md' inside of it
            assert_eq!(
                render_markdown("[phantom data](foo.html#phantomdata)", false),
                "<p><a href=\"foo.html#phantomdata\">phantom data</a></p>\n"
            );
        }

        #[test]
        fn it_can_keep_quotes_straight() {
            assert_eq!(render_markdown("'one'", false), "<p>'one'</p>\n");
        }

        #[test]
        fn it_can_make_quotes_curly_except_when_they_are_in_code() {
            let input = r#"
'one'
```
'two'
```
`'three'` 'four'"#;
            let expected = r#"<p>‘one’</p>
<pre><code>'two'
</code></pre>
<p><code>'three'</code> ‘four’</p>
"#;
            assert_eq!(render_markdown(input, true), expected);
        }

        #[test]
        fn whitespace_outside_of_codeblock_header_is_preserved() {
            let input = r#"
some text with spaces
```rust
fn main() {
// code inside is unchanged
}
```
more text with spaces
"#;

            let expected = r#"<p>some text with spaces</p>
<pre><code class="language-rust">fn main() {
// code inside is unchanged
}
</code></pre>
<p>more text with spaces</p>
"#;
            assert_eq!(render_markdown(input, false), expected);
            assert_eq!(render_markdown(input, true), expected);
        }

        #[test]
        fn rust_code_block_properties_are_passed_as_space_delimited_class() {
            let input = r#"
```rust,no_run,should_panic,property_3
```
"#;

            let expected = r#"<pre><code class="language-rust,no_run,should_panic,property_3"></code></pre>
"#;
            assert_eq!(render_markdown(input, false), expected);
            assert_eq!(render_markdown(input, true), expected);
        }

        #[test]
        fn rust_code_block_properties_with_whitespace_are_passed_as_space_delimited_class() {
            let input = r#"
```rust,    no_run,,,should_panic , ,property_3
```
"#;

            let expected = r#"<pre><code class="language-rust,no_run,,,should_panic,,property_3"></code></pre>
"#;
            assert_eq!(render_markdown(input, false), expected);
            assert_eq!(render_markdown(input, true), expected);
        }

        #[test]
        fn rust_code_block_without_properties_has_proper_html_class() {
            let input = r#"
```rust
```
"#;

            let expected = r#"<pre><code class="language-rust"></code></pre>
"#;
            assert_eq!(render_markdown(input, false), expected);
            assert_eq!(render_markdown(input, true), expected);

            let input = r#"
```rust
```
"#;
            assert_eq!(render_markdown(input, false), expected);
            assert_eq!(render_markdown(input, true), expected);
        }

        use std::fs;
        use std::fs::File;
        use std::io::Write;
        use std::path::PathBuf;
        use tempfile;

        #[test]
        fn test_link_rewriting() {
            use pulldown_cmark::CowStr;

            let _ = env_logger::builder().is_test(true).try_init();
            let test = |dest, path, exists, expected| {
                let src_dir = tempfile::tempdir().unwrap();
                let path = PathBuf::from(path);

                let ctx = if exists {
                    Some(RenderMarkdownContext {
                        path: path,
                        src_dir: PathBuf::new(),
                        language: None,
                        fallback_language: None,
                        prepend_parent: false,
                    })
                } else {
                    let localized_dir = src_dir.path().join("ja");
                    fs::create_dir_all(&localized_dir).unwrap();

                    let fallback_dir = src_dir.path().join("en");
                    fs::create_dir_all(&fallback_dir).unwrap();

                    let chapter_path = fallback_dir.join(path.parent().unwrap()).join(dest);
                    fs::create_dir_all(chapter_path.parent().unwrap()).unwrap();
                    debug!("Create: {}", chapter_path.display());
                    File::create(&chapter_path)
                        .unwrap()
                        .write_all(b"# Chapter")
                        .unwrap();

                    Some(RenderMarkdownContext {
                        path: path,
                        src_dir: PathBuf::from(src_dir.path()),
                        language: Some(String::from("ja")),
                        fallback_language: Some(String::from("en")),
                        prepend_parent: false,
                    })
                };

                assert_eq!(
                    fix(CowStr::from(dest), ctx.as_ref()),
                    CowStr::from(expected)
                );
            };

            test("../b/summary.md", "a/index.md", true, "../b/summary.html");
            test(
                "../b/summary.md",
                "a/index.md",
                false,
                "../../en/../b/summary.html",
            );
            test("../c/summary.md", "a/b/index.md", true, "../c/summary.html");
            test(
                "../c/summary.md",
                "a/b/index.md",
                false,
                "../../../en/../c/summary.html",
            );
            test("#translations", "config.md", true, "#translations");
            test("#translations", "config.md", false, "#translations");
        }
    }

    mod html_munging {
        use super::super::{id_from_content, normalize_id};

        #[test]
        fn it_generates_anchors() {
            assert_eq!(
                id_from_content("## Method-call expressions"),
                "method-call-expressions"
            );
            assert_eq!(id_from_content("## **Bold** title"), "bold-title");
            assert_eq!(id_from_content("## `Code` title"), "code-title");
        }

        #[test]
        fn it_generates_anchors_from_non_ascii_initial() {
            assert_eq!(
                id_from_content("## `--passes`: add more rustdoc passes"),
                "--passes-add-more-rustdoc-passes"
            );
            assert_eq!(
                id_from_content("## 中文標題 CJK title"),
                "中文標題-cjk-title"
            );
            assert_eq!(id_from_content("## Über"), "Über");
        }

        #[test]
        fn it_normalizes_ids() {
            assert_eq!(
                normalize_id("`--passes`: add more rustdoc passes"),
                "--passes-add-more-rustdoc-passes"
            );
            assert_eq!(
                normalize_id("Method-call 🐙 expressions \u{1f47c}"),
                "method-call--expressions-"
            );
            assert_eq!(normalize_id("_-_12345"), "_-_12345");
            assert_eq!(normalize_id("12345"), "12345");
            assert_eq!(normalize_id("中文"), "中文");
            assert_eq!(normalize_id("にほんご"), "にほんご");
            assert_eq!(normalize_id("한국어"), "한국어");
            assert_eq!(normalize_id(""), "");
        }
    }

    mod convert_quotes_to_curly {
        use super::super::convert_quotes_to_curly;

        #[test]
        fn it_converts_single_quotes() {
            assert_eq!(convert_quotes_to_curly("'one', 'two'"), "‘one’, ‘two’");
        }

        #[test]
        fn it_converts_double_quotes() {
            assert_eq!(convert_quotes_to_curly(r#""one", "two""#), "“one”, “two”");
        }

        #[test]
        fn it_treats_tab_as_whitespace() {
            assert_eq!(convert_quotes_to_curly("\t'one'"), "\t‘one’");
        }
    }
}
