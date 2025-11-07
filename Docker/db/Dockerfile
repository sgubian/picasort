# Copyright (c) 2024 Lemur-Catta.org
# Author: Sylvain Gubian <sgubian@lemur-catta.org>
FROM postgis/postgis:latest

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    postgresql-18-pgvector \
    && rm -rf /var/lib/apt/lists/*

# Optional: Set environment variables
ENV POSTGRES_USER=$PICASORT_DB_USER_NAME \
    POSTGRES_PASSWORD=$PICASORT_DB_USER_PASSWD \
    POSTGRES_DB=$PICASORT_DB_NAME

# Copy any custom SQL scripts (optional)
# COPY ./initdb/ /docker-entrypoint-initdb.d/

# Expose default PostgreSQL port
EXPOSE $PICASORT_DB_DOCKER_PORT
