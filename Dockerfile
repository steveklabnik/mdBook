FROM alpine:3.12.0 AS builder

ARG MDBOOK_VERSION=0.3.7

RUN apk add --no-cache cargo && \
    wget -O /tmp/mdbook.tar.gz "https://github.com/rust-lang/mdBook/archive/v$MDBOOK_VERSION.tar.gz" && \
    cd /tmp && tar -xzvf mdbook.tar.gz && \
    cd /tmp/mdBook* && cargo build --release && \
    mv /tmp/mdBook*/target/release/mdbook /tmp/mdbook

########## ########## ##########

FROM alpine:3.12.0

COPY --from=builder /tmp/mdbook /usr/local/bin/mdbook
COPY docker-entrypoint.sh /docker-entrypoint.sh

RUN apk add --no-cache libgcc && \
    adduser -D mdbook && \
    chmod +x /docker-entrypoint.sh

USER mdbook
VOLUME ["/book"]
WORKDIR /book
ENTRYPOINT ["/docker-entrypoint.sh"]
