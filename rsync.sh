#!/bin/sh
rsync -avz \
        --exclude 'rsync.sh' \
        --exclude 'ci' \
        --exclude '.git' \
        --exclude '.git*' \
        --exclude '.github' \
        --exclude '*.swp' \
        --exclude 'target' \
        --exclude 'book-example' \
        --exclude 'examples' \
        --exclude 'tests' \
        . \
        yggdrasil:/root/data/etatismus/mdbook/
