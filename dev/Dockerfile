FROM rust:latest
RUN useradd -m -U dev
EXPOSE 3000
VOLUME [ "/code" ]
USER dev
RUN curl https://sh.rustup.rs -sSf -y | sh
RUN cargo install cargo-watch
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install cargo-edit
WORKDIR /code
CMD trunk serve