## Docker Postgres database helpers
With docker-compose.yml setup and docker running, you can start the database with:

`-d` is for detached, running the container in the backgorund
```sh
docker-compose up -d
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

to go inside the container
```sh
docker exec -it rusty-shorts-db bash
```

## Diesel
Install diesel

```sh
echo DATABASE_URL=postgres://admin:password@localhost:5432/rusty_shorts > .env
```

Diesel setup will use the database in `.env`, which should also be in `.gitignore`
```sh
diesel setup
```

### Create a migration
```sh
diesel migration generate create_urls_table
```

After filling `up.sql` and `down.sql` in folder /migration/<datetime>_create_urls_table/ test migration.
```sh
diesel migration run
```

Ensure migration works (this should fail for example if down.sql is empty)
```sh
diesel migration redo
```