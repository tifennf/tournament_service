FROM rust:1.57 as build

RUN USER=root cargo new --bin tournament_service
WORKDIR /tournament_service

COPY ./Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs 

ADD . ./



RUN rm ./target/release/deps/tournament_service*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
	&& apt-get install -y ca-certificates tzdata \
	&& rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
	APP_USER=appuser

RUN groupadd $APP_USER \
	&& useradd -g $APP_USER $APP_USER \
	&& mkdir -p ${APP}

COPY --from=builder /tournament_service/target/release/tournament_service ${APP}/tournament_service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}


CMD [ "./tournament_service" ]