FROM rust:latest
WORKDIR /app
COPY . .

RUN rustup default
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["cargo", "watch", "-x", "run", "-w", "src"]