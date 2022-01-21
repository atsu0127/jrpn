FROM rust:1.58.1 AS builder

# 作業ディレクトリ
WORKDIR /todo
COPY Cargo.toml Cargo.toml

# まずはダミーを使って依存関係持ってくる
RUN mkdir -p src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# そのあと本番持ってくる
Copy ./src ./src/
Copy ./templates ./templates/

# ダミーのデータは消しておく
RUN rm -f target/release/deps/todo*

# 本物のビルド
RUN cargo build --release

# 実行用のイメージ
FROM debian:stable

# バイナリ持ってくる
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo

# アプリの実行
CMD ["todo"]