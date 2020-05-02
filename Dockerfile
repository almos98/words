FROM rustlang/rust:nightly-alpine3.10

RUN apk add build-base

WORKDIR /app
COPY ./ /app

RUN cargo build --release

FROM alpine:3.10

WORKDIR /app
COPY --from=0 /app/target/release/words .
COPY ./Rocket.toml .
COPY ./static ./static
COPY ./www ./www

EXPOSE 5000

ENTRYPOINT ["/words"]