FROM node:14
RUN npm install -g cypress
RUN useradd -m -U dev
VOLUME [ "/code" ]
USER dev
WORKDIR /code
CMD npm test