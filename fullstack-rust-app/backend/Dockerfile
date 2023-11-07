FROM rust:1.73-slim-bullseye AS build

RUN apt --yes update && apt --yes install git pkg-config curl libssl-dev ca-certificates curl gnupg
RUN cargo install wasm-bindgen-cli sqlx-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/fullstack-rust-app/
COPY . .
RUN cd backend && cargo build --release 


FROM alpine AS release

WORKDIR /app
COPY --from=build /usr/src/fullstack-rust-app/target/release/backend /usr/local/bin/backend

