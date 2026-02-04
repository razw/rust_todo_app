#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
COMPOSE_FILE="$ROOT_DIR/docker-compose.e2e.yml"
ENV_FILE="$ROOT_DIR/.env.e2e"
PROJECT_NAME="rust-todo-e2e"

API_URL="http://localhost:3100"
E2E_BASE_URL="http://localhost:3101"

cleanup() {
  docker compose -p "$PROJECT_NAME" -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down
}
trap cleanup EXIT

cd "$ROOT_DIR"

# Ensure local env file exists for docker compose
if [ ! -f "$ENV_FILE" ]; then
  cp .env.example "$ENV_FILE"
fi

docker compose -p "$PROJECT_NAME" -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d --build

# Wait for API
for _ in {1..30}; do
  if curl -fsS "$API_URL/todos" >/dev/null; then
    break
  fi
  sleep 2
done

# Wait for Frontend
for _ in {1..30}; do
  if curl -fsS "$E2E_BASE_URL" >/dev/null; then
    break
  fi
  sleep 2
done

cd "$ROOT_DIR/frontend"

if [ ! -d "node_modules" ]; then
  npm ci
fi

API_URL="$API_URL" E2E_BASE_URL="$E2E_BASE_URL" npm run test:e2e
