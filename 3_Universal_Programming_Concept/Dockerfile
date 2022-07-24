FROM rust:1.62.0

LABEL email="release93@gmail.com"
LABEL name="linakim"
LABEL version="0.0.1"

RUN apt-get update \
    && apt-get upgrade -y

COPY * /home/linakim/rust-study/3_Universal_Programming_Concept/
ENV DIR=/home/linakim/rust-study/3_Universal_Programming_Concept/

WORKDIR /home/linakim/rust-study/3_Universal_Programming_Concept/

RUN cargo new --bin variables