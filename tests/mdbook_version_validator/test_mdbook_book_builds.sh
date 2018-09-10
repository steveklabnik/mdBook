#!/bin/bash 

red=$'\e[1;31m'
grn=$'\e[1;32m'
blu=$'\e[1;34m'
mag=$'\e[1;35m'
cyn=$'\e[1;36m'
white=$'\e[0m'

indent() { sed 's/^/  /'; }

MDBOOK_BRANCH_1=$1
MDBOOK_BRANCH_2=$2
# comma separated
EXTRA_REPOS=$3

#
# Example run
#  sh tests/mdbook_version_validator/test_mdbook_book_builds.sh master smart-preprocessor
#
#  or, add your own repos
#
#  sh tests/mdbook_version_validator/test_mdbook_book_builds.sh master smart-preprocessor http://git/my/repo1,http://git/my/repo2
#

#
# Globals
#
G_BOOKS=()
G_TMPDIR=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')
G_BINARY_BRANCH_NAME_1=$( echo $MDBOOK_BRANCH_1 | sed 's/[^a-zA-Z0-9-]/_/g')
G_BINARY_BRANCH_NAME_2=$( echo $MDBOOK_BRANCH_2 | sed 's/[^a-zA-Z0-9-]/_/g')
G_DIRNAME=$(dirname $(perl -e 'use Cwd "abs_path";print abs_path(shift)' $0))

#-----------------------------------------
function collect_book_repos()  {
  echo $blu:: Collecting the "test" book repos $white
  repos=$1

  for i in $(echo $repos | sed "s/,/ /g")
  do
    # call your procedure/other scripts here below
    G_BOOKS+=( $i )
  done

  # Search for book.toml on github.com in these Org Repos - org:rust-lang-nursery org:rust-lang
  for i in $(curl --silent --fail "https://api.github.com/search/code?q=book+description+in:file+filename:book.toml+org:rust-lang-nursery+org:rust-lang" | jq -r '.items[] | .repository.full_name')
  do
    G_BOOKS+=( "https://github.com/"$i )
    echo ":: https://github.com/$i" | indent
  done
}
#-----------------------------------------
function build_books_and_compare() {
  for book_repo in "${G_BOOKS[@]}"
  do
    echo ${blu}:: Building ${book_repo}$white
    repo_name_safe=$( echo $book_repo | sed 's/[^a-zA-Z0-9]/_/g')
    book_dir=${G_TMPDIR}/${repo_name_safe}
    local user_org=$( basename $( dirname $book_repo ))
    local repo=$( basename $book_repo )
    git clone --depth=1 --branch=master ${book_repo} ${book_dir} 2>&1 | indent
    rm -rf ${book_dir}/.git
    ${G_DIRNAME}/build_books_and_compare.sh --version1 ${G_BINARY_PATH_1} --version2 ${G_BINARY_PATH_2} --book-name ${user_org}/${repo} --book-path ${book_dir} --working-dir ${G_TMPDIR}/generated_books/${repo_name_safe} 2>&1 | indent
  done
}
#-----------------------------------------

function compile_mdbook_binaries() {
  #
  # Shell out to another script to create the mdbook binaries, according to which "two" branches
  # We could make this a little bit more loose .. but currently the binaries will be in ${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_1}
  #
  if [[ ! ( -f ${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_1} && -f ${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_2} ) ]]
  then
    echo $blu:: Building the mdbook binaries [${MDBOOK_BRANCH_1},${MDBOOK_BRANCH_2}] $white
    ${G_DIRNAME}/build_mdbook_branches.sh ${MDBOOK_BRANCH_1},${MDBOOK_BRANCH_2} ${G_TMPDIR} ${PWD}/target 2>&1 | indent
  fi

  if [[ ! ( -f ${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_1} && -f ${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_2} ) ]]
  then
    echo $red:: Failed to build the mdbook binaries$white
    exit 1
  else 
    G_BINARY_PATH_1=${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_1}
    G_BINARY_PATH_2=${PWD}/target/mdbook-${G_BINARY_BRANCH_NAME_2}
    echo $blu:: We have both binaries [${G_BINARY_PATH_1}, ${G_BINARY_PATH_2}]$white
  fi
}

compile_mdbook_binaries ${MDBOOK_BRANCH_1} ${MDBOOK_BRANCH_2}
collect_book_repos ${EXTRA_REPOS}
build_books_and_compare
