FROM rust
WORKDIR /usr/src/tipb-build
RUN apt update && apt install unzip golang cmake -y
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip
RUN unzip protoc-3.5.1-linux-x86_64.zip
ENV PATH="/usr/src/tipb-build/bin:/root/go/bin:${PATH}"
WORKDIR /tipb
ENTRYPOINT ["/usr/bin/make"]
CMD ["all"]
