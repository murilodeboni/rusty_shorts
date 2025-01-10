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
psql -h localhost -p 5432 -U admin -d rusty_shorts_db
```

query and check urls exist once a table is setup
```sh
SELECT * FROM urls LIMIT 10;
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

## Rocket

Try to shorten an url
```sh
curl -X POST http://127.0.0.1:8000/api/shorten \
     -H "Content-Type: application/json" \
     -d '{"url": "https://google.com/"}'
```

you should get a response in the format
```json
{"slug":"abc123","shortened_url":"http://localhost:8000/abc123"}
```

and get redirected to the original by going to http://localhost:8000/abc123

To delete a slug
```sh
curl -X DELETE http://127.0.0.1:8000/api/delete/abc123
```

You should receive
```json
{"message":"Slug 'abc123' deleted successfully"}
```