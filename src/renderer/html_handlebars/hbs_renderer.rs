use book::{Book, BookItem, Chapter};
use config::{Config, HtmlConfig, Playpen};
use errors::*;
use renderer::html_handlebars::helpers;
use renderer::{RenderContext, Renderer};
use theme::{self, playpen_editor, Theme};
use utils;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use regex::{Captures, Regex};
use serde_json;

#[derive(Default)]
pub struct HtmlHandlebars;

impl HtmlHandlebars {
    pub fn new() -> Self {
        HtmlHandlebars
    }

    fn render_item(
        &self,
        item: &BookItem,
        mut ctx: RenderItemContext,
        print_content: &mut String,
    ) -> Result<()> {
        // FIXME: This should rely less on mutable state
        if let BookItem::Chapter(ref ch) = *item {
            // "print.html" is used for the print page.
            if ch.path == Path::new("print.md") {
                bail!(ErrorKind::ReservedFilenameError(ch.path.clone()));
            };

            let path = ch
                .path
                .to_str()
                .chain_err(|| "Could not convert path to str")?;

            print_content.push_str(&self.render_print_content(
                &ch.content,
                &ch.path,
                &ctx.html_config,
            ));

            let content = ch.content.clone();
            let content = utils::render_markdown(&content, ctx.html_config.curly_quotes);
            ctx.data.insert("path".to_owned(), json!(path));
            ctx.data.insert("content".to_owned(), json!(content));
            ctx.data.insert("chapter_title".to_owned(), json!(ch.name));

            let title = self.get_title(&ctx.data, &ch.name);
            ctx.data.insert("title".to_owned(), json!(title));

            ctx.data.insert(
                "path_to_root".to_owned(),
                json!(utils::fs::path_to_root(&ch.path)),
            );

            // Render the handlebars template with the data
            debug!("Render template for {}", ch.path.display());
            self.render_content(&ctx, &Path::new(&ch.path).with_extension("html"))?;

            if ctx.is_index {
                ctx.data.insert("path".to_owned(), json!("index.html"));
                ctx.data.insert("path_to_root".to_owned(), json!(""));

                debug!("Creating index.html from {}", path);
                self.render_content(&ctx, &Path::new("index.html"))?;
            }
        }

        Ok(())
    }

    fn render_content(
        &self,
        ctx: &RenderItemContext,
        filepath: &Path
    ) -> Result <()> {
        let rendered = ctx.handlebars.render("index", &ctx.data)?;
        let rendered = self.post_process(rendered, &ctx.html_config.playpen);

        // Write to file
        debug!("Creating {}", filepath.display());
        utils::fs::write_file(&ctx.destination, &filepath, rendered.as_bytes())
    }

    fn render_print_content(
        &self,
        content: &String,
        path: &PathBuf,
        config: &HtmlConfig,
    ) -> String {
        let string_path = path.parent().unwrap().display().to_string();

        let fixed_content =
            utils::render_markdown_with_base(content, config.curly_quotes, &string_path);

        fixed_content
    }

