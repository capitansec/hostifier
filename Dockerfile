FROM rust:1.76-bullseye as builder

WORKDIR /usr/src/hostifier
COPY . .

RUN apt-get update && apt-get install -y libssl-dev
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /usr/src/hostifier

COPY --from=builder /usr/src/hostifier/target/release/hostifier .
COPY --from=builder /usr/src/hostifier/names.txt .

ENTRYPOINT ["./hostifier"]
