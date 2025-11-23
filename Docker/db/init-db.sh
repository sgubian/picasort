#!/bin/bash
set -e

# Read the password from the Docker secret
DB_PASSWORD=$(cat /run/secrets/db_password)
DB_USER="picasort"
DB_NAME="picasort"

echo ">>> Creating database and user..."
psql -v ON_ERROR_STOP=1 --username "postgres" << EOSQL
DO \$\$
    BEGIN
        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '$DB_USER') THEN
            CREATE ROLE $DB_USER WITH LOGIN PASSWORD '$DB_PASSWORD';
        END IF;
    END
    \$\$;
CREATE DATABASE "$DB_NAME" OWNER "$DB_USER";
GRANT ALL PRIVILEGES ON DATABASE "$DB_NAME" TO "$DB_USER";
EOSQL

echo ">>> Enabling extensions on $DB_NAME..."
psql -v ON_ERROR_STOP=1 --username "postgres" --dbname "$DB_NAME" << EOSQL
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS vector;
EOSQL

echo ">>> PostgreSQL setup complete!"
