use crate::config::HtmlConfig;
use crate::errors::*;
use crate::renderer::html_handlebars::helpers::resources::ResourceHelper;
use crate::theme::{self, playground_editor, Theme};
use crate::utils;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Map static files to their final names and contents.
///
/// It performs [fingerprinting], if you call the `hash_files` method.
/// If hash-files is turned off, then the files will not be renamed.
/// It also writes files to their final destination, when `write_files` is called,
/// and interprets the `{{ resource }}` directives to allow assets to name each other.
///
/// [fingerprinting]: https://guides.rubyonrails.org/asset_pipeline.html#what-is-fingerprinting-and-why-should-i-care-questionmark
pub struct StaticFiles {
    static_files: Vec<StaticFile>,
    hash_map: HashMap<String, String>,
}

enum StaticFile {
    Builtin {
        data: Vec<u8>,
        filename: String,
    },
    Additional {
        input_location: PathBuf,
        filename: String,
    },
}

impl StaticFiles {
    pub fn new(theme: &Theme, html_config: &HtmlConfig, root: &Path) -> Result<StaticFiles> {
        let static_files = Vec::new();
        let mut this = StaticFiles {
            hash_map: HashMap::new(),
            static_files,
        };

        this.add_builtin("book.js", &theme.js);
        this.add_builtin("css/general.css", &theme.general_css);
        this.add_builtin("css/chrome.css", &theme.chrome_css);
        if html_config.print.enable {
            this.add_builtin("css/print.css", &theme.print_css);
        }
        this.add_builtin("css/variables.css", &theme.variables_css);
        if let Some(contents) = &theme.favicon_png {
            this.add_builtin("favicon.png", &contents);
        }
        if let Some(contents) = &theme.favicon_svg {
            this.add_builtin("favicon.svg", &contents);
        }
        this.add_builtin("highlight.css", &theme.highlight_css);
        this.add_builtin("tomorrow-night.css", &theme.tomorrow_night_css);
        this.add_builtin("ayu-highlight.css", &theme.ayu_highlight_css);
        this.add_builtin("highlight.js", &theme.highlight_js);
        this.add_builtin("clipboard.min.js", &theme.clipboard_js);
        this.add_builtin("FontAwesome/css/font-awesome.css", theme::FONT_AWESOME);
        this.add_builtin(
            "FontAwesome/fonts/fontawesome-webfont.eot",
            theme::FONT_AWESOME_EOT,
        );
        this.add_builtin(
            "FontAwesome/fonts/fontawesome-webfont.svg",
            theme::FONT_AWESOME_SVG,
        );
        this.add_builtin(
            "FontAwesome/fonts/fontawesome-webfont.ttf",
            theme::FONT_AWESOME_TTF,
        );
        this.add_builtin(
            "FontAwesome/fonts/fontawesome-webfont.woff",
            theme::FONT_AWESOME_WOFF,
        );
        this.add_builtin(
            "FontAwesome/fonts/fontawesome-webfont.woff2",
            theme::FONT_AWESOME_WOFF2,
        );
        this.add_builtin("FontAwesome/fonts/FontAwesome.ttf", theme::FONT_AWESOME_TTF);
        if html_config.copy_fonts {
            this.add_builtin("fonts/fonts.css", theme::fonts::CSS);
            for (file_name, contents) in theme::fonts::LICENSES.iter() {
                this.add_builtin(file_name, contents);
            }
            for (file_name, contents) in theme::fonts::OPEN_SANS.iter() {
                this.add_builtin(file_name, contents);
            }
            this.add_builtin(
                theme::fonts::SOURCE_CODE_PRO.0,
                theme::fonts::SOURCE_CODE_PRO.1,
            );
        }

        let playground_config = &html_config.playground;

        // Ace is a very large dependency, so only load it when requested
        if playground_config.editable && playground_config.copy_js {
            // Load the editor
            this.add_builtin("editor.js", playground_editor::JS);
            this.add_builtin("ace.js", playground_editor::ACE_JS);
            this.add_builtin("mode-rust.js", playground_editor::MODE_RUST_JS);
            this.add_builtin("theme-dawn.js", playground_editor::THEME_DAWN_JS);
            this.add_builtin(
                "theme-tomorrow_night.js",
                playground_editor::THEME_TOMORROW_NIGHT_JS,
            );
        }

        let custom_files = html_config
            .additional_css
            .iter()
            .chain(html_config.additional_js.iter());

        for custom_file in custom_files.cloned() {
            let input_location = root.join(&custom_file);

            this.static_files.push(StaticFile::Additional {
                input_location,
                filename: custom_file
                    .to_str()
                    .with_context(|| "resource file names must be valid utf8")?
                    .to_owned(),
            });
        }

        Ok(this)
    }
    pub fn add_builtin(&mut self, filename: &str, data: &[u8]) {
        self.static_files.push(StaticFile::Builtin {
            filename: filename.to_owned(),
            data: data.to_owned(),
        });
    }
    pub fn hash_files(&mut self) -> Result<()> {
        use sha2::{Digest, Sha256};
        use std::io::Read;
        for static_file in &mut self.static_files {
            match static_file {
                StaticFile::Builtin {
                    ref mut filename,
                    ref data,
                } => {
                    let mut parts = filename.splitn(2, '.');
                    let parts = parts.next().and_then(|p| Some((p, parts.next()?)));
                    if let Some((name, suffix)) = parts {
                        // FontAwesome already does its own cache busting with the ?v=4.7.0 thing,
                        // and I don't want to have to patch its CSS file to use `{{ resource }}`
                        if name != ""
                            && suffix != ""
                            && suffix != "txt"
                            && !name.starts_with("FontAwesome/fonts/")
                        {
                            let hex = hex::encode(&Sha256::digest(data)[..4]);
                            let new_filename = format!("{}-{}.{}", name, hex, suffix);
                            self.hash_map.insert(filename.clone(), new_filename.clone());
                            *filename = new_filename;
                        }
                    }
                }
                StaticFile::Additional {
                    ref mut filename,
                    ref input_location,
                } => {
                    let mut parts = filename.splitn(2, '.');
                    let parts = parts.next().and_then(|p| Some((p, parts.next()?)));
                    if let Some((name, suffix)) = parts {
                        if name != "" && suffix != "" {
                            let mut digest = Sha256::new();
                            let mut input_file = File::open(input_location)
                                .with_context(|| "open static file for hashing")?;
                            let mut buf = vec![0; 1024];
                            loop {
                                let amt = input_file
                                    .read(&mut buf)
                                    .with_context(|| "read static file for hashing")?;
                                if amt == 0 {
                                    break;
                                };
                                digest.update(&buf[..amt]);
                            }
                            let hex = hex::encode(&digest.finalize()[..4]);
                            let new_filename = format!("{}-{}.{}", name, hex, suffix);
                            self.hash_map.insert(filename.clone(), new_filename.clone());
                            *filename = new_filename;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn write_files(self, destination: &Path) -> Result<ResourceHelper> {
        use crate::utils::fs::write_file;
        use regex::bytes::{Captures, Regex};
        use std::io::Read;
        // The `{{ resource "name" }}` directive in static resources look like
        // handlebars syntax, even if they technically aren't.
        let resource = Regex::new(r#"\{\{ resource "([^"]+)" \}\}"#).unwrap();
        for static_file in self.static_files {
            match static_file {
                StaticFile::Builtin { filename, data } => {
                    debug!("Writing builtin -> {}", filename);
                    let hash_map = &self.hash_map;
                    let data = if filename.ends_with(".css") || filename.ends_with(".js") {
                        resource.replace_all(&data, |captures: &Captures<'_>| {
                            let name = captures
                                .get(1)
                                .expect("capture 1 in resource regex")
                                .as_bytes();
                            let name =
                                std::str::from_utf8(name).expect("resource name with invalid utf8");
                            let resource_filename = hash_map.get(name).map(|s| &s[..]).unwrap_or(&name);
                            let path_to_root = utils::fs::path_to_root(&filename);
                            format!("{}{}", path_to_root, resource_filename)
                                .as_bytes()
                                .to_owned()
                        })
                    } else {
                        Cow::Borrowed(&data[..])
                    };
                    write_file(destination, &filename, &data)?;
                }
                StaticFile::Additional {
                    ref input_location,
                    ref filename,
                } => {
                    let output_location = destination.join(filename);
                    debug!(
                        "Copying {} -> {}",
                        input_location.display(),
                        output_location.display()
                    );
                    if let Some(parent) = output_location.parent() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("Unable to create {}", parent.display()))?;
                    }
                    if filename.ends_with(".css") || filename.ends_with(".js") {
                        let hash_map = &self.hash_map;
                        let mut file = File::open(input_location)?;
                        let mut data = Vec::new();
                        file.read_to_end(&mut data)?;
                        let data = resource.replace_all(&data, |captures: &Captures<'_>| {
                            let name = captures
                                .get(1)
                                .expect("capture 1 in resource regex")
                                .as_bytes();
                            let name =
                                std::str::from_utf8(name).expect("resource name with invalid utf8");
                            let resource_filename = hash_map.get(name).map(|s| &s[..]).unwrap_or(&name);
                            let path_to_root = utils::fs::path_to_root(&filename);
                            format!("{}{}", path_to_root, resource_filename)
                                .as_bytes()
                                .to_owned()
                        });
                        write_file(destination, &filename, &data)?;
                    } else {
                        fs::copy(&input_location, &output_location).with_context(|| {
                            format!(
                                "Unable to copy {} to {}",
                                input_location.display(),
                                output_location.display()
                            )
                        })?;
                    }
                }
            }
        }
        let hash_map = self.hash_map;
        Ok(ResourceHelper { hash_map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::config::HtmlConfig;
    use crate::utils::fs::write_file;
    use std::io::Read;
    #[test]
    fn test_write_directive() {
        let theme = Theme {
            index: Vec::new(),
            head: Vec::new(),
            redirect: Vec::new(),
            header: Vec::new(),
            chrome_css: Vec::new(),
            general_css: Vec::new(),
            print_css: Vec::new(),
            variables_css: Vec::new(),
            favicon_png: Some(Vec::new()),
            favicon_svg: Some(Vec::new()),
            js: Vec::new(),
            highlight_css: Vec::new(),
            tomorrow_night_css: Vec::new(),
            ayu_highlight_css: Vec::new(),
            highlight_js: Vec::new(),
            clipboard_js: Vec::new(),
        };
        let reference_js = PathBuf::from("target/static-files-test-case-reference.js");
        let test_case = PathBuf::from("target/static-files-test-case");
        let mut html_config = HtmlConfig::default();
        html_config.additional_js.push(reference_js.clone());
        write_file(&Path::new("."), &reference_js, br#"{{ resource "book.js" }}"#).unwrap();
        let mut static_files = StaticFiles::new(&theme, &html_config, &Path::new(".")).unwrap();
        static_files.hash_files().unwrap();
        static_files.write_files(&test_case).unwrap();
        // custom JS winds up referencing book.js
        let mut reference_js_dest = File::open("target/static-files-test-case/target/static-files-test-case-reference-635c9cdc.js").unwrap();
        let mut reference_js_content = Vec::new();
        reference_js_dest.read_to_end(&mut reference_js_content).unwrap();
        assert_eq!(br#"../book-e3b0c442.js"#, &reference_js_content[..]);
        // book.js winds up empty
        let mut reference_js_dest = File::open("target/static-files-test-case/book-e3b0c442.js").unwrap();
        let mut reference_js_content = Vec::new();
        reference_js_dest.read_to_end(&mut reference_js_content).unwrap();
        assert_eq!(br#""#, &reference_js_content[..]);
        std::fs::remove_dir_all(&test_case).unwrap();
        std::fs::remove_file(&reference_js).unwrap();
    }
}