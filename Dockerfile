# Syntax to use BuildKit
# syntax=docker/dockerfile:1.2

FROM rust:1 as builder

# Mount the secret as /root/.ssh/id_rsa
RUN mkdir -p /root/.ssh
RUN --mount=type=secret,id=client_files_secret \
  cat /run/secrets/client_files_secret > /root/.ssh/id_rsa && \
  echo "" >> /root/.ssh/id_rsa

# Setup ssh access
RUN chmod 0600 /root/.ssh/id_rsa && \
    touch /root/.ssh/known_hosts && \
    ssh-keyscan github.com >> /root/.ssh/known_hosts

# setup workdir
WORKDIR /usr/src/anotherland
COPY . .

# build
RUN cargo install core_service --path services/core_service
RUN cargo install frontend_server --path services/frontend_server
RUN cargo install login_server --path services/login_server
RUN cargo install realm_manager_service --path services/realm_manager_service
RUN cargo install cluster_server --path services/cluster_server
RUN cargo install world_service --path services/world_service
RUN cargo install kismet-plotter --path tools/kismet-plotter
RUN cargo install seed-realm --path tools/seed-realm

# bundle
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libsqlite3-0 libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/
COPY --from=builder /usr/src/anotherland/conf /etc/anotherland
COPY --from=builder /usr/src/anotherland/content /usr/local/lib/anotherland
ENV CONTENT_PATH /usr/local/lib/anotherland
CMD ["anotherland"]

