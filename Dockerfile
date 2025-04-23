FROM docker.io/rust:bookworm as rust-builder
COPY ./Cargo* /opt/
COPY ./src /opt/src
WORKDIR /opt
RUN cargo install --path . --root .

FROM docker.io/debian:bookworm-slim
WORKDIR /opt
ENTRYPOINT ["/bin/pdu-mib-gen-rs"]

COPY --from=rust-builder /opt/bin/pdu-mib-gen-rs /bin/pdu-mib-gen-rs
