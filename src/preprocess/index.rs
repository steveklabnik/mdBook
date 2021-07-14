use core::ops::Range;
use pulldown_cmark::{Event, LinkType, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use regex::Regex;
use std::path::Path;

use crate::errors::*;

use super::{Preprocessor, PreprocessorContext};
use crate::book::{Book, BookItem};

/// A preprocessor for converting file name `README.md` to `index.md` since
/// `README.md` is the de facto index file in markdown-based documentation.
#[derive(Default)]
pub struct IndexPreprocessor;

impl IndexPreprocessor {
    pub(crate) const NAME: &'static str = "index";

    /// Create a new `IndexPreprocessor`.
    pub fn new() -> Self {
        IndexPreprocessor
    }
}

impl Preprocessor for IndexPreprocessor {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let source_dir = ctx.root.join(&ctx.config.book.src);
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                if let Some(ref mut path) = ch.path {
                    if is_readme_file(&path) {
                        let mut index_md = source_dir.join(path.with_file_name("index.md"));
                        if index_md.exists() {
                            warn_readme_name_conflict(&path, &&mut index_md);
                        }

                        path.set_file_name("index.md");
                    }
                }
                ch.content = replace_readme_in_string(&ch.content);
            }
        });

        Ok(book)
    }
}

fn replace_readme_in_string(content: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?ix)        #ignorecase, allow regex definition eXtended on multiple lines
            (/readme.md)    #part that will be replaced by index.md
            "
        )
        .unwrap();
    }
    let parser = pulldown_cmark::Parser::new(content);
    let mut buf = String::with_capacity(content.len());

    let mut events = vec![];
    for event in parser {
        events.push(match event {
            Event::Start(Tag::Link(link_type, dest, title)) => Event::Start(Tag::Link(
                link_type,
                pulldown_cmark::CowStr::from(RE.replace_all(&dest, "/index.md").into_owned()),
                title,
            )),
            Event::End(Tag::Link(link_type, dest, title)) => Event::End(Tag::Link(
                link_type,
                pulldown_cmark::CowStr::from(RE.replace_all(&dest, "/index.md").into_owned()),
                title,
            )),
            // Event::FootnoteReference(text) => Event::FootnoteReference(
            //     pulldown_cmark::CowStr::from(RE.replace_all(&text, "/index.md").into_owned()),
            // ),
            // Event::End(Tag::Link(link_type, dest, title)) => Event::End(Tag::Link(
            //     link_type,
            //     pulldown_cmark::CowStr::from(dbg!(RE.replace_all(&dest, "/index.md").into_owned())),
            //     title,
            // )),
            // pulldown_cmark::Event::Text(text)
            //     if (text.chars().nth(0).unwrap() == Char::from(":")) =>
            // {
            //     pulldown_cmark::Event::Text(pulldown_cmark::CowStr::from(dbg!(RE
            //         .replace_all(&text, "/index.md")
            //         .into_owned())))
            // },
            v => dbg!(v),
        });
    }

    // dbg!(
    // match
    cmark(events.iter(), &mut buf, None).unwrap() // .map(|_| buf)
                                             // {
                                             //     Ok(_) => ,
                                             //     Err(_) => content.to_string(),
                                             // }
    // )
    ;
    buf

    // //TODO : change
    // content.to_string()
}

fn warn_readme_name_conflict<P: AsRef<Path>>(readme_path: P, index_path: P) {
    let file_name = readme_path.as_ref().file_name().unwrap_or_default();
    let parent_dir = index_path
        .as_ref()
        .parent()
        .unwrap_or_else(|| index_path.as_ref());
    warn!(
        "It seems that there are both {:?} and index.md under \"{}\".",
        file_name,
        parent_dir.display()
    );
    warn!(
        "mdbook converts {:?} into index.html by default. It may cause",
        file_name
    );
    warn!("unexpected behavior if putting both files under the same directory.");
    warn!("To solve the warning, try to rearrange the book structure or disable");
    warn!("\"index\" preprocessor to stop the conversion.");
}

