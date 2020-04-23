FROM rust:1-buster

ARG key
ARG iv
ENV key=$key
ENV iv=$iv

ENV APP_HOME /app
WORKDIR $APP_HOME
COPY . .

RUN apt-get update -y && apt-get upgrade -y

# Decrypt secrets
WORKDIR $APP_HOME
RUN openssl aes-256-cbc -K $key -iv $iv -in secrets.tar.gz.enc -out secrets.tar.gz -d && tar xvf secrets.tar.gz

RUN cargo build 
RUN cargo test

RUN ls -la secrets/
