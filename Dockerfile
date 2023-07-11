FROM rust as builder

WORKDIR /app
ENV CARGO_REGISTRY=https://github.com/rust-lang/crates.io-index

# 复制项目代码到容器中
COPY ./Cargo.toml .
COPY ./src ./src
# 构建项目
RUN cargo build --release

FROM multiarch/debian-debootstrap:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/yanbing-edge .
RUN mkdir -p /app/conf
COPY ./conf ./conf
EXPOSE 8000
CMD ["./yanbing-edge"]
