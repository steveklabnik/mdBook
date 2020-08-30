# 内部リンクの入れ替え

以下のイメージは英語バージョンから移植されたでしょうか。

If inline link substitution works, then an image should appear below, sourced from the English translation.
 
![Rust logo](rust_logo.png)

Here is an [inline link](translation-local-page.md) to an existing page in this translation.

Here is an [inline link](missing-summary-chapter.md) to a page missing from this translation's `SUMMARY.md`. It should have been modified to point to the page in the English version of the book.

Also, here is an [inline link](blah.md) to a page missing from both translations. It should point to this language's 404 page.

Here is a file included from the default language.
```rust
{{ #include example.rs }}
```

The substitution won't work if you specify the `-l`/`--language` option, since it only builds a single translation in that case.
