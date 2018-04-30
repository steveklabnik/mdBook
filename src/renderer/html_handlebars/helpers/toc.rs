use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

use serde_json;
use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError};
use pulldown_cmark::{html, Event, Parser, Tag};

// Handlebars helper to construct TOC
pub struct RenderToc {
    pub no_section_label: bool,
    pub rewrite_to_dir: Vec<String>,
}

impl HelperDef for RenderToc {
    fn call(&self, _h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        // get value from context data
        // rc.get_path() is current json parent path, you should always use it like this
        // param is the key of value you want to display
        let chapters = rc.evaluate_absolute("chapters", true).and_then(|c| {
            serde_json::value::from_value::<Vec<BTreeMap<String, String>>>(c.clone())
                .map_err(|_| RenderError::new("Could not decode the JSON data"))
        })?;
        let current = rc.evaluate_absolute("path", true)?
            .as_str()
            .ok_or_else(|| RenderError::new("Type error for `path`, string expected"))?
            .replace("\"", "");

        rc.writer.write_all(b"<ol class=\"chapter\">")?;

        let mut current_level = 1;

        for item in chapters {
            // Spacer
            if item.get("spacer").is_some() {
                rc.writer.write_all(b"<li class=\"spacer\"></li>")?;
                continue;
            }

            let level = if let Some(s) = item.get("section") {
                s.matches('.').count()
            } else {
                1
            };

            if level > current_level {
                while level > current_level {
                    rc.writer.write_all(b"<li>")?;
                    rc.writer.write_all(b"<ol class=\"section\">")?;
                    current_level += 1;
                }
                rc.writer.write_all(b"<li>")?;
            } else if level < current_level {
                while level < current_level {
                    rc.writer.write_all(b"</ol>")?;
                    rc.writer.write_all(b"</li>")?;
                    current_level -= 1;
                }
                rc.writer.write_all(b"<li>")?;
            } else {
                rc.writer.write_all(b"<li")?;
                if item.get("section").is_none() {
                    rc.writer.write_all(b" class=\"affix\"")?;
                }
                rc.writer.write_all(b">")?;
            }

            // Link
            let path_exists = if let Some(path) = item.get("path") {
                if !path.is_empty() {
                    rc.writer.write_all(b"<a href=\"")?;

                    let tmp = {
                        // To be recognized by browsers, rewrite extenstion to `.html`.
                        let path = Path::new(path).with_extension("html");
                        self.rewrite_directory_index(&path)
                    }
                        .to_str()
                        .unwrap()
                        // Hack for windows who tends to use `\` as separator instead of `/`
                        .replace("\\", "/");

                    // Add link
                    rc.writer.write_all(tmp.as_bytes())?;
                    rc.writer.write_all(b"\"")?;

                    if path == &current {
                        rc.writer.write_all(b" class=\"active\"")?;
                    }

                    rc.writer.write_all(b">")?;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if !self.no_section_label {
                // Section does not necessarily exist
                if let Some(section) = item.get("section") {
                    rc.writer.write_all(b"<strong aria-hidden=\"true\">")?;
                    rc.writer.write_all(section.as_bytes())?;
                    rc.writer.write_all(b"</strong> ")?;
                }
            }

            if let Some(name) = item.get("name") {
                // Render only inline code blocks

                // filter all events that are not inline code blocks
                let parser = Parser::new(name).filter(|event| match *event {
                    Event::Start(Tag::Code)
                    | Event::End(Tag::Code)
                    | Event::InlineHtml(_)
                    | Event::Text(_) => true,
                    _ => false,
                });

                // render markdown to html
                let mut markdown_parsed_name = String::with_capacity(name.len() * 3 / 2);
                html::push_html(&mut markdown_parsed_name, parser);

                // write to the handlebars template
                rc.writer.write_all(markdown_parsed_name.as_bytes())?;
            }

            if path_exists {
                rc.writer.write_all(b"</a>")?;
            }

            rc.writer.write_all(b"</li>")?;
        }
        while current_level > 1 {
            rc.writer.write_all(b"</ol>")?;
            rc.writer.write_all(b"</li>")?;
            current_level -= 1;
        }

        rc.writer.write_all(b"</ol>")?;
        Ok(())
    }

}

impl RenderToc {
    // Rewrite filenames matches any in `rewrite_to_dir` to directory index.
    fn rewrite_directory_index(&self, path: &Path) -> PathBuf {
        for filename in self.rewrite_to_dir.iter() {
            if filename.as_str() == path.file_name().unwrap_or_default() {
                return path.with_file_name("");
            }
        }
        return path.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_dir_success() {
        let render = RenderToc {
            no_section_label: true,
            rewrite_to_dir: vec![
                "index.html".to_owned(),
                "index.md".to_owned(),
            ],
        };
        let path = PathBuf::from("index.html");
        assert_eq!(render.rewrite_directory_index(&path), PathBuf::from(""));

        let path = PathBuf::from("index.md");
        assert_eq!(render.rewrite_directory_index(&path), PathBuf::from(""));

        let path = PathBuf::from("index.asp");
        assert_eq!(render.rewrite_directory_index(&path), path);
    }
}
