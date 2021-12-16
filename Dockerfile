# Build configuration
ARG project_name=kagi
# Fill in name of crate^ here

# Set up rust build environment
FROM rust:latest as builder
ARG project_name

# Create layer for the dependencies, so we don't have to rebuild them later
WORKDIR /usr/src
RUN USER=root cargo new $project_name
WORKDIR /usr/src/$project_name
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Build the actual source
COPY src ./src
RUN touch ./src/main.rs && cargo build --release

# second stage.
FROM gcr.io/distroless/cc-debian11
ARG project_name
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/src/$project_name/target/release/$project_name ./app
USER 1000
EXPOSE 3000
CMD ["/app"]