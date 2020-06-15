FROM alpine

RUN mkdir /application
COPY target/release/generate-event /application/generate-event
COPY transform-lib/target/release/libtransform_lib.so /application/libtransform_lib.so

ENV  TRANSFORM_LIB_PATH=/application/libtransform_lib.so

CMD ["/application/generate-event"]
