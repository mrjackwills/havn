#############
## Runtime ##
#############
FROM ubuntu
# Set env that we're running in a container
ENV HAVN_RUNTIME=container

# Copy application binary from builder image
COPY ./target/x86_64-unknown-linux-musl/release/havn /app/

# Run the application
ENTRYPOINT [ "/app/havn"]

# Dev build for testing
# docker build -t havn_dev -f containerised/dev.Dockerfile . && docker run --rm -it havn_dev -p10000 www.mealpedant.com

# Dev build one liner, x86 host
# docker image prune -a; cargo build --release --target x86_64-unknown-linux-musl && docker build -t havn_dev -f containerised/dev.Dockerfile . && docker run --rm --network=host -it havn_dev

## One liner to build musl program, build docker image, then execute the image
# cargo build --release --target x86_64-unknown-linux-musl && docker build -t havn_dev -f containerised/Dockerfile . && docker run --rm -it havn_dev

# Build production version
# docker build --platform linux/arm/v6 --platform linux/arm64 --platform linux/amd64 -t havn_dev -f containerised/Dockerfile . && docker run --rm -it havn_dev

# Buildx command to build musl version for all three platforms, should probably be executed in create_release
# docker buildx create --use
# docker buildx build --platform linux/arm/v6,linux/arm64,linux/amd64 -t havn_dev_all -o type=tar,dest=/tmp/havn_dev_all.tar -f containerised/Dockerfile .
