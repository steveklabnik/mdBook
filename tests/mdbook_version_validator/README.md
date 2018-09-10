# Testing Upstream Books for Breakage from mdbook

- Theory - when mdbook changes, we should be able to coarsely tell if building a book with the new version fails
- In Practice - Build the book twice and compare


    1. Build two binaries of mdbook, some "current" (master) and some "new" branch
       (Building dependent binaries - mdbook-linkcheck for example from https://github.com/Michael-F-Bryan/mdbook-linkcheck - are not supported "per build"
       but rather the same binary is executed for both. Should not be hard to change that)

       This is it's own BASH shell script

       `tests/mdbook_version_validator/build_mdbook_branches.sh ${MDBOOK_BRANCH_1},${MDBOOK_BRANCH_2} ${G_TMPDIR} ${PWD}/target`

        **Now we have two mdbook binaries**

    2. Use the github API and find all repos under known orgs on github.com (and support extras via the command line)
       
    2. Checkout the master branch of each one
    3. Build each book twice
    4. Compare the output directories

       Comparison is difficult, but the following applies

       `OK`   - if the two output directories are identical
       `WARN` - status==OK, if the two directories are not identical, but the size of the second is +/- 5% of the first.
       `FAIL` - if the size of the second is greater than +/- 5% of the first.

Not bullet proof - but it works.

## Quick Start Guide

`sh tests/mdbook_version_validator/test_mdbook_book_builds.sh <branch1> <branch2>`
- This will build both versions of mdbook
- Search Github for `book.toml` files in repos
- Download each book, build them with both binaries, compare and report

## Output - Failure

Showing a tolerance failure (within +/- 5%)

```
$ sh tests/mdbook_version_validator/test_mdbook_book_builds.sh master smart-preprocessor
:: We have both binaries [/home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master, /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor]
:: Collecting the test book repos
  :: https://github.com/rust-lang-nursery/mdBook
  :: https://github.com/rust-lang/rust-by-example
  :: https://github.com/rust-lang-nursery/api-guidelines
  :: https://github.com/rust-lang-nursery/rustc-guide
  :: https://github.com/rust-lang-nursery/rust-cookbook
:: Building https://github.com/rust-lang-nursery/mdBook
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.018quaRm/https___github_com_rust_lang_nursery_mdBook'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang-nursery/mdBook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:50:38 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:50:38 [INFO] (mdbook::book): Running the html backend
  :: Building rust-lang-nursery/mdBook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:50:40 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:50:40 [INFO] (mdbook::book): Running the html backend
  :: /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.018quaRm/https___github_com_rust_lang_nursery_mdBook/book-example/book.toml the directories differ
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.018quaRm/generated_books/https___github_com_rust_lang_nursery_mdBook/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.018quaRm/generated_books/https___github_com_rust_lang_nursery_mdBook/mdbook-smart-preprocessor
  :: WARN differed, but inside tolerance +/- 5%
:: Building https://github.com/rust-lang/rust-by-example
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.018quaRm/https___github_com_rust_lang_rust_by_example'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang/rust-by-example with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:50:45 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:50:45 [INFO] (mdbook::book): Running the html backend
...
```

## Output - Success

```
23:42 $ sh tests/mdbook_version_validator/test_mdbook_book_builds.sh master smart-preprocessor
:: We have both binaries [/home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master, /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor]
:: Collecting the test book repos
  :: https://github.com/rust-lang-nursery/mdBook
  :: https://github.com/rust-lang/rust-by-example
  :: https://github.com/rust-lang-nursery/api-guidelines
  :: https://github.com/rust-lang-nursery/rustc-guide
  :: https://github.com/rust-lang-nursery/rust-cookbook
:: Building https://github.com/rust-lang-nursery/mdBook
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_mdBook'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang-nursery/mdBook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:42:40 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:42:40 [INFO] (mdbook::book): Running the html backend
  :: Building rust-lang-nursery/mdBook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:42:42 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:42:42 [INFO] (mdbook::book): Running the html backend
  :: OK /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_mdBook/book-example/book.toml is identical
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_mdBook/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_mdBook/mdbook-smart-preprocessor
:: Building https://github.com/rust-lang/rust-by-example
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_rust_by_example'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang/rust-by-example with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:42:48 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:42:48 [INFO] (mdbook::book): Running the html backend
  :: Building rust-lang/rust-by-example with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:43:02 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:02 [INFO] (mdbook::book): Running the html backend
  :: OK /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_rust_by_example/book.toml is identical
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_rust_by_example/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_rust_by_example/mdbook-smart-preprocessor
:: Building https://github.com/rust-lang-nursery/api-guidelines
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_api_guidelines'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang-nursery/api-guidelines with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:43:20 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:20 [INFO] (mdbook::book): Running the html backend
  :: Building rust-lang-nursery/api-guidelines with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:43:23 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:23 [INFO] (mdbook::book): Running the html backend
  :: OK /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_api_guidelines/book.toml is identical
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_api_guidelines/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_api_guidelines/mdbook-smart-preprocessor
:: Building https://github.com/rust-lang-nursery/rustc-guide
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_rustc_guide'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang-nursery/rustc-guide with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:43:29 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:29 [INFO] (mdbook::book): Running the html backend
  2018-09-03 23:43:40 [INFO] (mdbook::book): Running the linkcheck backend
  2018-09-03 23:43:40 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
  :: Building rust-lang-nursery/rustc-guide with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:43:40 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:40 [INFO] (mdbook::book): Running the html backend
  2018-09-03 23:43:50 [INFO] (mdbook::book): Running the linkcheck backend
  2018-09-03 23:43:50 [INFO] (mdbook::renderer): Invoking the "linkcheck" renderer
  :: OK /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_rustc_guide/book.toml is identical
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_rustc_guide/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_rustc_guide/mdbook-smart-preprocessor
:: Building https://github.com/rust-lang-nursery/rust-cookbook
  Cloning into '/var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_rust_cookbook'...
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  :: /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  :: Building rust-lang-nursery/rust-cookbook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-master
  2018-09-03 23:43:54 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:43:54 [INFO] (mdbook::book): Running the html backend
  :: Building rust-lang-nursery/rust-cookbook with /home/rbuckland/projects/github.com/rust-lang-nursery/mdBook/target/mdbook-smart-preprocessor
  2018-09-03 23:44:01 [INFO] (mdbook::book): Book building has started
  2018-09-03 23:44:01 [INFO] (mdbook::book): Running the html backend
  :: OK /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/https___github_com_rust_lang_nursery_rust_cookbook/book.toml is identical
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_rust_cookbook/mdbook-master
   - /var/folders/cy/m597b3cj1s71904cxdtxk8dnlthn4x/T/tmp.wtHqLOnn/generated_books/https___github_com_rust_lang_nursery_rust_cookbook/mdbook-smart-preprocessor
```
