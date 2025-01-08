With docker-compose.yml setup and docker running, you can start the database with:

```sh
docker-compose up
```

Status and logs with
```sh
docker-compose ps
docker logs rusty_shorts_db
```

You can go into the databse with 
```sh
docker exec -it rusty-shorts-db psql -U user -d rusty_shorts_db
```