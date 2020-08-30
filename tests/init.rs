use mdbook::config::Config;
use mdbook::MDBook;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tempfile::Builder as TempFileBuilder;

/// Run `mdbook init` in an empty directory and make sure the default files
/// are created.
#[test]
fn base_mdbook_init_should_create_default_content() {
    let created_files = vec![
        "book",
        "src",
        "src/en",
        "src/en/SUMMARY.md",
        "src/en/chapter_1.md",
    ];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    for file in &created_files {
        assert!(!temp.path().join(file).exists());
    }

    MDBook::init(temp.path()).build().unwrap();

    for file in &created_files {
        let target = temp.path().join(file);
        println!("{}", target.display());
        assert!(target.exists(), "{} doesn't exist", file);
    }

    let contents = fs::read_to_string(temp.path().join("book.toml")).unwrap();
    assert_eq!(
        contents,
        "[book]\nauthors = []\nlanguage = \"en\"\nsrc = \"src\"\n[language.en]\nname = \"English\"\n"
    );
}

/// Run `mdbook init` in a directory containing a SUMMARY.md should create the
/// files listed in the summary.
#[test]
fn run_mdbook_init_should_create_content_from_summary() {
    let created_files = vec!["intro.md", "first.md", "outro.md"];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let src_dir = temp.path().join("src").join("en");
    fs::create_dir_all(src_dir.clone()).unwrap();
    static SUMMARY: &str = r#"# Summary

[intro](intro.md)

- [First chapter](first.md)

[outro](outro.md)

"#;

    let mut summary = File::create(src_dir.join("SUMMARY.md")).unwrap();
    summary.write_all(SUMMARY.as_bytes()).unwrap();
    MDBook::init(temp.path()).build().unwrap();

    for file in &created_files {
        let target = src_dir.join(file);
        println!("{}", target.display());
        assert!(target.exists(), "{} doesn't exist", file);
    }
}

/// Set some custom arguments for where to place the source and destination
/// files, then call `mdbook init`.
#[test]
fn run_mdbook_init_with_custom_book_and_src_locations() {
    let created_files = vec![
        "out",
        "in",
        "in/en",
        "in/en/SUMMARY.md",
        "in/en/chapter_1.md",
    ];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    for file in &created_files {
        assert!(
            !temp.path().join(file).exists(),
            "{} shouldn't exist yet!",
            file
        );
    }

    let mut cfg = Config::default();
    cfg.book.src = PathBuf::from("in");
    cfg.build.build_dir = PathBuf::from("out");

    MDBook::init(temp.path()).with_config(cfg).build().unwrap();

    for file in &created_files {
        let target = temp.path().join(file);
        assert!(
            target.exists(),
            "{} should have been created by `mdbook init`",
            file
        );
    }
}

#[test]
fn book_toml_isnt_required() {
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let md = MDBook::init(temp.path()).build().unwrap();

    let _ = fs::remove_file(temp.path().join("book.toml"));

    md.build().unwrap();
}
