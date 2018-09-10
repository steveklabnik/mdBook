#!/bin/sh

#
# Example:
# sh tests/mdbook_version_validator/build_branches.sh master,smart-preprocessor /tmp/fff `pwd`/target
#
# This will generate two binaries, mdbook_master and mdbook_smart-preprocessor
# We can then use both these binaries, to check the "build" of other books to see if they differ.
#

BRANCHES=$1
WORKING_DIR=$2
OUT_PATH=$3
REPO=${4:-"https://github.com/rust-lang-nursery/mdbook"}

for branch in $(echo $BRANCHES | sed "s/,/ /g")
do
    safe_branch=$(echo $branch | sed "s/[^A-Za-z0-9-]/_/g")
    workdir_for_branch=${WORKING_DIR}_${safe_branch}
    binary_name=mdbook-${safe_branch}

    git clone --depth=1 --branch=${branch} ${REPO} ${workdir_for_branch}
    rm -rf ${workdir_for_branch}/.git
    cd ${workdir_for_branch} && \
    cargo build && \
    mv target/debug/mdbook ${OUT_PATH}/${binary_name} && \
    echo "${safe_branch}=${OUT_PATH}/${binary_name}"
done

