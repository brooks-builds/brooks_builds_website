FROM brookzerker/rust:latest
RUN useradd -m -U dev
EXPOSE 3000
VOLUME [ "/code" ]
WORKDIR /code
CMD trunk serve