fn is_readme_file<P: AsRef<Path>>(path: P) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?i)^readme$").unwrap();
    }
    RE.is_match(
        path.as_ref()
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_stem_exactly_matches_readme_case_insensitively() {
        let path = "path/to/Readme.md";
        assert!(is_readme_file(path));

        let path = "path/to/README.md";
        assert!(is_readme_file(path));

        let path = "path/to/rEaDmE.md";
        assert!(is_readme_file(path));

        let path = "path/to/README.markdown";
        assert!(is_readme_file(path));

        let path = "path/to/README";
        assert!(is_readme_file(path));

        let path = "path/to/README-README.md";
        assert!(!is_readme_file(path));
    }

    #[test]
    fn do_not_replace_readme_in_normal_string_test() {
        let expected = "content: ( index.md)";

        let content = "content: ( Readme.md )";
        assert_ne!(expected, replace_readme_in_string(content));

        let content = "content: ( README.md )";
        assert_ne!(expected, replace_readme_in_string(content));

        let content = "content: ( rEaDmE.md )";
        assert_ne!(expected, replace_readme_in_string(content));

        let content = "content: ( README-README.md )";
        assert_ne!(expected, replace_readme_in_string(content));
    }

    //inspired by https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust
    macro_rules! replace_readme_tests {
        ($expected:expr,$($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let expected = $expected;
                let content= $value;
                assert_eq!(expected, replace_readme_in_string(content));
            }
        )*
        }
    }

    replace_readme_tests! {
        "[content](./bla/index.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_1:"[content](./bla/readme.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_2:"[content](./bla/rEaDme.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_3:"[content](./bla/README.MD) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_4:"[content](./bla/reAdme.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_5:"[content](./bla/Readme.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_6:"[content](./bla/README.md) content: ( ./readme.md)",
        replace_readme_only_in_link_cases_7:"[content](./bla/readme.MD) content: ( ./readme.md)",
    }
    replace_readme_tests! {
        "🤞🏼[content](./bla/index.md)🤞🏼\n[content]: ./index.md",
        replace_readme_only_in_link_cases_even_with_multibyte_chars:"🤞🏼[content](./bla/readme.md)🤞🏼\n[content]:  ./README.MD",
    }

    replace_readme_tests! {
        "[content]: ./bla/index.md ",
        replace_readme_in_reference_link_test_1:"[content]: ./bla/readme.md ",
        replace_readme_in_reference_link_test_2:"[content]: ./bla/ReAdme.md ",
        replace_readme_in_reference_link_test_3:"[content]: ./bla/ReaDme.md ",
        replace_readme_in_reference_link_test_4:"[content]: ./bla/README.MD ",
        replace_readme_in_reference_link_test_5:"[content]: ./bla/REadmE.md ",
        replace_readme_in_reference_link_test_6:"[content]: ./bla/ReAdme.md ",
        replace_readme_in_reference_link_test_7:"[content]: ./bla/Readme.MD ",
        replace_readme_in_reference_link_test_8:"[content]: ./bla/readme.MD ",
    }
    replace_readme_tests! {
        "[content](./bla/index.md)",
        replace_readme_in_inline_link_test_1:"[content]( ./bla/readme.md)",
        replace_readme_in_inline_link_test_2:"[content]( ./bla/ReAdme.md)",
        replace_readme_in_inline_link_test_3:"[content]( ./bla/ReaDme.md)",
        replace_readme_in_inline_link_test_4:"[content]( ./bla/README.MD)",
        replace_readme_in_inline_link_test_5:"[content]( ./bla/REadmE.md)",
        replace_readme_in_inline_link_test_6:"[content]( ./bla/ReAdme.md)",
        replace_readme_in_inline_link_test_7:"[content]( ./bla/Readme.MD)",
        replace_readme_in_inline_link_test_8:"[content]( ./bla/readme.MD)",
    }
}
