FROM ubuntu:20.04 as base

WORKDIR /usr/src/emjudge-judgenode

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libseccomp-dev \
    libseccomp2 \
    libssl-dev \
    snapd \
    curl \
    make \
    pkg-config \
    python3 \
    python2 \
    g++ \
    gcc \ 
    default-jdk \
    pypy3 \
    ruby \
    perl \
    mono-complete \
    golang \
    nodejs \
    npm \
    gfortran \
    lua \
    php \
    gnu-smalltalk \
    ocaml \
    open-cobol \
    gnat \
    sbcl \
    scala \
    tcl \
    tcllib \
    octave \

    && rm -rf /var/lib/apt/lists/*

RUN ln -s /usr/bin/nodejs /usr/bin/node

RUN snap install kotlin --classic

RUN snap install --classic swift

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN curl -fsSL https://install.julialang.org | sh -s -- -y

ENV PATH="${HOME}/.cargo/bin:${PATH}"

ENV PATH="${HOME}/.juliaup/bin:${PATH}"

RUN g++ --version
RUN gcc --version
RUN java -version
RUN python3 --version
RUN pypy3 --version
RUN python2 --version
RUN ruby --version
RUN perl -v
RUN mono --version
RUN swift --version
RUN go version
RUN node --version
RUN rustc --version
RUN kotlin -version
RUN julia --version
RUN gfortran --version
RUN lua -v
RUN php --version
RUN gst --version
RUN ocaml -version
RUN cobc --version
RUN gnat --version
RUN sbcl --version
RUN scala -version
RUN echo 'puts [info patchlevel]' | tclsh
RUN octave --version
RUN pypy --version





COPY . .

RUN cargo run --release