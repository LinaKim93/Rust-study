FROM rust:1.62.0

LABEL email="release93@gmail.com"
LABEL name="linakim"
LABEL version="0.0.1"

RUN apt-get update \
    && apt-get upgrade -y

COPY * /home/linakim/rust-study/
ENV DIR=/home/linakim/rust-study/

WORKDIR /home/linakim/rust-study/