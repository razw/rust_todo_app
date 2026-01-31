# Rust Todo App

Rust (Axum + SQLite) のAPIと Next.js のフロントエンドで構成されたTodoアプリです。
Dockerのみで同じ手順で起動できます。

## Requirements

- Docker / Docker Compose

## Setup

```bash
cp .env.example .env
```

必要に応じて `.env` を編集してください。

## Run

```bash
docker compose up --build
```

起動後のアクセス先:

- Frontend: http://localhost:3001
- API: http://localhost:3000

## Stop

```bash
docker compose down
```

データも削除したい場合:

```bash
docker compose down -v
```

## Notes

- SQLiteのデータはDockerボリューム `api-data` に保存されます。
- フロントのSSRはコンテナ内から `http://api:3000` へ接続します。
