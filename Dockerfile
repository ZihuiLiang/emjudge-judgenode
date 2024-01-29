FROM ubuntu:22.04

WORKDIR /usr/src/emjudge-judgenode

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    curl

RUN g++ --version
RUN gcc --version

RUN apt-get install -y --no-install-recommends \
    python3-minimal \
    python2-minimal \
    pypy3 \
    pypy

RUN python3 --version
RUN pypy3 --version
RUN python2 --version
RUN pypy --version

RUN apt-get install -y --no-install-recommends default-jdk

RUN java -version

RUN apt-get install -y --no-install-recommends ruby
RUN ruby --version

RUN apt-get install -y --no-install-recommends perl

RUN perl -v

RUN apt-get install -y --no-install-recommends mono-mcs

RUN mcs --version

RUN apt-get install -y --no-install-recommends golang-go

RUN go version

RUN apt-get install -y --no-install-recommends nodejs
    
RUN node --version

RUN apt-get install -y --no-install-recommends gfortran

RUN gfortran --version

RUN apt-get install -y --no-install-recommends lua5.3

RUN lua -v

RUN apt-get install -y --no-install-recommends php

RUN php --version

RUN apt-get install -y --no-install-recommends gnu-smalltalk

RUN gst --version

RUN apt-get install -y --no-install-recommends ocaml

RUN ocaml -version

RUN apt-get install -y --no-install-recommends gnucobol

RUN cobc --version

RUN apt-get install -y --no-install-recommends gnat

RUN gnat --version

RUN apt-get install -y --no-install-recommends sbcl

RUN sbcl --version

RUN apt-get install -y --no-install-recommends scala

RUN scala -version

RUN apt-get install -y --no-install-recommends tcl

RUN echo 'puts [info patchlevel]' | tclsh

RUN apt-get install -y --no-install-recommends octave

RUN octave --version

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version

RUN cargo install juliaup

RUN julia --version

RUN apt-get install -y --no-install-recommends libcgroup-dev pkg-config libsqlite3-dev

RUN apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY config config
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

CMD ["target/release/emjudge-judgenode"]