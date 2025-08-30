#!/usr/bin/env bash
set -euo pipefail

# Go to project root (one level above sbin)
cd "$(dirname "$0")/.."

# Load env vars
ENV_FILE="settings.env"
if [[ -f "$ENV_FILE" ]]; then
  export $(grep -v '^#' "$ENV_FILE" | xargs)
else
  echo "❌ settings.env file not found in project root"
  exit 1
fi

# Ensure DATABASE_URL is set
: "${DATABASE_URL:?DATABASE_URL must be set in settings.env}"

# Extract DB name from DATABASE_URL
DB_NAME=$(echo $DATABASE_URL | awk -F/ '{print $NF}')

# Create database if it doesn't exist
if ! PGPASSWORD=$(echo $DATABASE_URL | awk -F: '{print $3}' | awk -F@ '{print $1}') \
  psql -h localhost -U $(echo $DATABASE_URL | awk -F: '{print $2}' | sed 's#//##') -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    echo "Database $DB_NAME does not exist. Creating..."
    PGPASSWORD=$(echo $DATABASE_URL | awk -F: '{print $3}' | awk -F@ '{print $1}') \
      createdb -h localhost -U $(echo $DATABASE_URL | awk -F: '{print $2}' | sed 's#//##') "$DB_NAME"
else
    echo "Database $DB_NAME already exists."
fi

# Run migrations
sqlx migrate run --database-url "$DATABASE_URL"
echo "✅ Migrations complete."
