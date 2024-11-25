FROM ubuntu:22.04
COPY ./target/release/versions-control-ui /target/release/versions-control-ui
COPY ./dist /target/release/dist
RUN chmod +x /target/release/versions-control-ui
WORKDIR /target/release/
ENTRYPOINT ["./versions-control-ui" ]