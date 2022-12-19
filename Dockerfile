FROM ubuntu:latest
WORKDIR /
ENV data=./api-config.json
COPY ./target/debug/mockoon-rust /usr/local/bin/mockoon-rust
ARG data="api-config.json"
CMD ["mockoon-rust", "--data", "$data"]