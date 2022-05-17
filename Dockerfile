FROM rust:1.60.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

ENTRYPOINT [ "./target/release/newsletter" ]
