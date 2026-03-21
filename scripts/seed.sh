#!/usr/bin/env bash
# WashCo Seed Script
# Usage: ./scripts/seed.sh [--reset]
#
# --reset: Drop and recreate database before seeding

set -euo pipefail

DB_URL="${DATABASE_URL:-postgres://washco:washco@localhost:5432/washco}"
DB_NAME="${DB_URL##*/}"

if [[ "${1:-}" == "--reset" ]]; then
    echo "Resetting database..."
    psql "${DB_URL%/*}/postgres" -c "DROP DATABASE IF EXISTS ${DB_NAME};"
    psql "${DB_URL%/*}/postgres" -c "CREATE DATABASE ${DB_NAME};"
    echo "Database recreated."
fi

echo "Running migrations..."
export DATABASE_URL="$DB_URL"
sqlx migrate run --source migrations/

echo "Seeding data..."
psql "$DB_URL" -f scripts/seed.sql

echo "Done! Database is ready."
