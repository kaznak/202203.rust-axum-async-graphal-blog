FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM rust:slim
COPY --from=builder /usr/local/cargo/bin/rust_axum_async_graphql_blog /usr/local/bin/rust_axum_async_graphql_blog
WORKDIR /app
CMD ["rust_axum_async_graphql_blog"]
