FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
ENV SQLX_OFFLINE=true
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/serigen ./app

FROM scratch AS runtime
ENV SERIGEN_HOST=0.0.0.0
ENV SERIGEN_PORT=80
ENV DATABASE_PATH=./data/numbers.sqlite
ENV SERIGEN_JWT_SECRET=S5zzHDP71TvNvPFAplSgycOIaBYdrMGT3O8mAOpzGeI=
WORKDIR /usr/local/bin
COPY --from=builder /app/app .
COPY --from=builder /app/assets ./assets/
ENTRYPOINT ["app"]