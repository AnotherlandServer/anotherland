FROM rust:1 as builder

# add credentials on build
ARG SSH_PRIVATE_KEY
RUN mkdir /root/.ssh/
RUN echo "${SSH_PRIVATE_KEY}" > /root/.ssh/id_rsa
RUN chmod 600 /root/.ssh/id_rsa

# prepare ssh for github
RUN touch /root/.ssh/known_hosts
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts

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