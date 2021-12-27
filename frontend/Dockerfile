FROM rust:latest
VOLUME [ "/code" ]
WORKDIR /code
RUN cargo install cargo-watch
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
CMD [ "trunk", "serve" ]
EXPOSE 8080
