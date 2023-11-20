# Syntax to use BuildKit
# syntax=docker/dockerfile:1.2

FROM rust:1 as builder


# Mount the secret as /root/.ssh/id_rsa
RUN --mount=type=secret,id=private_build_repo_key,dst=/root/.ssh/id_rsa

# Setup ssh access
RUN chmod 600 /root/.ssh/id_rsa && \
    mkdir -p /root/.ssh && \
    touch /root/.ssh/known_hosts && \
    ssh-keyscan github.com >> /root/.ssh/known_hosts

# setup workdir
WORKDIR /usr/src/anotherland
COPY . .

# clone private files required for build
RUN git clone git@github.com:AnotherlandServer/private-build-files.git

# build
ENV OTHERLAND_CLIENT_PATH /usr/src/anotherland/private-build-files/client_files/
RUN cargo install --path .

# bundle
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/anotherland /usr/local/bin/anotherland
CMD ["anotherland"]
