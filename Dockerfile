FROM rust:1.77.0

# Switch working directory to app. Created if not exists
WORKDIR /app

# Install dependencies for linking.
RUN apt update && apt install lld clang -y

# Copy from workdir to Docker Img
COPY . .
# Make sure that SQLX does not attempt to contact DB during compliation.
ENV SQLX_OFFLINE true

RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod2"]
