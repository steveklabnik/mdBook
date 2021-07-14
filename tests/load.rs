mod dummy_book;

use crate::dummy_book::{assert_contains_strings, DummyBook};
use mdbook::book::parse_summary;
use mdbook::config::Config;
use mdbook::utils::fs::write_file;
use mdbook::MDBook;
use std::fs;

#[test]
fn load_with_default_config() {
    let temp = DummyBook::new().build().unwrap();
    assert!(!temp.path().join("book.toml").exists());

    let md = MDBook::load(temp.path()).unwrap();

    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book</title>"#]);
}

#[test]
fn load_with_book_toml_implicit() {
    let temp = DummyBook::new().build().unwrap();

    let book_toml = r#"
[book]
title = "implicit"
    "#;

    write_file(&temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    assert!(temp.path().join("book.toml").exists());

    let md = MDBook::load(temp.path()).unwrap();

    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book - implicit</title>"#]);
}

#[test]
fn load_with_book_toml_explicit() {
    let temp = DummyBook::new().build().unwrap();

    let book_toml = r#"
[book]
title = "explicit"
    "#;

    write_file(&temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    assert!(temp.path().join("book.toml").exists());

    let md = MDBook::load_with_config_file(temp.path(), &temp.path().join("book.toml")).unwrap();

    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book - explicit</title>"#]);
}

#[test]
fn load_with_alternate_toml() {
    let temp = DummyBook::new().build().unwrap();

    let alternate_toml = r#"
[book]
title = "alternate"
    "#;

    write_file(&temp.path(), "not-book.toml", alternate_toml.as_bytes()).unwrap();

    assert!(!temp.path().join("book.toml").exists());
    assert!(temp.path().join("not-book.toml").exists());

    let md =
        MDBook::load_with_config_file(temp.path(), &temp.path().join("not-book.toml")).unwrap();

    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(
        index_html,
        &vec![r#"<title>Dummy Book - alternate</title>"#],
    );
}

#[test]
fn load_with_alternate_toml_with_book_toml_present() {
    let temp = DummyBook::new().build().unwrap();

    let book_toml = r#"
[book]
title = "book"
    "#;

    write_file(&temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    let alternate_toml = r#"
[book]
title = "not book"
    "#;

    write_file(&temp.path(), "not-book.toml", alternate_toml.as_bytes()).unwrap();

    assert!(temp.path().join("book.toml").exists());
    assert!(temp.path().join("not-book.toml").exists());

    let md =
        MDBook::load_with_config_file(temp.path(), &temp.path().join("not-book.toml")).unwrap();

    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book - not book</title>"#]);
}

#[test]
fn load_with_config_default() {
    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();

    let md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book</title>"#]);
}

#[test]
fn load_with_config_from_disk() {
    let temp = DummyBook::new().build().unwrap();

    let book_toml = r#"
[book]
title = "book"
    "#;

    write_file(&temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    let cfg = Config::from_disk(&temp.path().join("book.toml")).unwrap();

    let md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book - book</title>"#]);
}

#[test]
fn load_with_config_and_summary() {
    let temp = DummyBook::new().build().unwrap();

    let cfg = Config::default();
    let summary = fs::read_to_string(temp.path().join("src").join("SUMMARY.md")).unwrap();
    let summary = parse_summary(&summary).unwrap();

    let md = MDBook::load_with_config_and_summary(temp.path(), cfg, summary).unwrap();
    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(index_html, &vec![r#"<title>Dummy Book</title>"#]);
}

#[test]
#[should_panic]
fn try_load_with_missing_file() {
    let temp = DummyBook::new().build().unwrap();
    MDBook::load_with_config_file(temp.path(), &temp.path().join("not-there.toml")).unwrap();
}
