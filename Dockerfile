FROM fedora:33

RUN mkdir /application
COPY target/release/generate-event /application/generate-event

# Put your transform lib into this path
ENV  TRANSFORM_LIB_PATH=/application/libtransform.so

CMD ["/application/generate-event"]
