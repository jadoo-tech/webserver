FROM ubuntu:24.04

RUN apt update && apt install -y libc6 libc6-dev

COPY ./webserver .

ENTRYPOINT ["./webserver"]

