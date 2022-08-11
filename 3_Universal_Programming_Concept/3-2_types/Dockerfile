FROM rust:1.62.0

LABEL email="release93@gmail.com"
LABEL name="linakim"
LABEL version="0.0.1"

RUN apt-get update \
    && apt-get upgrade -y

COPY * /home/linakim/rust-study/3_Universal_Programming_Concept/3-2_types/
ENV DIR=/home/linakim/rust-study/3_Universal_Programming_Concept/3-2_types/

WORKDIR /home/linakim/rust-study/3_Universal_Programming_Concept/3-2_types/

RUN cargo new --bin types