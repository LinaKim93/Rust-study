FROM rust:1.66.0

LABEL email="release93@gmail.com"
LABEL name="linakim"
LABEL version="0.0.1"

RUN apt-get update \
    && apt-get upgrade -y

COPY * /home/linakim/rust-study/3_Universal_Programming_Concept/3-3_function/
ENV DIR=/home/linakim/rust-study/3_Universal_Programming_Concept/3-3_function/

WORKDIR /home/linakim/rust-study/3_Universal_Programming_Concept/3-3_function/

RUN cargo new --bin function