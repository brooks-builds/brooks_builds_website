FROM rust:latest
RUN useradd -m -U dev
EXPOSE 3000
VOLUME [ "/code" ]
USER dev
RUN curl https://sh.rustup.rs -sSf -y | sh
RUN cargo install cargo-watch
WORKDIR /code
CMD cargo watch -x run