#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_USER=${POSTGRES_USER:=zero2prod}
DB_PASSWORD="${POSTGRES_PASSWORD:=zero2prod_password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]; then
  (cd docker && docker compose up -d)
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

echo >&2 "Postgres has been migrated and ready to go!"
