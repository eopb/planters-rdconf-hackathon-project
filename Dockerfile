FROM rust:latest AS server-build-env

COPY . .

RUN cargo install cargo-make
RUN cargo make build_release

EXPOSE 8000

CMD cargo make start
