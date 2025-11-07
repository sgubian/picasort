Picasort in an early project for sorting pictures that are stored on different locations.

The planned features will be:

- Indexing images from a list of sources (S3, folders, Google photos, icloud photos,...)
- Finding duplicates, similar ones
- Search for tags, text in the images, related images with text and objects
- Searching for images containing objects or persons


### PostgreSQL wit GIS and pgvector support, you need to build the corresponding image:
```bash
docker build -t picasortdb \
    --build-arg PICASORT_DB_USER_NAME=$PICASORT_DB_USER_NAME \
    --build-arg PICASORT_DB_USER_PASSWD=$PICASORT_DB_USER_PASSWD \
    --build-arg PICASORT_DB_NAME=$PICASORT_DB_NAME .
```

### PostgreSQL starting the container
```bash
docker compose -f Docker/docker-compose.yml -d up
```

### To connect to the running PostgreSQL
```bash
psql -U $PICASORT_DB_USER_NAME -d ${PICASORT_DB_NAME} -h 0.0.0.0 -p ${PICASORT_DB_HOST_PORT} -v username=$PICASORT_DB_USER_NAME -v userpass=$PICASORT_DB_USER_PASSWD -v dbname=$PICASORT_DB_NAME
```
