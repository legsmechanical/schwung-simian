# SIMIAN Module Build Environment
# Targets: Ableton Move (aarch64 Linux)
#
# No Faust here on purpose: the DSP is generated on a workstation by
# scripts/gen_dsp.sh and committed under src/dsp/generated/, so the container
# only needs a cross compiler.

FROM debian:bookworm

RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    make \
    file \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

ENV CROSS_PREFIX=aarch64-linux-gnu-
ENV CC=aarch64-linux-gnu-gcc
ENV CXX=aarch64-linux-gnu-g++
