use std::path::{Path, PathBuf};

use config::{load_config, Config};
use book::{Chapter, BookItem};
use errors::*;

/// Loader is the object in charge of loading the source documents from disk.
///
/// It Will:
///
/// - Initialize a new project
/// - Parse `SUMMARY.md`
/// - Traverse the source directory, looking for markdown files
/// - Turn all of that into a single data structure which is an in-memory
///   representation of the book
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Loader {
    root: PathBuf,
    config: Config,
}

impl Loader {
    /// Create a new `Loader` with `root` as the book's root directory.
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Loader> {
        let root = PathBuf::from(root.as_ref());

        let config = load_config(&root)?;
        Ok(Loader {
            root: root,
            config: config,
        })
    }

    fn parse_summary(&self) -> Result<Summary> {
        unimplemented!()
    }
}

struct Summary{}


fn parse_level(summary: &mut Vec<&str>, current_level: i32, mut section: Vec<i32>) -> Result<Vec<BookItem>> {
    debug!("[fn]: parse_level");
    let mut items: Vec<BookItem> = vec![];

    // Construct the book recursively
    while !summary.is_empty() {
        let item: BookItem;
        // Indentation level of the line to parse
        let level = level(summary[0], 4)?;

        // if level < current_level we remove the last digit of section,
        // exit the current function,
        // and return the parsed level to the calling function.
        if level < current_level {
            break;
        }

        // if level > current_level we call ourselves to go one level deeper
        if level > current_level {
            // Level can not be root level !!
            // Add a sub-number to section
            section.push(0);
            let last = items
                .pop()
                .expect("There should be at least one item since this can't be the root level");

            if let BookItem::Chapter(ref s, ref ch) = last {
                let mut ch = ch.clone();
                ch.items = parse_level(summary, level, section.clone())?;
                items.push(BookItem::Chapter(s.clone(), ch));

                // Remove the last number from the section, because we got back to our level..
                section.pop();
                continue;
            } else {
                bail!(
                                      "Your summary.md is messed up\n\n
                        Prefix, \
                                       Suffix and Spacer elements can only exist on the root level.\n
                        \
                                       Prefix elements can only exist before any chapter and there can be \
                                       no chapters after suffix elements.");
            };

        } else {
            // level and current_level are the same, parse the line
            item = if let Some(parsed_item) = parse_line(summary[0]) {
                let parsed_item = parsed_item?;

                // Eliminate possible errors and set section to -1 after suffix
                match parsed_item {
                    // error if level != 0 and BookItem is != Chapter
                    BookItem::Affix(_) |
                    BookItem::Spacer if level > 0 => {
                        bail!(
                                              "Your summary.md is messed up\n\n
                                \
                                               Prefix, Suffix and Spacer elements can only exist on the \
                                               root level.\n
                                Prefix \
                                               elements can only exist before any chapter and there can be \
                                               no chapters after suffix elements.");
                    },

                    // error if BookItem == Chapter and section == -1
                    BookItem::Chapter(_, _) if section[0] == -1 => {
                        bail!(
                                              "Your summary.md is messed up\n\n
                                \
                                               Prefix, Suffix and Spacer elements can only exist on the \
                                               root level.\n
                                Prefix \
                                               elements can only exist before any chapter and there can be \
                                               no chapters after suffix elements.");
                    },

                    // Set section = -1 after suffix
                    BookItem::Affix(_) if section[0] > 0 => {
                        section[0] = -1;
                    },

                    _ => {},
                }

                match parsed_item {
                    BookItem::Chapter(_, ch) => {
                        // Increment section
                        let len = section.len() - 1;
                        section[len] += 1;
                        let s = section
                            .iter()
                            .fold("".to_owned(), |s, i| s + &i.to_string() + ".");
                        BookItem::Chapter(s, ch)
                    },
                    other => other,
                }

            } else {
                // If parse_line does not return Some(_) continue...
                summary.remove(0);
                continue;
            };
        }

        summary.remove(0);
        items.push(item);
    }
    debug!("[*]: Level: {:?}", items);
    Ok(items)
}

fn level(line: &str, spaces_in_tab: i32) -> Result<i32> {
    debug!("[fn]: level");
    let mut spaces = 0;
    let mut level = 0;

    for ch in line.chars() {
        match ch {
            ' ' => spaces += 1,
            '\t' => level += 1,
            _ => break,
        }
        if spaces >= spaces_in_tab {
            level += 1;
            spaces = 0;
        }
    }

    // If there are spaces left, there is an indentation error
    if spaces > 0 {
        debug!("[SUMMARY.md]:");
        debug!("\t[line]: {}", line);
        debug!("[*]: There is an indentation error on this line. Indentation should be {} spaces", spaces_in_tab);
        bail!("Indentation error on line:\n\n{}", line);
    }

    Ok(level)
}


fn parse_line(l: &str) -> Option<Result<BookItem>> {
    debug!("[fn]: parse_line");

    // Remove leading and trailing spaces or tabs
    let line = l.trim_matches(|c: char| c == ' ' || c == '\t');

    // Spacers are "------"
    if line.starts_with("--") {
        debug!("[*]: Line is spacer");
        return Some(Ok(BookItem::Spacer));
    }

    if let Some(c) = line.chars().nth(0) {
        match c {
            // List item
            '-' | '*' => {
                debug!("[*]: Line is list element");

                if let Some((name, path)) = read_link(line) {
                    return Some(Chapter::new(name, path)
                        .map(|ch| BookItem::Chapter("0".to_owned(), ch)));
                } else {
                    return None;
                }
            },
            // Non-list element
            '[' => {
                debug!("[*]: Line is a link element");

                if let Some((name, path)) = read_link(line) {
                    match Chapter::new(name, path) {
                        Ok(ch) => return Some(Ok(BookItem::Affix(ch))),
                        Err(e) => return Some(Err(e)),
                    }
                } else {
                    return None;
                }
            },
            _ => {},
        }
    }

    None
}

fn read_link(line: &str) -> Option<(String, PathBuf)> {
    let mut start_delimitor;
    let mut end_delimitor;

    // In the future, support for list item that is not a link
    // Not sure if I should error on line I can't parse or just ignore them...
    if let Some(i) = line.find('[') {
        start_delimitor = i;
    } else {
        debug!("[*]: '[' not found, this line is not a link. Ignoring...");
        return None;
    }

    if let Some(i) = line[start_delimitor..].find("](") {
        end_delimitor = start_delimitor + i;
    } else {
        debug!("[*]: '](' not found, this line is not a link. Ignoring...");
        return None;
    }

    let name = line[start_delimitor + 1..end_delimitor].to_owned();

    start_delimitor = end_delimitor + 1;
    if let Some(i) = line[start_delimitor..].find(')') {
        end_delimitor = start_delimitor + i;
    } else {
        debug!("[*]: ')' not found, this line is not a link. Ignoring...");
        return None;
    }

    let path = PathBuf::from(line[start_delimitor + 1..end_delimitor].to_owned());

    Some((name, path))
}
