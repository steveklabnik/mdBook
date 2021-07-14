mod flag;
mod spacer;

use std::io;
use std::path::Path;

use crate::utils;

use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError};
use pulldown_cmark::{html, Event, Parser};

// Handlebars helper to construct TOC
#[derive(Clone, Copy)]
pub struct RenderToc {
    pub no_section_label: bool,
}

// Allowed format for the `chapters` array.
#[derive(Deserialize)]
struct ChapterItemData {
    name: Option<String>,
    part: Option<String>,
    #[serde(default)]
    section: SectionItemData,
    #[serde(default)]
    path: String,
    // notriddle: has_sub_items and spacer are kind of pointlessly different,
    // for backwards compatibility's sake. They should eventually
    // be made to use the same syntax.
    #[serde(default, with = "spacer")]
    spacer: bool,
    #[serde(default, with = "flag")]
    has_sub_items: bool,
}

struct SectionItemData {
    name: String,
    level: usize,
}

impl<'de> serde::Deserialize<'de> for SectionItemData {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<SectionItemData, D::Error> {
        let name: String = serde::Deserialize::deserialize(de)?;
        let level = name.bytes().filter(|c| *c == b'.').count();
        Ok(SectionItemData { name, level })
    }
}

impl Default for SectionItemData {
    fn default() -> SectionItemData {
        SectionItemData {
            name: String::new(),
            level: 1,
        }
    }
}

impl SectionItemData {
    fn is_parent_of(&self, other: &str) -> bool {
        !self.is_empty() && other.starts_with(&self.name)
    }
    fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

impl HelperDef for RenderToc {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'_>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        // get value from context data
        // rc.get_path() is current json parent path, you should always use it like this
        // param is the key of value you want to display
        let chapters = rc.evaluate(ctx, "@root/chapters").and_then(|c| {
            serde_json::value::from_value::<Vec<ChapterItemData>>(c.as_json().clone())
                .map_err(|_| RenderError::new("Could not decode the JSON data"))
        })?;
        let current_path = rc
            .evaluate(ctx, "@root/path")?
            .as_json()
            .as_str()
            .ok_or_else(|| RenderError::new("Type error for `path`, string expected"))?
            .replace("\"", "");

        let current_section = rc
            .evaluate(ctx, "@root/section")?
            .as_json()
            .as_str()
            .map(str::to_owned)
            .unwrap_or_default();

        let fold_enable = rc
            .evaluate(ctx, "@root/fold_enable")?
            .as_json()
            .as_bool()
            .ok_or_else(|| RenderError::new("Type error for `fold_enable`, bool expected"))?;

        let fold_level = rc
            .evaluate(ctx, "@root/fold_level")?
            .as_json()
            .as_u64()
            .ok_or_else(|| RenderError::new("Type error for `fold_level`, u64 expected"))?;

        out.write("<ol class=\"chapter\">")?;

        let mut current_level = 1;

        for item in chapters {
            // Spacer
            if item.spacer {
                out.write("<li class=\"spacer\"></li>")?;
                continue;
            }

            let level = item.section.level;

            let is_expanded = if !fold_enable || item.section.is_parent_of(&current_section) {
                // Expand if folding is disabled, or if the section is an
                // ancestor or the current section itself.
                true
            } else {
                // Levels that are larger than this would be folded.
                level - 1 < fold_level as usize
            };

            if level > current_level {
                while level > current_level {
                    out.write("<li>")?;
                    out.write("<ol class=\"section\">")?;
                    current_level += 1;
                }
                write_li_open_tag(out, is_expanded, false)?;
            } else if level < current_level {
                while level < current_level {
                    out.write("</ol>")?;
                    out.write("</li>")?;
                    current_level -= 1;
                }
                write_li_open_tag(out, is_expanded, false)?;
            } else {
                write_li_open_tag(out, is_expanded, item.section.is_empty())?;
            }

            // Part title
            if let Some(title) = &item.part {
                out.write("<li class=\"part-title\">")?;
                write_escaped(out, title)?;
                out.write("</li>")?;
                continue;
            }

            // Link
            let path_exists = if item.path.is_empty() {
                out.write("<div>")?;
                false
            } else {
                out.write("<a href=\"")?;

                let tmp = Path::new(&item.path)
                    .with_extension("html")
                    .to_str()
                    .unwrap()
                    // Hack for windows who tends to use `\` as separator instead of `/`
                    .replace("\\", "/");

                // Add link
                out.write(&utils::fs::path_to_root(&current_path))?;
                out.write(&tmp)?;
                out.write("\"")?;

                if &item.path == &current_path {
                    out.write(" class=\"active\"")?;
                }

                out.write(">")?;
                true
            };

            if !self.no_section_label {
                // Section does not necessarily exist
                if !item.section.is_empty() {
                    out.write("<strong aria-hidden=\"true\">")?;
                    out.write(&item.section.name)?;
                    out.write("</strong> ")?;
                }
            }

            if let Some(name) = &item.name {
                // Render only inline code blocks

                // filter all events that are not inline code blocks
                let parser = Parser::new(name).filter(|event| match *event {
                    Event::Code(_) | Event::Html(_) | Event::Text(_) => true,
                    _ => false,
                });

                // render markdown to html
                let mut markdown_parsed_name = String::with_capacity(name.len() * 3 / 2);
                html::push_html(&mut markdown_parsed_name, parser);

                // write to the handlebars template
                write_escaped(out, &markdown_parsed_name)?;
            }

            if path_exists {
                out.write("</a>")?;
            } else {
                out.write("</div>")?;
            }

            // Render expand/collapse toggle
            if fold_enable && item.has_sub_items {
                out.write("<a class=\"toggle\"><div>❱</div></a>")?;
            }
            out.write("</li>")?;
        }
        while current_level > 1 {
            out.write("</ol>")?;
            out.write("</li>")?;
            current_level -= 1;
        }

        out.write("</ol>")?;
        Ok(())
    }
}

fn write_li_open_tag(
    out: &mut dyn Output,
    is_expanded: bool,
    is_affix: bool,
) -> Result<(), std::io::Error> {
    let mut li = String::from("<li class=\"chapter-item ");
    if is_expanded {
        li.push_str("expanded ");
    }
    if is_affix {
        li.push_str("affix ");
    }
    li.push_str("\">");
    out.write(&li)
}

fn write_escaped(out: &mut dyn Output, mut title: &str) -> io::Result<()> {
    let needs_escape: &[char] = &['<', '>'];
    while let Some(next) = title.find(needs_escape) {
        out.write(&title[..next])?;
        match title.as_bytes()[next] {
            b'<' => out.write("&lt;")?,
            b'>' => out.write("&gt;")?,
            _ => unreachable!(),
        }
        title = &title[next + 1..];
    }
    out.write(title)?;
    Ok(())
}
