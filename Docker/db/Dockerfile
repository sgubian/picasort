# Copyright (c) 2024 Lemur-Catta.org
# Author: Sylvain Gubian <sgubian@lemur-catta.org>
FROM postgis/postgis:latest

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    postgresql-17-pgvector \
    && rm -rf /var/lib/apt/lists/*


# Copy any custom SQL scripts (optional)
COPY ./init-db.sh /docker-entrypoint-initdb.d/

# Expose default PostgreSQL port
EXPOSE 5432
