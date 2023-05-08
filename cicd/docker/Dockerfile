# currently on 1.69.0
FROM rust:latest

WORKDIR /path/to/myapp

#create new empty project
RUN cargo init

COPY ./.cargo .cargo
COPY ./vendor vendor
COPY Cargo.toml Cargo.lock ./

RUN cargo install --path .

CMD ["myapp"]