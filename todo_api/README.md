# todo_api
これはtodoアプリのAPI部分です

## 利用コンポーネント
- MySQL 8.0.28
- diesel 1.4.1

## 初期セットアップ
まずmysqlのコンテナを作ります
```bash
## 立ち上げ
cd mysql
docker-compose up -d

## 一応確認
mysql -h 127.0.0.1 -P 3306 -u appuser -p
```

その後dieselからの接続を確認します
```bash
cd ..

## 初期動作
diesel migration run

## 諸操作やっとく
### INSERT
cargo run --bin insert_todo
cargo run --bin show_todo

### UPDATE
cargo run --bin update_todo
cargo run --bin show_todo

### DELETE
cargo run --bin delete_todo
cargo run --bin show_todo
```