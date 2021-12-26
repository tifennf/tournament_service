FROM rust:1.57 as build

RUN USER=root cargo new --bin tournament_service
WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs 

COPY ./src ./src

RUN rm ./target/release/deps/tournament_service*
RUN cargo build --release

FROM rust:1.57

COPY --from=build /app/target/release/tournament_service .


CMD [ "./tournament_service" ]