    fn get_title(
        &self,
        render_data: &serde_json::Map<String, serde_json::Value>,
        chapter_name: &String,
    ) -> String {
        let book_title = render_data
            .get("book_title")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");

        let title = chapter_name.clone() + " - " + book_title;
        title
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::let_and_return))]
    fn post_process(&self, rendered: String, playpen_config: &Playpen) -> String {
        let rendered = build_header_links(&rendered);
        let rendered = fix_code_blocks(&rendered);
        let rendered = add_playpen_pre(&rendered, playpen_config);

        rendered
    }

    fn copy_static_files(
        &self,
        destination: &Path,
        theme: &Theme,
        html_config: &HtmlConfig,
    ) -> Result<()> {
        use utils::fs::write_file;

        write_file(
            destination,
            ".nojekyll",
            b"This file makes sure that Github Pages doesn't process mdBook's output.",
        )?;

        write_file(destination, "book.js", &theme.js)?;
        write_file(destination, "css/general.css", &theme.general_css)?;
        write_file(destination, "css/chrome.css", &theme.chrome_css)?;
        write_file(destination, "css/print.css", &theme.print_css)?;
        write_file(destination, "css/variables.css", &theme.variables_css)?;
        write_file(destination, "favicon.png", &theme.favicon)?;
        write_file(destination, "highlight.css", &theme.highlight_css)?;
        write_file(destination, "tomorrow-night.css", &theme.tomorrow_night_css)?;
        write_file(destination, "ayu-highlight.css", &theme.ayu_highlight_css)?;
        write_file(destination, "highlight.js", &theme.highlight_js)?;
        write_file(destination, "clipboard.min.js", &theme.clipboard_js)?;
        write_file(
            destination,
            "FontAwesome/css/font-awesome.css",
            theme::FONT_AWESOME,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.eot",
            theme::FONT_AWESOME_EOT,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.svg",
            theme::FONT_AWESOME_SVG,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.ttf",
            theme::FONT_AWESOME_TTF,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.woff",
            theme::FONT_AWESOME_WOFF,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/fontawesome-webfont.woff2",
            theme::FONT_AWESOME_WOFF2,
        )?;
        write_file(
            destination,
            "FontAwesome/fonts/FontAwesome.ttf",
            theme::FONT_AWESOME_TTF,
        )?;

        let playpen_config = &html_config.playpen;

        // Ace is a very large dependency, so only load it when requested
        if playpen_config.editable && playpen_config.copy_js {
            // Load the editor
            write_file(destination, "editor.js", playpen_editor::JS)?;
            write_file(destination, "ace.js", playpen_editor::ACE_JS)?;
            write_file(destination, "mode-rust.js", playpen_editor::MODE_RUST_JS)?;
            write_file(destination, "theme-dawn.js", playpen_editor::THEME_DAWN_JS)?;
            write_file(
                destination,
                "theme-tomorrow_night.js",
                playpen_editor::THEME_TOMORROW_NIGHT_JS,
            )?;
        }

        Ok(())
    }

    /// Update the context with data for this file
    fn configure_print_version(
        &self,
        data: &mut serde_json::Map<String, serde_json::Value>,
        print_content: &str,
    ) {
        // Make sure that the Print chapter does not display the title from
        // the last rendered chapter by removing it from its context
        data.remove("title");
        data.insert("is_print".to_owned(), json!(true));
        data.insert("path".to_owned(), json!("print.md"));
        data.insert("content".to_owned(), json!(print_content));
        data.insert(
            "path_to_root".to_owned(),
            json!(utils::fs::path_to_root(Path::new("print.md"))),
        );
    }

    fn register_hbs_helpers(&self, handlebars: &mut Handlebars, html_config: &HtmlConfig) {
        handlebars.register_helper(
            "toc",
            Box::new(helpers::toc::RenderToc {
                no_section_label: html_config.no_section_label,
            }),
        );
        handlebars.register_helper("previous", Box::new(helpers::navigation::previous));
        handlebars.register_helper("next", Box::new(helpers::navigation::next));
        handlebars.register_helper("theme_option", Box::new(helpers::theme::theme_option));
    }

    /// Copy across any additional CSS and JavaScript files which the book
    /// has been configured to use.
    fn copy_additional_css_and_js(
        &self,
        html: &HtmlConfig,
        root: &Path,
        destination: &Path,
    ) -> Result<()> {
        let custom_files = html.additional_css.iter().chain(html.additional_js.iter());

        debug!("Copying additional CSS and JS");

        for custom_file in custom_files {
            let input_location = root.join(custom_file);
            let output_location = destination.join(custom_file);
            if let Some(parent) = output_location.parent() {
                fs::create_dir_all(parent)
                    .chain_err(|| format!("Unable to create {}", parent.display()))?;
            }
            debug!(
                "Copying {} -> {}",
                input_location.display(),
                output_location.display()
            );

            fs::copy(&input_location, &output_location).chain_err(|| {
                format!(
                    "Unable to copy {} to {}",
                    input_location.display(),
                    output_location.display()
                )
            })?;
        }

        Ok(())
    }
}

