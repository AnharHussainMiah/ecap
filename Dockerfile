FROM alpine
COPY ./target/x86_64-unknown-linux-musl/release/ecap /root
CMD ["/root/ecap"]