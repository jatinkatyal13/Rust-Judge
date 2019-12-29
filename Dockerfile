FROM ubuntu:16.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update

RUN apt-get install -y gcc

COPY ./runguard/runguard.c ./runguard.c
COPY ./runguard/runguard-config.h ./runguard-config.h
RUN gcc -o /bin/runguard runguard.c

RUN apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cp $HOME/.cargo/bin/* /bin/
RUN apt-get install -y libssl-dev pkg-config
RUN cargo build --release
RUN cp target/release/worker /bin/worker

COPY ./languages ./languages

ENTRYPOINT ["/bin/worker"]
