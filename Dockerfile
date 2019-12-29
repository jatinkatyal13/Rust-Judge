FROM ubuntu:16.04

RUN apt-get update

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY ./ ./
RUN cargo build --release
RUN cp target/release/worker /bin/worker

RUN gcc runguard/runguard.c -o /bin/runguard

ENTRYPOINT ["/bin/worker"]
