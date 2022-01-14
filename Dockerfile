FROM brookzerker/rust:latest
EXPOSE 3000
VOLUME [ "/code" ]
WORKDIR /code
CMD trunk serve