// TODO(mattico): Remove some time after the 0.1.8 release
fn maybe_wrong_theme_dir(dir: &Path) -> Result<bool> {
    fn entry_is_maybe_book_file(entry: fs::DirEntry) -> Result<bool> {
        Ok(entry.file_type()?.is_file()
            && entry.path().extension().map_or(false, |ext| ext == "md"))
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            if entry_is_maybe_book_file(entry?).unwrap_or(false) {
                return Ok(false);
            }
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

impl Renderer for HtmlHandlebars {
    fn name(&self) -> &str {
        "html"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let html_config = ctx.config.html_config().unwrap_or_default();
        let src_dir = ctx.root.join(&ctx.config.book.src);
        let destination = &ctx.destination;
        let book = &ctx.book;

        trace!("render");
        let mut handlebars = Handlebars::new();

        let theme_dir = match html_config.theme {
            Some(ref theme) => theme.to_path_buf(),
            None => ctx.root.join("theme"),
        };

        if html_config.theme.is_none()
            && maybe_wrong_theme_dir(&src_dir.join("theme")).unwrap_or(false)
        {
            warn!(
                "Previous versions of mdBook erroneously accepted `./src/theme` as an automatic \
                 theme directory"
            );
            warn!("Please move your theme files to `./theme` for them to continue being used");
        }

        let theme = theme::Theme::new(theme_dir);

        debug!("Register the index handlebars template");
        handlebars.register_template_string("index", String::from_utf8(theme.index.clone())?)?;

        debug!("Register the header handlebars template");
        handlebars.register_partial("header", String::from_utf8(theme.header.clone())?)?;

        debug!("Register handlebars helpers");
        self.register_hbs_helpers(&mut handlebars, &html_config);

        let mut data = make_data(&ctx.root, &book, &ctx.config, &html_config)?;

        // Print version
        let mut print_content = String::new();

        fs::create_dir_all(&destination)
            .chain_err(|| "Unexpected error when constructing destination path")?;

        let mut is_index = true;
        for item in book.iter() {
            let ctx = RenderItemContext {
                handlebars: &handlebars,
                destination: destination.to_path_buf(),
                data: data.clone(),
                is_index,
                html_config: html_config.clone(),
            };
            self.render_item(item, ctx, &mut print_content)?;
            is_index = false;
        }

        // Print version
        self.configure_print_version(&mut data, &print_content);
        if let Some(ref title) = ctx.config.book.title {
            data.insert("title".to_owned(), json!(title));
        }

        // Render the handlebars template with the data
        debug!("Render template");
        let rendered = handlebars.render("index", &data)?;

        let rendered = self.post_process(rendered, &html_config.playpen);

        utils::fs::write_file(&destination, "print.html", rendered.as_bytes())?;
        debug!("Creating print.html ✓");

        debug!("Copy static files");
        self.copy_static_files(&destination, &theme, &html_config)
            .chain_err(|| "Unable to copy across static files")?;
        self.copy_additional_css_and_js(&html_config, &ctx.root, &destination)
            .chain_err(|| "Unable to copy across additional CSS and JS")?;

        // Render search index
        #[cfg(feature = "search")]
        {
            let search = html_config.search.unwrap_or_default();
            if search.enable {
                super::search::create_files(&search, &destination, &book)?;
            }
        }

        // Copy all remaining files
        utils::fs::copy_files_except_ext(&src_dir, &destination, true, &["md"])?;

        Ok(())
    }
}

fn make_data(
    root: &Path,
    book: &Book,
    config: &Config,
    html_config: &HtmlConfig,
) -> Result<serde_json::Map<String, serde_json::Value>> {
    trace!("make_data");
    let html = config.html_config().unwrap_or_default();

    let mut data = serde_json::Map::new();
    data.insert("language".to_owned(), json!("en"));
    data.insert(
        "book_title".to_owned(),
        json!(config.book.title.clone().unwrap_or_default()),
    );
    data.insert(
        "description".to_owned(),
        json!(config.book.description.clone().unwrap_or_default()),
    );
    data.insert("favicon".to_owned(), json!("favicon.png"));
    if let Some(ref livereload) = html_config.livereload_url {
        data.insert("livereload".to_owned(), json!(livereload));
    }

    let default_theme = match html_config.default_theme {
        Some(ref theme) => theme,
        None => "light",
    };
    data.insert("default_theme".to_owned(), json!(default_theme));

    // Add google analytics tag
    if let Some(ref ga) = config.html_config().and_then(|html| html.google_analytics) {
        data.insert("google_analytics".to_owned(), json!(ga));
    }

    if html.mathjax_support {
        data.insert("mathjax_support".to_owned(), json!(true));
    }

    // Add check to see if there is an additional style
    if !html.additional_css.is_empty() {
        let mut css = Vec::new();
        for style in &html.additional_css {
            match style.strip_prefix(root) {
                Ok(p) => css.push(p.to_str().expect("Could not convert to str")),
                Err(_) => css.push(style.to_str().expect("Could not convert to str")),
            }
        }
        data.insert("additional_css".to_owned(), json!(css));
    }

    // Add check to see if there is an additional script
    if !html.additional_js.is_empty() {
        let mut js = Vec::new();
        for script in &html.additional_js {
            match script.strip_prefix(root) {
                Ok(p) => js.push(p.to_str().expect("Could not convert to str")),
                Err(_) => js.push(script.to_str().expect("Could not convert to str")),
            }
        }
        data.insert("additional_js".to_owned(), json!(js));
    }

    if html.playpen.editable && html.playpen.copy_js {
        data.insert("playpen_js".to_owned(), json!(true));
    }

    let search = html_config.search.clone();
    if cfg!(feature = "search") {
        let search = search.unwrap_or_default();
        data.insert("search_enabled".to_owned(), json!(search.enable));
        data.insert(
            "search_js".to_owned(),
            json!(search.enable && search.copy_js),
        );
    } else if search.is_some() {
        warn!("mdBook compiled without search support, ignoring `output.html.search` table");
        warn!(
            "please reinstall with `cargo install mdbook --force --features search`to use the \
             search feature"
        )
    }

    if let Some(ref git_repository_url) = html_config.git_repository_url {
        data.insert("git_repository_url".to_owned(), json!(git_repository_url));
    }
    let git_repository_icon = match html_config.git_repository_icon {
        Some(ref git_repository_icon) => git_repository_icon,
        None => "fa-github",
    };
    data.insert("git_repository_icon".to_owned(), json!(git_repository_icon));

    let mut chapters = vec![];

    for item in book.iter() {
        // Create the data to inject in the template
        let mut chapter = BTreeMap::new();

        match *item {
            BookItem::Chapter(ref ch) => {
                if let Some(ref section) = ch.number {
                    chapter.insert("section".to_owned(), json!(section.to_string()));
                }

                chapter.insert("name".to_owned(), json!(ch.name));
                let path = ch
                    .path
                    .to_str()
                    .chain_err(|| "Could not convert path to str")?;
                chapter.insert("path".to_owned(), json!(path));
            }
            BookItem::Separator => {
                chapter.insert("spacer".to_owned(), json!("_spacer_"));
            }
        }

        chapters.push(chapter);
    }

    data.insert("chapters".to_owned(), json!(chapters));

    debug!("[*]: JSON constructed");
    Ok(data)
}

/// Goes through the rendered HTML, making sure all header tags are wrapped in
/// an anchor so people can link to sections directly.
fn build_header_links(html: &str) -> String {
    let regex = Regex::new(r"<h(\d)>(.*?)</h\d>").unwrap();
    let mut id_counter = HashMap::new();

    regex
        .replace_all(html, |caps: &Captures| {
            let level = caps[1]
                .parse()
                .expect("Regex should ensure we only ever get numbers here");

            wrap_header_with_link(level, &caps[2], &mut id_counter)
        })
        .into_owned()
}

/// Wraps a single header tag with a link, making sure each tag gets its own
/// unique ID by appending an auto-incremented number (if necessary).
fn wrap_header_with_link(
    level: usize,
    content: &str,
    id_counter: &mut HashMap<String, usize>,
) -> String {
    let raw_id = utils::id_from_content(content);

    let id_count = id_counter.entry(raw_id.clone()).or_insert(0);

    let id = match *id_count {
        0 => raw_id,
        other => format!("{}-{}", raw_id, other),
    };

    *id_count += 1;

    format!(
        r##"<a class="header" href="#{id}" id="{id}"><h{level}>{text}</h{level}></a>"##,
        level = level,
        id = id,
        text = content
    )
}

// The rust book uses annotations for rustdoc to test code snippets,
// like the following:
// ```rust,should_panic
// fn main() {
//     // Code here
// }
// ```
// This function replaces all commas by spaces in the code block classes
fn fix_code_blocks(html: &str) -> String {
    let regex = Regex::new(r##"<code([^>]+)class="([^"]+)"([^>]*)>"##).unwrap();
    regex
        .replace_all(html, |caps: &Captures| {
            let before = &caps[1];
            let classes = &caps[2].replace(",", " ");
            let after = &caps[3];

            format!(
                r#"<code{before}class="{classes}"{after}>"#,
                before = before,
                classes = classes,
                after = after
            )
        })
        .into_owned()
}

fn add_playpen_pre(html: &str, playpen_config: &Playpen) -> String {
    let regex = Regex::new(r##"((?s)<code[^>]?class="([^"]+)".*?>(.*?)</code>)"##).unwrap();
    regex
        .replace_all(html, |caps: &Captures| {
            let text = &caps[1];
            let classes = &caps[2];
            let code = &caps[3];

            if (classes.contains("language-rust")
                && !classes.contains("ignore")
                && !classes.contains("noplaypen"))
                || classes.contains("mdbook-runnable")
            {
                // wrap the contents in an external pre block
                if playpen_config.editable && classes.contains("editable")
                    || text.contains("fn main")
                    || text.contains("quick_main!")
                {
                    format!("<pre class=\"playpen\">{}</pre>", text)
                } else {
                    // we need to inject our own main
                    let (attrs, code) = partition_source(code);

                    format!(
                        "<pre class=\"playpen\"><code class=\"{}\">\n# \
                         #![allow(unused_variables)]\n{}#fn main() {{\n{}#}}</code></pre>",
                        classes, attrs, code
                    )
                }
            } else {
                // not language-rust, so no-op
                text.to_owned()
            }
        })
        .into_owned()
}

fn partition_source(s: &str) -> (String, String) {
    let mut after_header = false;
    let mut before = String::new();
    let mut after = String::new();

    for line in s.lines() {
        let trimline = line.trim();
        let header = trimline.chars().all(|c| c.is_whitespace()) || trimline.starts_with("#![");
        if !header || after_header {
            after_header = true;
            after.push_str(line);
            after.push_str("\n");
        } else {
            before.push_str(line);
            before.push_str("\n");
        }
    }

    (before, after)
}

struct RenderItemContext<'a> {
    handlebars: &'a Handlebars,
    destination: PathBuf,
    data: serde_json::Map<String, serde_json::Value>,
    is_index: bool,
    html_config: HtmlConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_build_header_links() {
        let inputs = vec![
            (
                "blah blah <h1>Foo</h1>",
                r##"blah blah <a class="header" href="#foo" id="foo"><h1>Foo</h1></a>"##,
            ),
            (
                "<h1>Foo</h1>",
                r##"<a class="header" href="#foo" id="foo"><h1>Foo</h1></a>"##,
            ),
            (
                "<h3>Foo^bar</h3>",
                r##"<a class="header" href="#foobar" id="foobar"><h3>Foo^bar</h3></a>"##,
            ),
            (
                "<h4></h4>",
                r##"<a class="header" href="#" id=""><h4></h4></a>"##,
            ),
            (
                "<h4><em>Hï</em></h4>",
                r##"<a class="header" href="#hï" id="hï"><h4><em>Hï</em></h4></a>"##,
            ),
            (
                "<h1>Foo</h1><h3>Foo</h3>",
                r##"<a class="header" href="#foo" id="foo"><h1>Foo</h1></a><a class="header" href="#foo-1" id="foo-1"><h3>Foo</h3></a>"##,
            ),
        ];

        for (src, should_be) in inputs {
            let got = build_header_links(&src);
            assert_eq!(got, should_be);
        }
    }

    struct PathTestContext<'a> {
        render_context: RenderItemContext<'a>,
        item: BookItem,
    }

    impl<'a> PathTestContext<'a> {
        pub fn new(path: String, dummy_handlebars: &'a Handlebars) -> PathTestContext<'a> {
            PathTestContext {
                render_context: RenderItemContext {
                    handlebars: dummy_handlebars,
                    destination: PathBuf::new(),
                    data: serde_json::from_str("{}").unwrap(),
                    is_index: false,
                    html_config: HtmlConfig {
                        ..Default::default()
                    },
                },
                item: BookItem::Chapter(Chapter {
                    path: PathBuf::from(path),
                    ..Default::default()
                }),
            }
        }
    }

    #[test]
    fn print_dot_md_is_reserved() {
        let dummy_handlebars = Handlebars::new();
        let ctx = PathTestContext::new(String::from("print.md"), &dummy_handlebars);
        let html_handlebars = HtmlHandlebars::new();

        let mut content = String::new();
        match html_handlebars.render_item(&ctx.item, ctx.render_context, &mut content) {
            Ok(_) => assert!(
                false,
                "Expected a failure, because print.md is a reserved filename"
            ),
            Err(error) => assert_eq!(error.to_string(), "print.md is reserved for internal use"),
        };
    }

    #[test]
    #[cfg(not(target_os = "windows"))] //The failure we're after does not occur on windows :(, on Linux it does.
    fn invalid_utf8_path_returns_error() {
        let mut invalid_unicode = String::from("AB");
        unsafe {
            let bytes = invalid_unicode.as_bytes_mut();
            bytes[0] = 0xC2;
            bytes[1] = 0xC2;
        }

        let dummy_handlebars = Handlebars::new();
        let ctx = PathTestContext::new(String::from(invalid_unicode), &dummy_handlebars);
        let html_handlebars = HtmlHandlebars::new();

        let mut content = String::new();
        match html_handlebars.render_item(&ctx.item, ctx.render_context, &mut content) {
            Ok(_) => assert!(
                false,
                "Expected a failure in PathBuf::to_str (for BookItem::Chapter::path)"
            ),
            Err(error) => assert_eq!(error.to_string(), "Could not convert path to str"),
        };
    }

    #[test]
    fn test_get_title() {
        let json: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str("{\"book_title\": \"Electric\"}").unwrap();
        let chapter_name = String::from("Froboz");

        let html_handlebars = HtmlHandlebars::new();
        let title = html_handlebars.get_title(&json, &chapter_name);
        assert_eq!("Froboz - Electric", title);
    }

    #[test]
    fn test_get_title_no_book_title() {
        let json: serde_json::Map<String, serde_json::Value> = serde_json::from_str("{}").unwrap();
        let chapter_name = String::from("Froboz");

        let html_handlebars = HtmlHandlebars::new();
        let title = html_handlebars.get_title(&json, &chapter_name);
        assert_eq!("Froboz - ", title); // Mmm, I'd ditch the " - " here
    }

    #[test]
    fn test_render_print_content() {
        let path = PathBuf::from("foobar.md");
        let content = String::from("# Awesome 'quotes'");
        let html_config = HtmlConfig {
            curly_quotes: false,
            ..Default::default()
        };

        let html_handlebars = HtmlHandlebars::new();
        assert_eq!(
            "<h1>Awesome 'quotes'</h1>\n",
            html_handlebars.render_print_content(&content, &path, &html_config)
        );
    }

    #[test]
    fn test_render_print_content_with_curly_quotes() {
        let path = PathBuf::from("foobar.md");
        let content = String::from("# Some curly 'quotes'?");
        let html_config = HtmlConfig {
            curly_quotes: true,
            ..Default::default()
        };

        let html_handlebars = HtmlHandlebars::new();
        assert_eq!(
            "<h1>Some curly ‘quotes’?</h1>\n",
            html_handlebars.render_print_content(&content, &path, &html_config)
        );
    }
}
