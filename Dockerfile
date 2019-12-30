FROM ubuntu:16.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update

RUN apt-get install -y gcc curl libssl-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cp $HOME/.cargo/bin/* /bin/

COPY ./runguard/runguard.c ./runguard.c
COPY ./runguard/runguard-config.h ./runguard-config.h
RUN gcc -o /bin/runguard runguard.c

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build
RUN cp target/debug/worker /bin/worker

COPY ./languages ./languages
RUN mkdir /tmp/run

RUN useradd -ms /bin/bash domjudge
ENTRYPOINT ["/bin/worker"]
