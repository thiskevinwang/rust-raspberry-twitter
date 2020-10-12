# Start with a rust debian image
FROM arm32v7/rust:1.46-slim-buster AS cargo-build
RUN apt-get upgrade

# if needed, install dependencies here
#RUN apk add libseccomp-dev

# set the workdir and copy the source into it
WORKDIR /app
COPY Cargo.lock /app
COPY Cargo.toml /app
RUN mkdir .cargo
RUN cargo vendor > /app/.cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final Stage
# -----------------

# use a plain image, the version needs to match the builder
FROM arm32v7/debian
RUN apt-get upgrade

# if needed, install dependencies here
#RUN apk add libseccomp

# copy the binary into the final image
COPY --from=cargo-build /app/target/release/my_project .

# set the binary as entrypoint
ENTRYPOINT ["/my_project"]⏎   