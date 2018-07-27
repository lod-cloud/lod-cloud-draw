FROM ubuntu:18.04
MAINTAINER john@mccr.ae

ADD target/release/lod-cloud-draw /usr/bin/lod-cloud-draw
RUN apt-get update
RUN apt-get install -y gfortran libgfortran3
ADD libcgfam.so /usr/lib/libcgfam.so
ADD liblbfgs.so /usr/lib/liblbfgs.so

ENTRYPOINT /usr/bin/lod-cloud-draw
