# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/
FROM rust AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Build the project
WORKDIR /usr/src/hello-world
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/hello-world .
CMD ["./hello-world"]