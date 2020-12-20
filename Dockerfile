FROM rust

COPY . rust-figlet

CMD ["./rust-figlet", "It worked."]