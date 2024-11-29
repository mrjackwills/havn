#############
## Builder ##
#############

FROM --platform=$BUILDPLATFORM rust:slim AS builder

ARG TARGETARCH

# These are build platform depandant, but will be ignored if not needed
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER="aarch64-linux-gnu-gcc"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-lgcc"
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER="arm-linux-gnueabihf-ld"

COPY ./containerised/target.sh .

RUN chmod +x ./target.sh && ./target.sh

RUN apt-get update && apt-get install $(cat /.compiler) -y

WORKDIR /usr/src

# Create blank project
RUN cargo new havn

# We want dependencies cached, so copy those first
COPY Cargo.* /usr/src/havn/

# Set the working directory
WORKDIR /usr/src/havn

# Install target platform (Cross-Compilation)
RUN rustup target add $(cat /.target)

# This is a dummy build to get the dependencies cached - probably not needed - as run via a github action
RUN cargo build --target $(cat /.target) --release

# Now copy in the rest of the sources
COPY src /usr/src/havn/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/havn/src/main.rs

# This is the actual application build
RUN cargo build --release --target $(cat /.target)

RUN cp /usr/src/havn/target/$(cat /.target)/release/havn /

#############
## Runtime ##
#############

FROM scratch AS runtime

# Set an ENV that we're running in a container
ENV HAVN_RUNTIME=container

# Copy application binary from builder image
COPY --from=BUILDER /havn /app/

# Run the application
ENTRYPOINT [ "/app/havn"]
