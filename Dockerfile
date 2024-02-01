FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y build-essential autoconf automake libtool wget byacc flex libpam-dev libsystemd-dev && \
    rm -rf /var/lib/apt/lists/*


WORKDIR /tmp
RUN wget https://github.com/libcgroup/libcgroup/releases/download/v3.1.0/libcgroup-3.1.0.tar.gz && \
    tar -xzvf libcgroup-3.1.0.tar.gz && \
    rm libcgroup-3.1.0.tar.gz

WORKDIR /tmp/libcgroup-3.1.0
RUN ./configure && \
    make && \
    make install

WORKDIR /
RUN rm -rf /tmp/libcgroup-3.1.0

RUN ldconfig


RUN apt-get remove -y autoconf automake libtool  byacc flex libpam-dev libsystemd-dev && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN apt update && apt install -y --no-install-recommends \
    libssl-dev \
    curl
    

RUN g++ --version
RUN gcc --version

RUN apt install -y --no-install-recommends \
    python3-minimal \
    python2-minimal \
    pypy3 \
    pypy

RUN python3 --version
RUN pypy3 --version
RUN python2 --version
RUN pypy --version

RUN apt install -y --no-install-recommends default-jdk

RUN java -version

RUN apt install -y --no-install-recommends ruby
RUN ruby --version

RUN apt install -y --no-install-recommends perl

RUN perl -v

RUN apt install -y --no-install-recommends mono-mcs

RUN mcs --version

RUN apt install -y --no-install-recommends golang-go

RUN go version

RUN apt install -y --no-install-recommends nodejs
    
RUN node --version

RUN apt install -y --no-install-recommends gfortran

RUN gfortran --version

RUN apt install -y --no-install-recommends lua5.3

RUN lua -v

RUN apt install -y --no-install-recommends php

RUN php --version

RUN apt install -y --no-install-recommends gnu-smalltalk

RUN gst --version

RUN apt install -y --no-install-recommends ocaml

RUN ocaml -version

RUN apt install -y --no-install-recommends gnucobol

RUN cobc --version

RUN apt install -y --no-install-recommends gnat

RUN gnat --version

RUN apt install -y --no-install-recommends sbcl

RUN sbcl --version

RUN apt install -y --no-install-recommends scala

RUN scala -version

RUN apt install -y --no-install-recommends tcl

RUN echo 'puts [info patchlevel]' | tclsh

RUN apt install -y --no-install-recommends octave

RUN octave --version

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version

RUN apt install -y --no-install-recommends julia

RUN julia --version

RUN apt install -y --no-install-recommends pkg-config libsqlite3-dev

RUN apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /usr/src/emjudge-judgenode

COPY config config
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release --features "full_v2"

CMD ["bash"]

CMD ["target/release/emjudge-judgenode"]